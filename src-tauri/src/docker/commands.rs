use std::collections::HashMap;

use serde::Deserialize;

use crate::docker::types::DockerContainerCommand;

use super::cli::Docker;
use super::types::{
    DockerClientVersion, DockerComposeProject, DockerContainer, DockerContainerComposeInfo,
    DockerContainerPlatform, DockerContainerResult, DockerContainerState, DockerNetworkBinding,
    DockerServerVersion, DockerStatus,
};

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct DockerVersionRaw {
    client: DockerClientVersionRaw,
    server: Option<DockerServerVersionRaw>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct DockerPlatformRaw {
    name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct DockerClientVersionRaw {
    version: String,
    api_version: String,
    os: String,
    arch: String,
    context: String,
    platform: DockerPlatformRaw,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct DockerServerVersionRaw {
    version: String,
    api_version: String,
    os: String,
    arch: String,
    platform: DockerPlatformRaw,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct DockerContainerRaw {
    #[serde(rename = "ID")]
    id: String,
    names: String,
    image: String,
    state: String,
    status: String,
    labels: String,
    ports: String,
    platform: DockerContainerPlatformRaw,
}

#[derive(Deserialize)]
struct DockerContainerPlatformRaw {
    pub architecture: String,
    pub os: String,
    pub variant: Option<String>,
}

#[derive(Default)]
struct DockerComposeCollector {
    /// Total container count.
    containers: usize,
    /// Running container state count.
    running: usize,
    /// Unknown container state count.
    unknown: usize,
}

pub fn get_status() -> Result<DockerStatus, String> {
    let raw: DockerVersionRaw = Docker::cmd("version")
        .json()
        .run_json_safe()?
        .ok_or_else(|| "Could not retrieve Docker version".to_string())?;

    let client = raw.client;
    Ok(DockerStatus {
        client: DockerClientVersion {
            name: client.platform.name,
            version: client.version,
            api_version: client.api_version,
            os: client.os,
            arch: client.arch,
            context: client.context,
        },
        server: raw.server.map(|server| DockerServerVersion {
            name: server.platform.name,
            version: server.version,
            api_version: server.api_version,
            os: server.os,
            arch: server.arch,
        }),
    })
}

pub fn get_containers() -> Result<DockerContainerResult, String> {
    let raw: Vec<DockerContainerRaw> = Docker::cmd("ps").arg("-a").json().run_ndjson()?;
    let mut containers: Vec<DockerContainer> = raw.into_iter().map(Into::into).collect();
    containers.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    let mut projects = build_compose_projects(&containers);
    projects.sort_by(|a, b| a.project.to_lowercase().cmp(&b.project.to_lowercase()));
    Ok(DockerContainerResult {
        containers,
        projects,
    })
}

pub fn run_container_command(command: &DockerContainerCommand, id: &str) -> Result<(), String> {
    let cmd = match command {
        DockerContainerCommand::Start => Docker::cmd("start").arg(id),
        DockerContainerCommand::Stop => Docker::cmd("stop").arg(id),
        DockerContainerCommand::Restart => Docker::cmd("restart").arg(id),
        DockerContainerCommand::Pause => Docker::cmd("pause").arg(id),
        DockerContainerCommand::Remove => Docker::cmd("rm").arg(id),
        DockerContainerCommand::Unknown => return Err("Unknown command".to_string()),
    };

    log::info!(
        "Running Docker command... — Command: {:?}, Container: {}",
        command,
        id
    );

    cmd.run()?;
    Ok(())
}

fn build_compose_projects(containers: &[DockerContainer]) -> Vec<DockerComposeProject> {
    // Iterate over the containers once...
    let mut collectors: HashMap<String, DockerComposeCollector> = HashMap::new();
    for c in containers {
        if let Some(compose) = &c.compose {
            let collector = collectors.entry(compose.project.clone()).or_default();
            collector.containers += 1;
            match c.state {
                DockerContainerState::Running | DockerContainerState::Restarting => {
                    collector.running += 1
                }
                DockerContainerState::Unknown => collector.unknown += 1,
                _ => {}
            }
        }
    }

    // ...and then build the project list based on the collected stats.
    let projects: Vec<DockerComposeProject> = collectors
        .into_iter()
        .map(|(project, collector)| {
            let state = if collector.running == collector.containers {
                DockerContainerState::Running
            } else if collector.running > 0 {
                DockerContainerState::Partial
            } else if collector.unknown == collector.containers {
                DockerContainerState::Unknown
            } else {
                DockerContainerState::Exited
            };

            DockerComposeProject { project, state }
        })
        .collect();

    projects
}

fn parse_state(s: &str) -> DockerContainerState {
    serde_json::from_value(serde_json::Value::String(s.to_string())).unwrap_or_default()
}

fn parse_labels(s: &str) -> HashMap<String, Option<String>> {
    if s.is_empty() {
        return HashMap::new();
    }

    s.split(',')
        .filter_map(|pair| {
            let mut parts = pair.splitn(2, '=');
            let key = parts.next()?.to_string();
            let value = parts
                .next()
                .map(|v| v.to_string())
                .filter(|v| !v.is_empty());
            Some((key, value))
        })
        .collect()
}

fn extract_compose_labels(
    labels: &HashMap<String, Option<String>>,
) -> Option<DockerContainerComposeInfo> {
    let project = labels.get("com.docker.compose.project")?.clone()?;
    let service = labels.get("com.docker.compose.service")?.clone()?;
    Some(DockerContainerComposeInfo { project, service })
}

/// Parses the `Status` string — format: `"{status} ({status_detail})"`.
fn split_status(s: &str) -> (String, Option<String>) {
    match s.split_once(' ') {
        Some((head, tail)) => (head.to_string(), Some(tail.to_string())),
        None => (s.to_string(), None),
    }
}

/// Parses `Ports` string — e.g. `0.0.0.0:15672->15672/tcp, 4369/tcp`.
fn parse_ports(s: &str, only_bound: bool) -> Vec<DockerNetworkBinding> {
    s.split(',')
        .map(str::trim)
        .filter(|p| !p.is_empty())
        .filter_map(|entry| {
            let (host_port, container_side) = match entry.split_once("->") {
                Some((host_side, rest)) => {
                    let port = host_side.rsplit(':').next()?.parse().ok();
                    (port, rest)
                }
                None if only_bound => return None,
                None => (None, entry),
            };

            let (container_port_str, protocol) = container_side.split_once('/')?;
            let container_port: u16 = container_port_str.parse().ok()?;
            Some(DockerNetworkBinding {
                host_port,
                container_port,
                protocol: protocol.to_string(),
            })
        })
        .collect()
}

impl From<DockerContainerRaw> for DockerContainer {
    fn from(raw: DockerContainerRaw) -> Self {
        let labels = parse_labels(&raw.labels);
        let compose = extract_compose_labels(&labels);
        let (status, status_detail) = split_status(&raw.status);
        let name = raw.names.split(',').next().unwrap_or("").to_string();
        let state = parse_state(&raw.state);
        let state_raw = if state == DockerContainerState::Unknown {
            Some(raw.state)
        } else {
            None
        };

        DockerContainer {
            id: raw.id,
            name,
            image: raw.image,
            state,
            state_raw,
            status,
            status_detail,
            labels,
            ports: parse_ports(&raw.ports, false),
            compose,
            platform: DockerContainerPlatform {
                architecture: raw.platform.architecture,
                os: raw.platform.os,
                variant: raw.platform.variant,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_returns_client_info() {
        let result = get_status();
        assert!(result.is_ok(), "version() failed: {:?}", result.err());
        let json = serde_json::to_value(result.unwrap()).unwrap();
        let client = json.get("client").expect("missing client");
        assert!(client.get("version").and_then(|v| v.as_str()).is_some());
        assert!(client.get("context").and_then(|v| v.as_str()).is_some());
    }
}
