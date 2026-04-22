use serde::{Deserialize, Serialize};
use specta::Type;
use super::cli::DockerCli;

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

#[derive(Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct DockerClientVersion {
    name: String,
    version: String,
    api_version: String,
    os: String,
    arch: String,
    context: String,
}

#[derive(Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct DockerServerVersion {
    name: String,
    version: String,
    api_version: String,
    os: String,
    arch: String,
}

#[derive(Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct DockerStatus {
    client: DockerClientVersion,
    server: Option<DockerServerVersion>,
}

pub fn version() -> Result<DockerStatus, String> {
    let cli = DockerCli::new();
    let args = cli.build_args("version", &["--format", "json"]);
    let stdout = cli.run_allow_failure(&args)?;
    let raw: DockerVersionRaw = serde_json::from_str(&stdout).map_err(|e| e.to_string())?;
    Ok(DockerStatus {
        client: DockerClientVersion {
            name: raw.client.platform.name,
            version: raw.client.version,
            api_version: raw.client.api_version,
            os: raw.client.os,
            arch: raw.client.arch,
            context: raw.client.context,
        },
        server: raw.server.map(|s| DockerServerVersion {
            name: s.platform.name,
            version: s.version,
            api_version: s.api_version,
            os: s.os,
            arch: s.arch,
        }),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_returns_client_info() {
        let result = version();
        assert!(result.is_ok(), "version() failed: {:?}", result.err());
        let json = serde_json::to_value(result.unwrap()).unwrap();
        let client = json.get("client").expect("missing client");
        assert!(client.get("version").and_then(|v| v.as_str()).is_some());
        assert!(client.get("context").and_then(|v| v.as_str()).is_some());
    }
}
