#![allow(non_snake_case)] // This module contains Lua function implementations with Lua style names

mod lovr;
mod filesystem;

use mlua::Lua;
use mlua::prelude::{LuaTable, LuaFunction, LuaResult, LuaError};
use crate::core;

pub fn load(lua: &Lua, table: LuaTable) -> Result<(), LuaError> {
	table.set("lovr", lua.create_function(lovr::make)?)?;
	table.set("lovr.filesystem", lua.create_function(filesystem::make)?)?;

	Ok(())
}

pub fn register_loader(lua: &Lua, loader: LuaFunction, index: i32) -> Result<(), LuaError> {
	let globals = lua.globals();
    
	let table:LuaTable = globals.get("table")?;
    let insert:LuaFunction = table.get("insert")?;
    drop(table);

    let package:LuaTable = globals.get("package")?;
	let searchers:LuaTable = package.get("loaders")?; // TODO: In Lua 5.2 should be "searchers"
	drop(package);

	insert.call::<_, ()>((searchers, index, loader))?;

    Ok(())
}

// API tools

// FIXME: This could be done with Into
pub fn core_result_to_lua<T>(result: core::Result<T>) -> LuaResult<T> {
	result.map_err(|e| LuaError::RuntimeError(e.to_string()))
}

// FIXME: This is almost a precise copypaste of the versions in core/mod. Could this be generalized somehow?

pub fn print_nonfatal(result: core::Result<()>) -> LuaResult<bool> {
	if let Err(error) = result {
		if (error.fatal) {
			Err(LuaError::RuntimeError(error.to_string()))
		} else {
			print!("Warning: {}", error.to_string());
			Ok(false)
		}
	} else {
		Ok(true)
	}
}

pub fn forgive_nonfatal(result: core::Result<()>) -> LuaResult<bool> {
	if let Err(error) = result {
		if (error.fatal) {
			Err(LuaError::RuntimeError(error.to_string()))
		} else {
			Ok(false)
		}
	} else {
		Ok(true)
	}
}
