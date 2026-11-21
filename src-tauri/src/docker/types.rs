//! Docker CLI result types.

use serde::{Deserialize, Serialize};
use specta::Type;
use std::collections::BTreeMap;

/// Untyped JSON object — passthrough for keys that aren't typed yet.
pub type JsonObject = serde_json::Map<String, serde_json::Value>;

pub trait DockerCommand {
    fn as_command(&self) -> Result<&'static str, String>;
}

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
    /// Docker Compose version, if available.
    pub compose: Option<String>,
}

/// Docker container state — including a partial state for groups/projects.
#[derive(Type, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
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

#[derive(Type, Serialize, Hash)]
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

    // pub labels: BTreeMap<String, Option<String>>,
    pub ports: Vec<DockerNetworkBinding>,
    pub platform: DockerContainerPlatform,

    /// Docker Compose info, if available.
    pub compose: Option<DockerContainerComposeInfo>,

    /// Receipt acts as a change token for the container — it's updated whenever the container changes,
    /// allowing easier change tracking without having to compare all fields.
    pub receipt: Option<String>,

    /// Durable receipt is a receipt that remains unchanged across transient field updates,
    /// such as status changing from `"Up 10 seconds"` to `"Up 11 seconds"`.
    ///
    /// Bumps only when structural fields change.
    /// Useful when consumers need stable container identity across polls and shouldn't react to cosmetic churn.
    ///
    /// Transient fields: raw state, status, status detail. (should these be provided by the frontend instead?)
    pub receipt_durable: Option<String>,
}

#[derive(Type, Serialize, Hash)]
#[serde(rename_all = "camelCase")]
pub struct DockerNetworkBinding {
    pub host_port: Option<u16>,
    pub host_addresses: Vec<String>,
    pub container_port: u16,
    pub protocol: String,
}

#[derive(Type, Serialize, Hash)]
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

#[derive(Type, Serialize, Hash)]
#[serde(rename_all = "camelCase")]
pub struct DockerContainerComposeInfo {
    /// Docker Compose project name.
    pub project: String,
    /// Service name within the project.
    pub service: String,
    /// Compose file paths the project was created from.
    pub config_files: Vec<String>,
}

#[derive(Type, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DockerContainerStats {
    /// Container ID.
    pub id: String,

    pub cpu_percentage: f32,
    pub mem_percentage: f32,
    pub mem_usage_current: Option<String>,
    pub mem_usage_available: Option<String>,

    pub block_io_current: Option<String>,
    pub block_io_available: Option<String>,

    pub net_io_current: Option<String>,
    pub net_io_available: Option<String>,

    pub pids: i32,
}

#[derive(Type, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DockerContainerDetails {
    pub config: DockerContainerConfig,
    pub network_settings: DockerContainerNetworkSettings,
    /// Mounts — volumes, bind mounts, tmpfs, etc.
    pub mounts: Vec<DockerContainerMount>,
    /// Remaining keys — passed through as-is.
    #[specta(type = BTreeMap<String, specta_typescript::Unknown>)]
    pub extra: JsonObject,
}

#[derive(Type, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DockerContainerConfig {
    pub labels: BTreeMap<String, String>,
    pub env: BTreeMap<String, String>,
    /// Remaining keys — passed through as-is.
    #[specta(type = BTreeMap<String, specta_typescript::Unknown>)]
    pub extra: JsonObject,
}

#[derive(Type, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DockerContainerNetworkSettings {
    /// Connected networks — sorted by name.
    pub networks: Vec<DockerContainerNetwork>,
    /// Remaining keys — passed through as-is.
    #[specta(type = BTreeMap<String, specta_typescript::Unknown>)]
    pub extra: JsonObject,
}

#[derive(Type, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DockerContainerNetwork {
    pub name: String,
    pub id: String,
    pub ip_address: String,
    pub gateway: String,
    pub mac_address: String,
    pub aliases: Vec<String>,
    /// Remaining keys — passed through as-is.
    #[specta(type = BTreeMap<String, specta_typescript::Unknown>)]
    pub extra: JsonObject,
}

#[derive(Type, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DockerContainerMount {
    /// Mount type — e.g. `volume`, `bind`, `tmpfs`.
    pub kind: String,
    /// Volume name — only for volume mounts.
    pub name: Option<String>,
    pub source: String,
    pub destination: String,
    pub mode: String,
    pub read_write: bool,
    /// Remaining keys — passed through as-is.
    #[specta(type = BTreeMap<String, specta_typescript::Unknown>)]
    pub extra: JsonObject,
}

#[derive(Type, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum DockerContainerAction {
    #[default]
    Unknown,
    Start,
    Stop,
    Restart,
    Pause,
    Remove,
}

impl DockerCommand for DockerContainerAction {
    fn as_command(&self) -> Result<&'static str, String> {
        Ok(match self {
            DockerContainerAction::Start => "start",
            DockerContainerAction::Stop => "stop",
            DockerContainerAction::Restart => "restart",
            DockerContainerAction::Pause => "pause",
            DockerContainerAction::Remove => "rm",
            _ => return Err(format!("Unknown Docker container action: {:?}", self)),
        })
    }
}

#[derive(Type, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum DockerProjectAction {
    #[default]
    Unknown,
    Up,
    Down,
    Start,
    Stop,
    Restart,
    Pause,
}

impl DockerCommand for DockerProjectAction {
    fn as_command(&self) -> Result<&'static str, String> {
        Ok(match self {
            DockerProjectAction::Up => "up",
            DockerProjectAction::Down => "down",
            DockerProjectAction::Start => "start",
            DockerProjectAction::Stop => "stop",
            DockerProjectAction::Restart => "restart",
            DockerProjectAction::Pause => "pause",
            _ => return Err(format!("Unknown Docker project action: {:?}", self)),
        })
    }
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
