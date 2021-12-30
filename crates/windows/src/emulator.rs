use iced_wgpu::wgpu::util::StagingBelt;
use soc::SOC;

use gilrs::Gilrs;
use winit_input_helper::WinitInputHelper;
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
    pub input: WinitInputHelper,
    pub gilrs: Gilrs
}

impl Emulator {
    pub fn new(event_loop: &EventLoop<()>, soc: SOC) -> Self {
        let title = "GBMU";
        let input = WinitInputHelper::new();
        let gilrs = Gilrs::new().unwrap();
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
            gilrs,
            input
        }
    }

    pub fn process_event(&mut self, event: WindowEvent, control_flow: &mut ControlFlow) {
        match event {
            WindowEvent::Resized(size) => {
                self.state.resize(size, self.window.scale_factor());
                self.pixels.resize_surface(size.width, size.height);
            }
            WindowEvent::CloseRequested => {
                println!("Request to close on Emulator");
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
        self.state.update();
    }

    pub fn request_redraw(&mut self) {
        self.state.update();
        self.window.request_redraw();
        //        println!("[WINDOW][Emulator] Redraw requested");
    }

    pub fn redraw(&mut self, control_flow: &mut ControlFlow) {
        //       println!("[WINDOW][Emulator] Redrawing");
        let ppu = self.soc.borrow().get_ppu();
        let frame = self.pixels.get_frame();
        ppu.borrow_mut().render(frame);

        let render_result = self.pixels.render_with(|encoder, view, context| {
            let device = &context.device;

            context.scaling_renderer.render(encoder, view);

            let interaction = self
                .state
                .draw(encoder, view, device, &mut self.staging_belt);
            self.staging_belt.finish();

            // Update the mouse cursor
            self.window.set_cursor_icon(mouse_interaction(interaction));

            Ok(())
        });
        // Recall staging buffers
        self.format_pool
            .spawner()
            .spawn(self.staging_belt.recall())
            .expect("Recall staging buffers");

        self.format_pool.run_until_stalled();
        if render_result.is_err() {
            println!("I quited from here!!");
            *control_flow = ControlFlow::Exit;
        }
        // Then we submit the work
    }
}
