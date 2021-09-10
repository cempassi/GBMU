use std::collections::HashMap;

use glium::glutin::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowId,
};

use glium::Display;
use crate::debugger::generate_debugger;
use crate::emulator::generate_emulator;

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
                if let Some(_) = map.remove(&id) {
                    println!("Window with ID {:?} has been closed", id);
                }
            }
            _ => (),
        }
    }

    pub fn run() {
        let event_loop = EventLoop::new();
        let mut displays: DisplayMap = HashMap::new();
        let (window_id, display) = generate_debugger(&event_loop);
        displays.insert(window_id, display);
        let (window_id, display) = generate_emulator(&event_loop);
        displays.insert(window_id, display);

        event_loop.run(move |event, _, control_flow| {
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
