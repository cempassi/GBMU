use iced_wgpu::wgpu::{
    util::StagingBelt, CommandEncoderDescriptor, Device, DeviceDescriptor, Features, Instance,
    Limits, PowerPreference, PresentMode, Queue, RequestAdapterOptions, Surface,
    SurfaceConfiguration, SurfaceError, TextureFormat, TextureUsages, TextureViewDescriptor,
};

use iced_winit::{
    //conversion::mouse_interaction,
    futures::{executor::LocalPool, task::SpawnExt},
    winit::{
        event::{ModifiersState, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::{Window, WindowId},
    },
};

pub struct Emulator {
    pub id: WindowId,
    pub window: Window,
    pub surface: Surface,
    pub device: Device,
    pub queue: Queue,
    //pub state: gui::Debugger,
    pub modifiers: ModifiersState,
    pub resized: bool,
    pub staging_belt: StagingBelt,
    pub format: TextureFormat,
    pub format_pool: LocalPool,
}

fn init_device(
    window: &Window,
    instance: &Instance,
    surface: &Surface,
) -> (TextureFormat, Device, Queue) {
    let (format, (device, queue)) = iced_winit::futures::executor::block_on(async {
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Request adapter");

        (
            surface
                .get_preferred_format(&adapter)
                .expect("Get preferred format"),
            adapter
                .request_device(
                    &DeviceDescriptor {
                        label: None,
                        features: Features::empty(),
                        limits: Limits::default(),
                    },
                    None,
                )
                .await
                .expect("Request device"),
        )
    });

    let size = window.inner_size();
    surface.configure(
        &device,
        &iced_wgpu::wgpu::SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width,
            height: size.height,
            present_mode: PresentMode::Mailbox,
        },
    );
    (format, device, queue)
}

impl Emulator {
    pub fn new(event_loop: &EventLoop<()>, instance: &Instance) -> Self {
        let title = "Debugger";
        let window = Window::new(&event_loop).unwrap();
        window.set_title(title);
        let modifiers = ModifiersState::default();

        let id = window.id();
        let resized = false;

        // Initialize wgpu
        let surface = unsafe { instance.create_surface(&window) };
        let (format, device, queue) = init_device(&window, &instance, &surface);

        // Initialize staging belt and local pool
        let staging_belt = StagingBelt::new(5 * 1024);
        let format_pool = LocalPool::new();

        //let state = gui::Debugger::new(&window, &device, format);

        Self {
            id,
            window,
            surface,
            device,
            queue,
            modifiers,
            format,
            //state,
            resized,
            staging_belt,
            format_pool,
        }
    }
    pub fn process_event(&mut self, event: WindowEvent, control_flow: &mut ControlFlow) {
        match event {
            WindowEvent::Resized(_physical_size) => {
                //        self.state.resize(physical_size, self.window.scale_factor());
                self.resized = true;
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
        if let Some(event) =
            iced_winit::conversion::window_event(&event, self.window.scale_factor(), self.modifiers)
        {
            println!("Queuing event: {:?}", event);
            //   self.state.state.queue_event(event);
        }
    }

    pub fn update(&mut self) {
        //self.state.update(self.window.scale_factor());
    }

    fn resize(&mut self) {
        let size = self.window.inner_size();

        self.surface.configure(
            &self.device,
            &SurfaceConfiguration {
                usage: TextureUsages::RENDER_ATTACHMENT,
                format: self.format,
                width: size.width,
                height: size.height,
                present_mode: PresentMode::Mailbox,
            },
        );

        self.resized = false;
    }

    pub fn redraw(&mut self) {
        if self.resized {
            self.resize()
        }

        match self.surface.get_current_frame() {
            Ok(_frame) => {
                let encoder = self
                    .device
                    .create_command_encoder(&CommandEncoderDescriptor { label: None });

                //let program = self.state.state.program();

                // let view = frame
                //     .output
                //     .texture
                //     .create_view(&TextureViewDescriptor::default());

                // And then iced on top
                // let mouse_action = self.state.renderer.backend_mut().draw(
                //     &mut self.device,
                //     &mut self.staging_belt,
                //     &mut encoder,
                //     &view,
                //     &self.state.viewport,
                //     self.state.state.primitive(),
                //     &self.state.debug.overlay(),
                // );

                // Then we submit the work
                self.staging_belt.finish();
                self.queue.submit(Some(encoder.finish()));

                // Update the mouse cursor
                //self.window.set_cursor_icon(mouse_interaction(mouse_action));

                // And recall staging buffers
                self.format_pool
                    .spawner()
                    .spawn(self.staging_belt.recall())
                    .expect("Recall staging buffers");

                self.format_pool.run_until_stalled();
            }
            Err(error) => match error {
                SurfaceError::OutOfMemory => {
                    panic!("Swapchain error: {}. Rendering cannot continue.", error)
                }
                _ => {
                    // Try rendering again next frame.
                    self.window.request_redraw();
                }
            },
        }
    }
}
