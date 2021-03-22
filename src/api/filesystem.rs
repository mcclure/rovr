// This file contains Lua functions

use mlua::Lua;
use mlua::prelude::{LuaError, LuaResult, LuaValue, LuaString, LuaTable};
use crate::modules::filesystem;
use std::path::PathBuf;
use super::forgive_nonfatal;

fn unimplemented(_: &Lua, _: ()) -> LuaResult<()> {
	Err(LuaError::RuntimeError("This function is not implemented yet in rovr.".to_string()))
}

fn load_lua_file(lua: &Lua, path:String) -> LuaResult<LuaValue> {
	let contents = filesystem::read_file(path.clone()).map_err(|e| LuaError::RuntimeError(e.to_string()))?;
	return lua.load(
            &contents
        )
        .set_name(&path)?
        .eval();
}

fn lua_loader<'a>(lua: &'a Lua, path: LuaString) -> LuaResult<LuaValue<'a>> {
	let pathStr = path.to_str()?;
	let require_path = filesystem::get_require_path();
	let candidates = require_path.split(';');

	for candidate in candidates {
		let path = candidate.replace("?", pathStr);
		if (filesystem::is_file(path.clone())) {
			return load_lua_file(lua, path);
		}
	}

	Ok(LuaValue::Nil)
}

fn lib_loader<'a>(_: &'a Lua, path: LuaString) -> LuaResult<LuaValue<'a>> {
	let extension = if (cfg!(target_os = "windows")) { ".dll" } else { ".so" };

	Ok(LuaValue::Nil)
}

fn getIdentity(lua: &Lua, _: ()) -> LuaResult<LuaString> {
	Ok(lua.create_string(&filesystem::get_identity())?)
}

fn getRequirePath(lua:&Lua, _: ()) -> LuaResult<LuaString> {
	Ok(lua.create_string(&filesystem::get_require_path())?)
}

fn getSource(lua: &Lua, _: ()) -> LuaResult<LuaValue> {
	match filesystem::get_source() {
		Some(path) =>
			if let Some(s) = path.to_str() { Ok(LuaValue::String(lua.create_string(&s.to_string())?)) }
			else {Err(LuaError::RuntimeError("Could not convert path to string".to_string()))},
		None =>	Ok(LuaValue::Nil)
	}
}

fn isFile(_: &Lua, path: LuaString) -> LuaResult<bool> {
	Ok(filesystem::is_file(path.to_str()?.to_string()))
}

fn isFused(_:&Lua, _:()) -> LuaResult<LuaValue> {
	Ok(LuaValue::Boolean(false)) // TODO
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

	super::register_loader(lua, lua.create_function(lua_loader)?, 2)?;
	super::register_loader(lua, lua.create_function(lib_loader)?, 3)?;

	let table = lua.create_table()?;

	table.set("append", lua.create_function(unimplemented)?)?;
	table.set("createDirectory", lua.create_function(unimplemented)?)?;
	table.set("getAppdataDirectory", lua.create_function(unimplemented)?)?;
	table.set("getDirectoryItems", lua.create_function(unimplemented)?)?;
	table.set("getExecutablePath", lua.create_function(unimplemented)?)?;
	table.set("getIdentity", lua.create_function(getIdentity)?)?;
	table.set("getLastModified", lua.create_function(unimplemented)?)?;
	table.set("getRealDirectory", lua.create_function(unimplemented)?)?;
	table.set("getRequirePath", lua.create_function(getRequirePath)?)?;
	table.set("getSaveDirectory", lua.create_function(unimplemented)?)?;
	table.set("getSize", lua.create_function(unimplemented)?)?;
	table.set("getSource", lua.create_function(getSource)?)?;
	table.set("getUserDirectory", lua.create_function(unimplemented)?)?;
	table.set("getWorkingDirectory", lua.create_function(unimplemented)?)?;
	table.set("isDirectory", lua.create_function(unimplemented)?)?;
	table.set("isFile", lua.create_function(isFile)?)?;
	table.set("isFused", lua.create_function(isFused)?)?;
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
