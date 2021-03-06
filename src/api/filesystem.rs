// This file contains Lua functions

use mlua::Lua;
use mlua::prelude::{LuaError, LuaResult, LuaValue, LuaString, LuaTable};
use crate::modules::filesystem;
use std::path::PathBuf;
use super::forgive_nonfatal;

fn unimplemented(_: &Lua, _: ()) -> LuaResult<()> {
	Err(LuaError::RuntimeError("This function is not implemented yet in rovr.".to_string()))
}

fn isFile(_: &Lua, path: LuaString) -> LuaResult<bool> {
	Ok(filesystem::is_file(path.to_str()?.to_string()))
}

fn getSource(lua: &Lua, _: ()) -> LuaResult<LuaValue> {
	match filesystem::get_source() {
		Some(path) =>
			if let Some(s) = path.to_str() { Ok(LuaValue::String(lua.create_string(&s.to_string())?)) }
			else {Err(LuaError::RuntimeError("Could not convert path to string".to_string()))},
		None =>	Ok(LuaValue::Nil)
	}
}


fn getIdentity(lua: &Lua, _: ()) -> LuaResult<LuaString> {
	Ok(lua.create_string(&filesystem::get_identity())?)
}


pub fn make(lua: &Lua, _: ()) -> LuaResult<LuaTable> {
	let globals = lua.globals();

	if let Ok(LuaValue::Table(arg)) = globals.get("arg") {
		let target:LuaString = arg.get(0)?;
		let targetString = target.to_str()?;

		forgive_nonfatal(filesystem::init(PathBuf::from(targetString)))?;
	} else {
		return Err(LuaError::RuntimeError("Internal error: arg array not found".to_string()));
	}

	let table = lua.create_table()?;

	table.set("append", lua.create_function(unimplemented)?)?;
	table.set("createDirectory", lua.create_function(unimplemented)?)?;
	table.set("getAppdataDirectory", lua.create_function(unimplemented)?)?;
	table.set("getDirectoryItems", lua.create_function(unimplemented)?)?;
	table.set("getExecutablePath", lua.create_function(unimplemented)?)?;
	table.set("getIdentity", lua.create_function(getIdentity)?)?;
	table.set("getLastModified", lua.create_function(unimplemented)?)?;
	table.set("getRealDirectory", lua.create_function(unimplemented)?)?;
	table.set("getRequirePath", lua.create_function(unimplemented)?)?;
	table.set("getSaveDirectory", lua.create_function(unimplemented)?)?;
	table.set("getSize", lua.create_function(unimplemented)?)?;
	table.set("getSource", lua.create_function(getSource)?)?;
	table.set("getUserDirectory", lua.create_function(unimplemented)?)?;
	table.set("getWorkingDirectory", lua.create_function(unimplemented)?)?;
	table.set("isDirectory", lua.create_function(unimplemented)?)?;
	table.set("isFile", lua.create_function(isFile)?)?;
	table.set("isFused", lua.create_function(unimplemented)?)?;
	table.set("load", lua.create_function(unimplemented)?)?;
	table.set("mount", lua.create_function(unimplemented)?)?;
	table.set("newBlob", lua.create_function(unimplemented)?)?;
	table.set("read", lua.create_function(unimplemented)?)?;
	table.set("remove", lua.create_function(unimplemented)?)?;
	table.set("setRequirePath", lua.create_function(unimplemented)?)?;
	table.set("setIdentity", lua.create_function(unimplemented)?)?;
	table.set("unmount", lua.create_function(unimplemented)?)?;
	table.set("write", lua.create_function(unimplemented)?)?;
	
	Ok(table)
}
