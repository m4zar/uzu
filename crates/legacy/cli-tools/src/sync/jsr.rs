use anyhow::Result;

use crate::{
    configs::{PlatformsConfig, WorkspaceManifest},
    sync::{SyncTask, replace::replace_json_string},
};

pub struct JsrSyncTask;

impl SyncTask for JsrSyncTask {
    fn process(
        &self,
        _platforms: &PlatformsConfig,
        workspace: &WorkspaceManifest,
        input: &str,
    ) -> Result<String> {
        replace_json_string(input, "version", &workspace.workspace.package.version)
    }
}
