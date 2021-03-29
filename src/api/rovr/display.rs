//! Modules visible to Lua API under "rovr" namespace
// This file contains Lua functions

use mlua::Lua;
use mlua::prelude::{LuaTable, LuaResult, LuaError, LuaValue, LuaMultiValue};
use crate::modules::display;
use crate::api;
use api::{forgive_nonfatal, core_result_to_lua};

pub fn make(lua: &Lua, _: ()) -> Result<LuaTable, LuaError> {
	forgive_nonfatal(display::init())?;

	let table = lua.create_table()?;

	Ok(table)
}
