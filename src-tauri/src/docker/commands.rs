use std::collections::{BTreeMap, HashMap};
use std::hash::{DefaultHasher, Hash, Hasher};

use serde::Deserialize;

use crate::docker::types::DockerCommand;

use super::cli::Docker;
use super::types::{
    DockerClientVersion, DockerComposeProject, DockerContainer, DockerContainerAction,
    DockerContainerComposeInfo, DockerContainerPlatform, DockerContainerResult,
    DockerContainerState, DockerContainerStats, DockerNetworkBinding, DockerProjectAction,
    DockerServerVersion, DockerStatus,
};

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct DockerVersionRaw {
    client: DockerClientVersionRaw,
    server: Option<DockerServerVersionRaw>,
}

#[derive(Deserialize)]
struct DockerComposeVersionRaw {
    version: String,
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
    architecture: String,
    os: String,
    variant: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct DockerContainerStatsRaw {
    /// Container ID.
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "BlockIO")]
    block_io: String,
    #[serde(rename = "CPUPerc")]
    cpu_perc: String,
    mem_perc: String,
    mem_usage: String,
    #[serde(rename = "NetIO")]
    net_io: String,
    #[serde(rename = "PIDs")]
    pids: String,
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

const LABEL_COMPOSE_PROJECT: &str = "com.docker.compose.project";
const LABEL_COMPOSE_SERVICE: &str = "com.docker.compose.service";
const LABEL_COMPOSE_PROJECT_CONFIG_FILES: &str = "com.docker.compose.project.config_files";

