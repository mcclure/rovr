mod lovr;
mod filesystem;

use mlua::Lua;
use mlua::prelude::{LuaTable, LuaError};

pub fn load(lua: &Lua, table: LuaTable) -> Result<(), LuaError> {
	table.set("lovr", lua.create_function(lovr::make)?)?;
	table.set("lovr.filesystem", lua.create_function(filesystem::make)?)?;

	Ok(())
}