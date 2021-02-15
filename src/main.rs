#![allow(unused_parens)]

use std::env;
use mlua::{Lua};
use mlua::prelude::LuaError;

fn main() -> Result<(), LuaError> {
	let mut args = env::args().fuse();
	let _ = args.next();
	let arg1 = args.next();

	if (match arg1 {
		None => false, Some(x) => x == "-v" || x == "--version"
	}) {
		println!("ROVR 0.0.1");
		return Ok(()); // Print version and abort
	}

	loop { // Run actual program
		let lua = Lua::new();

		lua.load(
	        r#"
	            print("Printing from Lua")
	        "#,
	    )
	    .set_name("example code")?
	    .exec()?;

	    break;
	}

    Ok(())
}
