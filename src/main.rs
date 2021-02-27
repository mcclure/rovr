#![allow(unused_parens)]

mod api;
mod modules;

use std::env;
use mlua::{Lua, ThreadStatus, Value};
use mlua::prelude::{LuaError, LuaTable, LuaFunction};

fn main() -> Result<(), LuaError> {
    let mut args = env::args().fuse();
    let mut restart = false;
    let arg0 = args.next();
    let arg1 = args.next();
    let arg_rest:Vec<String> = args.collect();

    if (match arg1 {
        None => false, Some(ref x) => x == "-v" || x == "--version"
    }) {
        println!("ROVR 0.0.1");
        return Ok(()); // Print version and abort
    }

    // FIXME handle --console

    loop { // Run actual program
        let lua = Lua::new();

        // TODO: luax_setmainthread(L);

        let arg_table = lua.create_table()?;
        
        // Set arg[-1] and arg[exe]
        let exe;
        if let Some(ref x) = arg0 {
            arg_table.set(-1, x.clone())?;
            exe = x.clone();
        } else {
            exe = "rovr".to_string();
        }
        arg_table.set("exe", exe)?;

        // Set arg[0]
        match arg1 { None => (), Some(ref x) => arg_table.set(0, x.clone())? }

        // TODO set "cookie"

        { // Set arg[1] and later
            let mut idx = 1;
            for arg in &arg_rest {
                arg_table.set(idx, arg.clone())?;
                idx += 1;
            }
        }

        let globals = lua.globals();
        globals.set("arg", arg_table)?;

        let package:LuaTable = globals.get("package")?;
        let preload:LuaTable = package.get("preload")?;
        api::load(&lua, preload)?;

        // TODO lovrModules

        let boot_lua = include_str!("resources/boot.lua");

        let coroutine:LuaFunction = lua.load(
            boot_lua
        )
        .set_name("@boot.lua")?
        .eval()?;

        let coroutine_thread = lua.create_thread(coroutine)?;
        loop {
            let coroutine_result = coroutine_thread.resume(())?;
            if coroutine_thread.status() == ThreadStatus::Resumable {
                // TODO SLEEP
            } else {
                match coroutine_result {
                    Value::String(s) => { if (s == "restart") {restart = true } } // TODO COOKIE
                    _ => ()
                }
                break;
            }
        }

        if (!restart) {
            break;
        }
    }

    Ok(())
}
