use anyhow::{Context, Result};

use crate::{
    configs::{PlatformsConfig, WorkspaceManifest},
    sync::{
        SyncTask,
        replace::{replace_json_string, replace_required},
    },
    types::Language,
};

pub struct PackageJsonSyncTask {
    language: Language,
}

impl PackageJsonSyncTask {
    pub fn new(language: Language) -> Self {
        Self {
            language,
        }
    }
}

impl SyncTask for PackageJsonSyncTask {
    fn process(
        &self,
        platforms: &PlatformsConfig,
        workspace: &WorkspaceManifest,
        input: &str,
    ) -> Result<String> {
        let package = &workspace.workspace.package;

        let mut output = input.to_string();
        output = replace_json_string(&output, "version", &package.version)?;
        output = replace_json_string(&output, "description", &package.description)?;
        output = replace_json_string(&output, "author", &package.authors.join(", "))?;
        output = replace_json_string(&output, "repository", &package.repository)?;
        output = replace_json_string(&output, "license", &package.license)?;

        // The umbrella package installs anywhere: each target's binary is its own package, which napi
        // generates with that target's `os` and `cpu` (see the TypeScript release).
        if self.language == Language::TypeScript {
            let targets = &platforms
                .languages
                .get(&self.language)
                .with_context(|| format!("Missing [languages.{}] in platforms.toml", self.language.name()))?
                .targets;
            output = replace_string_array_field(&output, "targets", targets)?;
        }

        Ok(output)
    }
}

/// Replaces the array at `key`, one item per line, indented one level (4 spaces) past the key.
fn replace_string_array_field(
    input: &str,
    key: &str,
    values: &[String],
) -> Result<String> {
    let items = values.iter().map(serde_json::to_string).collect::<Result<Vec<_>, _>>()?;
    replace_required(input, &format!(r#"(?m)^([ \t]*)("{key}"\s*:\s*)\[[^\]]*\]"#), |captures: &regex::Captures| {
        let indent = &captures[1];
        let items: Vec<String> = items.iter().map(|item| format!("{indent}    {item}")).collect();
        format!("{indent}{}[\n{}\n{indent}]", &captures[2], items.join(",\n"))
    })
}
