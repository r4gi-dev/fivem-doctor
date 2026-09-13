use anyhow::{Context, Result};
use full_moon::ast::{Ast, Call, FunctionArgs, Prefix, Stmt, Suffix};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Default)]
pub struct ResourceManifest {
    pub client_scripts: Vec<PathBuf>,
    pub server_scripts: Vec<PathBuf>,
    pub shared_scripts: Vec<PathBuf>,
    pub files: Vec<PathBuf>,
    pub dependencies: Vec<String>,
    pub lua54: bool,
}

pub fn parse_manifest(path: &Path) -> Result<ResourceManifest> {
    let source = fs::read_to_string(path)
        .with_context(|| format!("failed to read manifest: {}", path.display()))?;

    let ast = full_moon::parse(&source)
        .map_err(|errors| anyhow::anyhow!("failed to parse fxmanifest.lua: {errors:?}"))?;

    Ok(extract_manifest(&ast))
}

fn extract_manifest(ast: &Ast) -> ResourceManifest {
    let mut manifest = ResourceManifest::default();

    for stmt in ast.nodes().stmts() {
        let Stmt::FunctionCall(call) = stmt else {
            continue;
        };

        let Some(name) = function_call_name(call) else {
            continue;
        };

        let values = function_call_values(call);

        match name.as_str() {
            "client_script" | "client_scripts" => {
                manifest
                    .client_scripts
                    .extend(values.into_iter().map(PathBuf::from));
            }

            "server_script" | "server_scripts" => {
                manifest
                    .server_scripts
                    .extend(values.into_iter().map(PathBuf::from));
            }

            "shared_script" | "shared_scripts" => {
                manifest
                    .shared_scripts
                    .extend(values.into_iter().map(PathBuf::from));
            }

            "file" | "files" => {
                manifest.files.extend(values.into_iter().map(PathBuf::from));
            }

            "dependency" | "dependencies" => {
                manifest.dependencies.extend(values);
            }

            "lua54" if values.iter().any(|value| value.eq_ignore_ascii_case("yes")) => {
                manifest.lua54 = true;
            }

            _ => {}
        }
    }

    manifest
}

fn function_call_name(call: &full_moon::ast::FunctionCall) -> Option<String> {
    match call.prefix() {
        Prefix::Name(name) => Some(name.token().to_string()),
        _ => None,
    }
}

fn function_call_values(call: &full_moon::ast::FunctionCall) -> Vec<String> {
    let Some(suffix) = call.suffixes().next() else {
        return Vec::new();
    };

    let Suffix::Call(Call::AnonymousCall(args)) = suffix else {
        return Vec::new();
    };

    extract_function_args(args)
}

fn extract_function_args(args: &FunctionArgs) -> Vec<String> {
    match args {
        FunctionArgs::String(token) => {
            vec![unquote_lua_string(&token.to_string())]
        }

        FunctionArgs::TableConstructor(table) => extract_strings_from_display(&table.to_string()),

        FunctionArgs::Parentheses { arguments, .. } => arguments
            .iter()
            .flat_map(|expression| extract_strings_from_display(&expression.to_string()))
            .collect(),

        _ => Vec::new(),
    }
}

fn extract_strings_from_display(value: &str) -> Vec<String> {
    let mut values = Vec::new();
    let mut current = String::new();
    let mut quote = None;

    for character in value.chars() {
        match quote {
            Some(active_quote) => {
                if character == active_quote {
                    values.push(current.clone());
                    current.clear();
                    quote = None;
                } else {
                    current.push(character);
                }
            }

            None if character == '\'' || character == '"' => {
                quote = Some(character);
            }

            None => {}
        }
    }

    values
}

fn unquote_lua_string(value: &str) -> String {
    let value = value.trim();

    if value.len() >= 2 {
        let first = value.as_bytes()[0];
        let last = value.as_bytes()[value.len() - 1];

        if (first == b'\'' && last == b'\'') || (first == b'"' && last == b'"') {
            return value[1..value.len() - 1].to_owned();
        }
    }

    value.to_owned()
}
