use anyhow::Result;

use crate::{
    configs::{PlatformsConfig, WorkspaceManifest},
    sync::{SyncTask, replace::replace_required},
};

/// The files the TypeScript build stamps with the version, kept in step without building.
pub enum TypeScriptVersionSyncTask {
    Constant,
    NapiLoader,
}

impl SyncTask for TypeScriptVersionSyncTask {
    fn process(
        &self,
        _platforms: &PlatformsConfig,
        workspace: &WorkspaceManifest,
        input: &str,
    ) -> Result<String> {
        let replacement = format!("${{1}}{}${{2}}", workspace.workspace.package.version);
        match self {
            Self::Constant => replace_required(input, r"(export const VERSION = ')[^']*(')", &replacement),
            Self::NapiLoader => {
                let output = replace_required(input, r"(bindingPackageVersion !== ')[^']*(')", &replacement)?;
                replace_required(&output, r"(version mismatch, expected )\S+( but got)", &replacement)
            },
        }
    }
}
