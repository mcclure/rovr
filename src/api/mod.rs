#![allow(non_snake_case)] // This module contains Lua function implementations with Lua style names

mod lovr;
mod filesystem;

use mlua::Lua;
use mlua::prelude::{LuaTable, LuaResult, LuaError};
use crate::core;

pub fn load(lua: &Lua, table: LuaTable) -> Result<(), LuaError> {
	table.set("lovr", lua.create_function(lovr::make)?)?;
	table.set("lovr.filesystem", lua.create_function(filesystem::make)?)?;

	Ok(())
}

// API tools

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
