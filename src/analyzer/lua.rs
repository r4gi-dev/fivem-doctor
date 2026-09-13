use anyhow::Result;
use full_moon::ast::Ast;
use std::fs;
use std::path::Path;

pub fn parse_lua_file(path: &Path) -> Result<Ast> {
    let source = fs::read_to_string(path)?;

    let ast = full_moon::parse(&source)
        .map_err(|errors| anyhow::anyhow!("failed to parse Lua: {errors:?}"))?;

    Ok(ast)
}