pub fn get_status() -> Result<DockerStatus, String> {
    let raw: DockerVersionRaw = Docker::cmd("version")
        .json()
        .run_json_safe()?
        .ok_or_else(|| "Could not retrieve Docker version".to_string())?;

    let compose_raw: Option<DockerComposeVersionRaw> = Docker::cmd("compose")
        .arg("version")
        .json()
        .run_json_safe()?;
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
        compose: compose_raw.map(|c| c.version),
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

pub fn run_container_action(action: &DockerContainerAction, id: &str) -> Result<(), String> {
    let command = action.as_command()?;
    let container = get_container(id)?;
    let cli = if let Some(compose) = &container.compose {
        let extra_args: &[&str] = match action {
            DockerContainerAction::Remove => &["-s", "-f"],
            _ => &[],
        };

        Docker::cmd("compose")
            .args(["-p", &compose.project])
            .arg(command)
            .args(extra_args.iter().copied())
            .arg(&compose.service)
            .log_prefix(
                log::Level::Info,
                format!("Docker container action (ID: {})", id),
            )
    } else {
        Docker::cmd(command)
            .arg(id)
            .log_prefix(log::Level::Info, "Docker container action")
    };

    cli.run()?;
    Ok(())
}

pub fn run_project_action(action: &DockerProjectAction, project: &str) -> Result<(), String> {
    // Project commands — fetch project location and invoke "docker compose" commands there
    let project_container = get_project_container(project)?;
    let compose = project_container
        .compose
        .as_ref()
        .ok_or_else(|| format!("Docker project not found: {}", project))?;

    let command = action.as_command()?;
    let cli = Docker::cmd("compose")
        .args(["-p", &compose.project])
        .arg(command)
        .log_prefix(log::Level::Info, "Docker project action");

    let _project_file = extract_project_config_file(&project_container)
        .ok_or_else(|| format!("Project config file not found: {}", project))?;

    cli.run()?;
    Ok(())
}

pub fn get_container_stats() -> Result<Vec<DockerContainerStats>, String> {
    let raw: Vec<DockerContainerStatsRaw> = Docker::cmd("stats")
        .arg("--no-stream")
        .json()
        .run_ndjson()?;

    Ok(raw.into_iter().map(Into::into).collect())
}

fn get_container(id: &str) -> Result<DockerContainer, String> {
    Docker::cmd("ps")
        .arg("-a")
        .filter("id", id)
        .json()
        .run_ndjson_safe::<DockerContainerRaw>()?
        // Pick the first container from the result
        .and_then(|v| v.into_iter().next())
        .map(Into::into)
        .ok_or_else(|| format!("Docker container not found: {}", id))
}

fn get_project_container(project: &str) -> Result<DockerContainer, String> {
    let raw: DockerContainerRaw = create_project_container_command(project)
        .args(["--last", "1"])
        .run_json()?;

    Ok(raw.into())
}

fn extract_project_config_file(container: &DockerContainer) -> Option<String> {
    container
        .labels
        .get(LABEL_COMPOSE_PROJECT_CONFIG_FILES)
        .and_then(|l| l.as_deref())
        .and_then(|l| l.split(',').map(str::trim).next())
        .map(|f| f.to_owned())
}

fn create_project_container_command(project: &str) -> Docker {
    Docker::cmd("ps")
        .arg("-a")
        .filter("label", &format!("{}={}", LABEL_COMPOSE_PROJECT, project))
        .json()
}

fn build_compose_projects(containers: &[DockerContainer]) -> Vec<DockerComposeProject> {
    // Iterate over the containers once and collect their project-related fields...
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

    // ...and then build the project list based on the collected fields.
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

fn parse_labels(s: &str) -> BTreeMap<String, Option<String>> {
    if s.is_empty() {
        return BTreeMap::new();
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
    labels: &BTreeMap<String, Option<String>>,
) -> Option<DockerContainerComposeInfo> {
    let project = labels.get(LABEL_COMPOSE_PROJECT)?.clone()?;
    let service = labels.get(LABEL_COMPOSE_SERVICE)?.clone()?;
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
    type Key = (Option<u16>, u16, String);
    let mut network_bindings: std::collections::HashMap<Key, DockerNetworkBinding> =
        std::collections::HashMap::new();

    for entry in s.split(',').map(str::trim).filter(|p| !p.is_empty()) {
        let (host_port, host_address, container_side) = match entry.split_once("->") {
            Some((host_side, rest)) => {
                let port = host_side.rsplit(':').next().and_then(|p| p.parse().ok());
                let address = host_side.rsplit_once(':').map(|(addr, _)| addr.to_string());
                (port, address, rest)
            }
            None if only_bound => continue,
            None => (None, None, entry),
        };

        let Some((container_port_str, protocol)) = container_side.split_once('/') else {
            continue;
        };
        let Ok(container_port) = container_port_str.parse::<u16>() else {
            continue;
        };

        let network_binding = network_bindings
            .entry((host_port, container_port, protocol.to_string()))
            .or_insert(DockerNetworkBinding {
                host_port,
                host_addresses: vec![],
                container_port,
                protocol: protocol.to_string(),
            });

        if let Some(addr) = host_address {
            if !network_binding.host_addresses.contains(&addr) {
                network_binding.host_addresses.push(addr);
            }
        }
    }

    let mut result = network_bindings.into_values().collect::<Vec<_>>();
    result.sort_by_key(|b| b.container_port);
    result
}

fn parse_current_available(s: &str) -> (Option<String>, Option<String>) {
    let parts: Vec<&str> = s.split('/').map(str::trim).collect();
    let current = parts.get(0).map(|s| (*s).to_owned());
    let available = parts.get(1).map(|s| (*s).to_owned());
    (current, available)
}

fn hash_receipts(container: &DockerContainer) -> (String, String) {
    // Receipt (default)
    let mut hasher = DefaultHasher::new();
    container.hash(&mut hasher);
    let receipt = format!("{:x}", hasher.finish()); // 16-char hex

    // Durable receipt — ignore transient fields
    let mut hasher_durable = DefaultHasher::new();
    container.id.hash(&mut hasher_durable);
    container.name.hash(&mut hasher_durable);
    container.image.hash(&mut hasher_durable);
    container.state.hash(&mut hasher_durable);
    container.labels.hash(&mut hasher_durable);
    container.ports.hash(&mut hasher_durable);
    container.platform.hash(&mut hasher_durable);
    container.compose.hash(&mut hasher_durable);
    let receipt_durable = format!("{:x}", hasher_durable.finish()); // 16-char hex

    (receipt, receipt_durable)
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

        let mut container = DockerContainer {
            id: raw.id,
            name,
            image: raw.image,
            state,
            state_raw,
            status,
            status_detail,
            labels,
            ports: parse_ports(&raw.ports, false),
            platform: DockerContainerPlatform {
                architecture: raw.platform.architecture,
                os: raw.platform.os,
                variant: raw.platform.variant,
            },
            compose,
            receipt: None,
            receipt_durable: None,
        };

        let (receipt, receipt_durable) = hash_receipts(&container);
        container.receipt = Some(receipt);
        container.receipt_durable = Some(receipt_durable);
        container
    }
}

impl From<DockerContainerStatsRaw> for DockerContainerStats {
    fn from(raw: DockerContainerStatsRaw) -> Self {
        let (mem_usage_current, mem_usage_available) = parse_current_available(&raw.mem_usage);
        let (block_io_current, block_io_available) = parse_current_available(&raw.block_io);
        let (net_io_current, net_io_available) = parse_current_available(&raw.net_io);
        DockerContainerStats {
            id: raw.id,
            cpu_percentage: raw.cpu_perc.trim_end_matches('%').parse().unwrap_or(0.0),
            mem_percentage: raw.mem_perc.trim_end_matches('%').parse().unwrap_or(0.0),
            mem_usage_current,
            mem_usage_available,
            block_io_current,
            block_io_available,
            net_io_current,
            net_io_available,
            pids: raw.pids.parse().unwrap_or(-1),
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
