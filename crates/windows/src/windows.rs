use iced_wgpu::wgpu::Instance;
use iced_winit::winit::event::Event;
use iced_winit::winit::event_loop::EventLoop;
use soc::{TryInit, SOC};

use crate::debugger;
use crate::emulator;

pub struct Windows {}

impl Windows {
    pub fn run(name: &str) {
        let soc = SOC::try_init(name).unwrap();
        let event_loop = EventLoop::new();
        let instance = Instance::new(iced_wgpu::wgpu::Backends::PRIMARY);
        let mut debugger = debugger::Debugger::new(&event_loop, &instance, soc.clone());
        let mut emulator = emulator::Emulator::new(&event_loop, soc.clone());

        event_loop.run(move |event, _, control_flow| {
            // Handle Events
            match event {
                Event::LoopDestroyed => (),
                Event::WindowEvent { event, window_id } if window_id == debugger.id => {
                    debugger.process_event(event, control_flow);
                }
                Event::WindowEvent { event, window_id } if window_id == emulator.id => {
                    emulator.process_event(event, control_flow);
                }
                Event::MainEventsCleared => {
                    // If there are events pending
                    if !debugger.state.state.is_queue_empty() {
                        // We update iced and request a redraw
                        debugger.request_redraw();
                        emulator.request_redraw();
                    }
                }
                Event::RedrawRequested(window_id) if window_id == debugger.id => {
                    debugger.redraw();
                }
                Event::RedrawRequested(window_id) if window_id == emulator.id => {
                    emulator.redraw(control_flow);
                }
                _ => (),
            };

            // Run Emulator here
            if soc.borrow_mut().run() {
                debugger.request_redraw();
                emulator.request_redraw();
            }
        })
    }
}
