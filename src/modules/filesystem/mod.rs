use std::cell::RefCell;
use std::path::PathBuf;
use crate::core::{Result, Error, ErrorKind, print_nonfatal};

struct State {
	root:Vec<PathBuf>
}

thread_local! {
	static STATE:RefCell<State> = RefCell::new(State {root:Vec::new()});
}

pub fn err(e:ErrorKind) -> Result<()> {
	return Err(Error{fatal: false, kind:e})
}

pub fn mount(target:PathBuf) -> Result<()> {
	STATE.with(|state| {
		let mut state = state.borrow_mut();

		// If target already exists in root list, return error
		if state.root.contains(&target) {
			return err(ErrorKind::FileExists(target.to_string_lossy().into_owned()))
		}

		if (!target.is_dir()) {
			if (!target.exists()) {
				return err(ErrorKind::FileExists(target.to_string_lossy().into_owned()))
			}
			return err(ErrorKind::FileWrongFormat(target.to_string_lossy().into_owned()))
		}

		state.root.push(target);

		Ok(())
	})
}

pub fn init(target:PathBuf) -> Result<()> {
	// TODO: Check fused exe first
	print_nonfatal(mount(target))?;
	Ok(())
}
