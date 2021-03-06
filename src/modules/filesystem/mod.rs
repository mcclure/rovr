use std::cell::RefCell;
use std::path::{PathBuf, Component};
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
	root:Vec<PathBuf>
}

thread_local! {
	static STATE:RefCell<State> = RefCell::new(State {root:Vec::new()});
}

pub fn err(e:ErrorKind) -> Result<()> {
	return Err(Error{fatal: false, kind:e})
}

pub fn is_file(path: String) -> bool {
	STATE.with(|state| {
		let state = state.borrow();
		
		'archive: for archive in &state.root {
			let mut filename = archive.clone();
			let mut depth = 0;
			for component_string in path.split("/") {
				if (!component_string.is_empty()) {
					let fragment_path = PathBuf::from(component_string);
					let mut fragment_components = fragment_path.components();
					let component_option = fragment_components.next();
					match component_option {
						Some(component@Component::Normal(_)) => { depth += 1; filename.push(component); }
						Some(Component::ParentDir) => { depth -= 1; if (depth < 0) { continue 'archive; } filename.pop(); }
						None | Some(Component::RootDir) | Some(Component::CurDir) => {}
						Some(Component::Prefix(_)) => { continue 'archive; }
					}
					if (component_option != None && fragment_components.next() != None) { // Shortcircuit required
						continue 'archive;
					}
				}
			}
			print!("Testing string {}", filename.to_string_lossy());
			if (filename.is_file()) {
				return true;
			}
		}

		false
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

pub fn init(target:PathBuf) -> Result<()> {
	// TODO: Check fused exe first
	print_nonfatal(mount(target))?;
	Ok(())
}
