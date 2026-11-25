use anyhow::{Context, Result};

use crate::{
    configs::{PlatformsConfig, WorkspaceManifest},
    sync::{SyncTask, replace::replace_json_string},
};

pub enum MiraiGuiSyncTask {
    PackageJson,
    TauriConf,
}

impl SyncTask for MiraiGuiSyncTask {
    fn process(
        &self,
        platforms: &PlatformsConfig,
        workspace: &WorkspaceManifest,
        input: &str,
    ) -> Result<String> {
        match self {
            Self::PackageJson => replace_json_string(input, "version", &workspace.workspace.package.version),
            Self::TauriConf => {
                let target = platforms
                    .envs
                    .get("MACOSX_DEPLOYMENT_TARGET")
                    .context("Missing MACOSX_DEPLOYMENT_TARGET in platforms.toml [envs]")?;
                replace_json_string(input, "minimumSystemVersion", target)
            },
        }
    }
}
