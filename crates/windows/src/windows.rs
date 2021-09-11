use std::collections::HashMap;

use glium::glutin::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowId,
};

use crate::debugger::generate_debugger;
use crate::emulator::generate_emulator;
use glium::glutin::platform::run_return::EventLoopExtRunReturn;

use glium::Display;

type DisplayMap = HashMap<WindowId, Display>;

pub struct Windows {}

impl Windows {
    fn window_event(map: &mut DisplayMap, event: WindowEvent, id: WindowId) {
        match event {
            WindowEvent::Resized(physical_size) => {
                let display = map.get(&id).unwrap();
                display.gl_window().resize(physical_size);
            }
            WindowEvent::CloseRequested => {
                if map.remove(&id).is_some() {
                    println!("Window with ID {:?} has been closed", id);
                }
            }
            _ => (),
        }
    }

    pub fn run() {
        let mut event_loop = EventLoop::new();
        let mut displays: DisplayMap = HashMap::new();
        let (window_id, display) = generate_debugger(&event_loop);
        displays.insert(window_id, display);
        let (window_id, display) = generate_emulator(&event_loop);
        displays.insert(window_id, display);

        event_loop.run_return(move |event, _, control_flow| {
            match event {
                Event::LoopDestroyed => return,
                Event::WindowEvent { event, window_id } => {
                    Windows::window_event(&mut displays, event, window_id)
                }
                _ => (),
            }
            if displays.is_empty() {
                *control_flow = ControlFlow::Exit
            } else {
                *control_flow = ControlFlow::Wait
            }
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
