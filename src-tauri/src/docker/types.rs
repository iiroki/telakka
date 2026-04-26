//! Docker CLI result types.

use serde::{Deserialize, Serialize};
use specta::Type;
use std::collections::HashMap;

/// Docker client version info.
#[derive(Type, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DockerClientVersion {
    pub name: String,
    pub version: String,
    pub api_version: String,
    pub os: String,
    pub arch: String,
    pub context: String,
}

/// Docker server/engine version info.
#[derive(Type, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DockerServerVersion {
    pub name: String,
    pub version: String,
    pub api_version: String,
    pub os: String,
    pub arch: String,
}

/// Docker status — containing both, client and server versions.
#[derive(Type, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DockerStatus {
    pub client: DockerClientVersion,
    /// This can be unavailable if the Docker daemon is unavailable.
    pub server: Option<DockerServerVersion>,
}

/// Docker container state — including a partial state for groups/projects.
#[derive(Type, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum DockerContainerState {
    #[default]
    Unknown,
    Created,
    Running,
    Paused,
    Restarting,
    Removing,
    Exited,
    Dead,
    /// Docker Compose state only — indicates that some containers in the project are running, but not all.
    Partial,
}

#[derive(Type, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DockerContainer {
    pub id: String,
    pub name: String,
    pub image: String,
    pub state: DockerContainerState,
    /// Raw state — only populated if the `state` could not be determined.
    pub state_raw: Option<String>,
    pub status: String,
    pub status_detail: Option<String>,

    pub labels: HashMap<String, Option<String>>,
    pub ports: Vec<DockerNetworkBinding>,
    pub platform: DockerContainerPlatform,

    /// Docker Compose info, if available.
    pub compose: Option<DockerContainerComposeInfo>,
}

#[derive(Type, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DockerNetworkBinding {
    pub host_port: Option<u16>,
    pub container_port: u16,
    pub protocol: String,
}

#[derive(Type, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DockerContainerPlatform {
    pub architecture: String,
    pub os: String,
    pub variant: Option<String>,
}

#[derive(Type, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DockerComposeProject {
    pub project: String,
    pub state: DockerContainerState,
}

#[derive(Type, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DockerContainerComposeInfo {
    /// Docker Compose project name.
    pub project: String,
    /// Service name within the project.
    pub service: String,
}

#[derive(Type, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum DockerContainerCommand {
    #[default]
    Unknown,
    Start,
    Stop,
    Restart,
    Pause,
    Remove,
}

//
// Results
//

/// Docker container list result — contains both, individual containers and Docker Compose projects.
#[derive(Type, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DockerContainerResult {
    pub containers: Vec<DockerContainer>,
    pub projects: Vec<DockerComposeProject>,
}
