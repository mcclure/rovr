use std::cell::RefCell;
use std::path::PathBuf;

struct State {
	root:Vec<PathBuf>
}

thread_local! {
	static STATE:RefCell<State> = RefCell::new(State {root:Vec::new()});
}

pub fn init(target:PathBuf) {
	println!("Initial path {}", target.to_str().unwrap_or("(INVALID)"));
	STATE.with(|state| {
		let mut state = state.borrow_mut();
		state.root.push(target)
	})
}
