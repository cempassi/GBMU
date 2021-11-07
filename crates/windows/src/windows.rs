use iced_wgpu::wgpu::Instance;
use iced_winit::winit::event::{Event, StartCause};
use iced_winit::winit::event_loop::{ControlFlow, EventLoop};
use soc::{TryInit, SOC};

use crate::debugger;
use crate::emulator;

pub struct Windows {}

impl Windows {
    pub fn run(name: &str) {
        let soc = SOC::try_init(name).unwrap();
        let event_loop = EventLoop::new();

        // Fix draw on top of fullscreen issue on macos
        #[cfg(target_os = "macos")]
        unsafe {
            use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivationPolicy::*};
            let ns_app = NSApp();
            ns_app.setActivationPolicy_(NSApplicationActivationPolicyRegular);
        }

        let instance = Instance::new(iced_wgpu::wgpu::Backends::PRIMARY);
        let mut debugger = debugger::Debugger::new(&event_loop, &instance, soc.clone());
        let mut emulator = emulator::Emulator::new(&event_loop, soc.clone());
        event_loop.run(move |event, _, flow| {
            *flow = ControlFlow::Poll;

            // Handle Events
            match event {
                Event::NewEvents(StartCause::Init) => {}
                Event::WindowEvent { event, window_id } if window_id == debugger.id => {
                    debugger.process_event(event, flow);
                }
                Event::WindowEvent { event, window_id } if window_id == emulator.id => {
                    emulator.process_event(event, flow);
                }
                Event::MainEventsCleared => {
                    debugger.request_redraw();
                    emulator.request_redraw();
                }
                Event::RedrawRequested(window_id) if window_id == debugger.id => {
                    debugger.redraw();
                }
                Event::RedrawRequested(window_id) if window_id == emulator.id => {
                    emulator.redraw(flow);
                }
                _ => (),
            };

            // Run Emulator here
            if soc.borrow_mut().run() {
                debugger.state.refresh();
            }
        })
    }
}
