use iced_wgpu::wgpu::util::StagingBelt;
use soc::SOC;

use pixels::SurfaceTexture;

use iced_winit::{
    conversion::{mouse_interaction, window_event},
    futures::{executor::LocalPool, task::SpawnExt},
    winit::{
        dpi::LogicalSize,
        event::{ModifiersState, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::{Window, WindowBuilder, WindowId},
    },
};
use pixels::Pixels;

const WIDTH: u32 = 160;
const HEIGHT: u32 = 144;

pub struct Emulator {
    pub id: WindowId,
    pub window: Window,
    pub state: ui::Emulator,
    pub modifiers: ModifiersState,
    pub resized: bool,
    pub staging_belt: StagingBelt,
    pub format_pool: LocalPool,
    pub pixels: Pixels,
    pub soc: SOC,
}

impl Emulator {
    pub fn new(event_loop: &EventLoop<()>, soc: SOC) -> Self {
        let title = "GBMU";
        let window = {
            let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
            WindowBuilder::new()
                .with_title(title)
                .with_inner_size(size)
                .with_min_inner_size(size)
                .build(event_loop)
                .unwrap()
        };
        let modifiers = ModifiersState::default();

        let id = window.id();
        let resized = false;

        // Initialize wgpu
        let size = window.inner_size();
        let surface_texture = SurfaceTexture::new(size.width, size.height, &window);
        let pixels = Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap();

        // Initialize staging belt and local pool
        let staging_belt = StagingBelt::new(5 * 1024);
        let format_pool = LocalPool::new();

        let state = ui::Emulator::new(&window, &pixels);

        Self {
            id,
            window,
            modifiers,
            state,
            resized,
            staging_belt,
            format_pool,
            pixels,
            soc,
        }
    }

    pub fn process_event(&mut self, event: WindowEvent, control_flow: &mut ControlFlow) {
        match event {
            WindowEvent::Resized(size) => {
                self.state.resize(size, self.window.scale_factor());
                self.pixels.resize_surface(size.width, size.height);
            }
            WindowEvent::CloseRequested => {
                println!("Request to close on debugger");
                *control_flow = ControlFlow::Exit;
            }
            WindowEvent::ModifiersChanged(new_modifiers) => {
                self.modifiers = new_modifiers;
            }
            _ => (),
        };
        if let Some(event) = window_event(&event, self.window.scale_factor(), self.modifiers) {
            self.state.state.queue_event(event);
        }
    }

    pub fn update(&mut self) {
        self.state.update(self.window.scale_factor());
    }

    pub fn request_redraw(&mut self) {
        self.state.update(self.window.scale_factor());
        self.window.request_redraw();
    }

    pub fn redraw(&mut self, control_flow: &mut ControlFlow) {
        let ppu = self.soc.borrow().get_ppu();
        let frame = self.pixels.get_frame();
        ppu.borrow_mut().render(frame);

        let render_result = self.pixels.render_with(|encoder, view, context| {
            let device = &context.device;

            // // Clear the screen to white before drawing
            // let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            //     label: Some("Reset Screen"),
            //     color_attachments: &[wgpu::RenderPassColorAttachment {
            //         view,
            //         resolve_target: None,
            //         ops: wgpu::Operations {
            //             load: wgpu::LoadOp::Clear(wgpu::Color {
            //                 r: 1.0,
            //                 g: 1.0,
            //                 b: 1.0,
            //                 a: 1.0,
            //             }),
            //             store: true,
            //         },
            //     }],
            //     depth_stencil_attachment: None,
            // });
            // drop(_render_pass);
            context.scaling_renderer.render(encoder, view);

            let interaction = self
                .state
                .draw(encoder, view, device, &mut self.staging_belt);

            // Update the mouse cursor
            self.window.set_cursor_icon(mouse_interaction(interaction));

            // And recall staging buffers
            self.format_pool
                .spawner()
                .spawn(self.staging_belt.recall())
                .expect("Recall staging buffers");

            self.staging_belt.finish();
            self.format_pool.run_until_stalled();
            Ok(())
        });
        if render_result.is_err() {
            *control_flow = ControlFlow::Exit;
        }
        // Then we submit the work
    }
}
