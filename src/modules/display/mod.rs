//! Roughly corresponds to rovr.display

use std::cell::RefCell;
use crate::core::{Result, Error, ErrorKind, print_nonfatal};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

// TODO: Rather than TLS we want a single pointer, access restricted to main thread.
// This capability would be more efficient and might be worth creating a crate for.
struct State {
	event_loop: EventLoop<()>,
	window: Window
}

thread_local! {
	static STATE:RefCell<Option<State>> = RefCell::new(None);
}

pub fn init() -> Result<()> {
//	STATE.with(|state| { // Winit messed everything up by turning out to have a nonreturning method
	    let event_loop = EventLoop::new();
    	let window = WindowBuilder::new().build(&event_loop).unwrap();

    	//let state = State {window, event_loop};
    	//state.replace(Some(state));

    	event_loop.run(move |event, _, control_flow: &mut ControlFlow| {
			*control_flow = ControlFlow::Wait;

	        match event {
	            Event::WindowEvent {
	                event: WindowEvent::CloseRequested,
	                window_id,
	            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
	            _ => (),
	        }
	    });

    	Ok(())
//    })
}
