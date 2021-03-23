use std::cell::RefCell;
use std::path::{PathBuf, Component};
use std::fs;
use crate::core::{Result, Error, ErrorKind, print_nonfatal};

// A note on filenames:
// Right now, all filenames are UTF-8. This will change at some point in one of two directions.
// Either I will:
//     Restrict filenames further (disallow backslash, etc as is done in LOVR/C)
// Or:
//     Open up filenames (make filenames WTF-8)
// The former is semi-blocked on Lovr fully documenting its allowed paths.
// The latter is blocked on me writing a fully functional WTF-8 path library.

struct State {
	root:Vec<PathBuf>,
	require_path:String
}

thread_local! {
	static STATE:RefCell<State> = RefCell::new(State {root:Vec::new(), require_path:"".to_string()});
}

pub fn err<T>(e:ErrorKind) -> Result<T> {
	return Err(Error{fatal: false, kind:e})
}

fn path_for_string(archive: PathBuf, path: String) -> Option<PathBuf> {
	let mut filepath = archive.clone();
	let mut depth = 0;
	for component_string in path.split("/") {
		if (!component_string.is_empty()) {
			let fragment_path = PathBuf::from(component_string);
			let mut fragment_components = fragment_path.components();
			let component_option = fragment_components.next();
			match component_option {
				Some(component@Component::Normal(_)) => { depth += 1; filepath.push(component); }
				Some(Component::ParentDir) => { depth -= 1; if (depth < 0) { return None } filepath.pop(); }
				None | Some(Component::RootDir) | Some(Component::CurDir) => {}
				Some(Component::Prefix(_)) => { return None; }
			}
			if (component_option != None && fragment_components.next() != None) { // Shortcircuit required
				return None;
			}
		}
	}
	Some(filepath)
}

pub fn is_file(path: String) -> bool {
	STATE.with(|state| {
		let state = state.borrow();
		
		for archive in &state.root {
			if let Some(filepath) = path_for_string(archive.to_path_buf(), path.clone()) {
				print!("Testing string {}", filepath.to_string_lossy());
				if (filepath.is_file()) {
					return true;
				}
			}
		}

		false
	})
}

pub fn read_file(path:String) -> Result<String> {
	STATE.with(|state| {
		let state = state.borrow();
		
		for archive in &state.root {
			if let Some(filepath) = path_for_string(archive.to_path_buf(), path.clone()) {
				print!("Testing string {}", filepath.to_string_lossy());
				if (filepath.is_file()) {
					return match fs::read_to_string(filepath) {
						Ok(contents) => Ok(contents),
						Err(_) => err(ErrorKind::FileNotAllowed(path))
					}
				}
			}
		}

		err(ErrorKind::FileNotFound(path))
	})
}

pub fn get_source() -> Option<PathBuf> {
	STATE.with(|state| {
		let state = state.borrow_mut();
		
		state.root.iter().next().map(|x| x.clone())
	})
}

pub fn get_identity() -> String {
	"rovr".to_string()
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

fn set_require_path(search: String) {
	STATE.with(|state| {
		let mut state = state.borrow_mut();

		state.require_path = search;
	})
}

pub fn get_require_path() -> String {
	STATE.with(|state| {
		let state = state.borrow_mut();

		state.require_path.clone()
	})
}

pub fn init(target_option:Option<PathBuf>) -> Result<()> {
	// TODO: Check fused exe first
	if let Some(target) = target_option {
		print_nonfatal(mount(target))?;
	}

	set_require_path("?.lua;?/init.lua".to_string());

	Ok(())
}
