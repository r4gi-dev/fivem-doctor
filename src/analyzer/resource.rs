use crate::analyzer::lua::parse_lua_file;
use crate::analyzer::manifest::{ResourceManifest, parse_manifest};
use crate::diagnostic::Diagnostic;
use crate::rules::lua::analyze_lua;
use anyhow::{Context, Result};
use std::path::PathBuf;

pub struct ResourceAnalyzer {
    root: PathBuf,
}

impl ResourceAnalyzer {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn analyze(&self) -> Result<Vec<Diagnostic>> {
        let mut diagnostics = Vec::new();

        let manifest_path = self.root.join("fxmanifest.lua");

        if !manifest_path.is_file() {
            diagnostics.push(
                Diagnostic::new(
                    "F001",
                    crate::diagnostic::Severity::Error,
                    "Resource manifest not found: fxmanifest.lua",
                    "fxmanifest.lua",
                )
                .with_suggestion("Create an fxmanifest.lua file for this resource."),
            );

            return Ok(diagnostics);
        }

        let manifest = parse_manifest(&manifest_path)
            .with_context(|| format!("failed to analyze {}", manifest_path.display()))?;

        self.check_missing_files(&manifest, &mut diagnostics);
        self.check_lua54(&manifest, &mut diagnostics);
        self.check_lua_files(&manifest, &mut diagnostics)?;

        Ok(diagnostics)
    }

    fn check_missing_files(&self, manifest: &ResourceManifest, diagnostics: &mut Vec<Diagnostic>) {
        let mut referenced_files = Vec::new();

        referenced_files.extend(&manifest.client_scripts);
        referenced_files.extend(&manifest.server_scripts);
        referenced_files.extend(&manifest.shared_scripts);
        referenced_files.extend(&manifest.files);

        for file in referenced_files {
            if file.to_string_lossy().starts_with('@') {
                continue;
            }

            let path = self.root.join(file);
            if !path.exists() {
                diagnostics.push(
                    Diagnostic::new(
                        "F002",
                        crate::diagnostic::Severity::Error,
                        format!("Referenced file does not exist: {}", file.display()),
                        "fxmanifest.lua",
                    )
                    .with_suggestion("Create the file or remove it from fxmanifest.lua."),
                );
            }
        }
    }

    fn check_lua54(&self, manifest: &ResourceManifest, diagnostics: &mut Vec<Diagnostic>) {
        if manifest.lua54 {
            diagnostics.push(
                Diagnostic::new(
                    "F003",
                    crate::diagnostic::Severity::Info,
                    "The 'lua54' manifest entry is deprecated.",
                    "fxmanifest.lua",
                )
                .with_suggestion(
                    "Remove the lua54 entry because Lua 5.4 is now the default runtime.",
                ),
            );
        }
    }

    fn check_lua_files(
        &self,
        manifest: &ResourceManifest,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<()> {
        let mut lua_files = Vec::new();

        lua_files.extend(manifest.client_scripts.iter());
        lua_files.extend(manifest.server_scripts.iter());
        lua_files.extend(manifest.shared_scripts.iter());

        for relative_path in lua_files {
            if relative_path.extension().and_then(|ext| ext.to_str()) != Some("lua") {
                continue;
            }

            let path = self.root.join(relative_path);

            if !path.is_file() {
                continue;
            }

            let ast = parse_lua_file(&path)
                .with_context(|| format!("failed to parse Lua file: {}", path.display()))?;

            let file = relative_path.to_string_lossy();

            diagnostics.extend(analyze_lua(&ast, &file));
        }

        Ok(())
    }
}
