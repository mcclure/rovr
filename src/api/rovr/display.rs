// This file contains Lua functions

use mlua::Lua;
use mlua::prelude::{LuaTable, LuaResult, LuaError, LuaValue, LuaMultiValue};

pub fn make(lua: &Lua, _: ()) -> Result<LuaTable, LuaError> {
	let table = lua.create_table()?;

	Ok(table)
}
