use soc::SOC;

use iced_wgpu::wgpu::{
    self, util::StagingBelt, CommandEncoderDescriptor, Device, DeviceDescriptor, Features,
    Instance, Limits, PowerPreference, PresentMode, Queue, RequestAdapterOptions, Surface,
    SurfaceConfiguration, SurfaceError, TextureFormat, TextureUsages, TextureViewDescriptor,
};

use iced_winit::{
    conversion::mouse_interaction,
    futures::{executor::LocalPool, task::SpawnExt},
    winit::{
        dpi::{PhysicalSize, Size},
        event::{ModifiersState, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::{Window, WindowBuilder, WindowId},
    },
};

pub struct Debugger {
    pub id: WindowId,
    pub window: Window,
    pub surface: Surface,
    pub device: Device,
    pub queue: Queue,
    pub state: ui::Debugger,
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
                force_fallback_adapter: false,
                compatible_surface: Some(surface),
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

impl Debugger {
    pub fn new(event_loop: &EventLoop<()>, instance: &Instance, soc: SOC) -> Self {
        let title = "Debugger";
        let size = Size::Physical(PhysicalSize::new(1340, 768));
        let window = WindowBuilder::new()
            .with_title(title)
            .with_resizable(true)
            .with_inner_size(size)
            .build(event_loop)
            .unwrap();

        let modifiers = ModifiersState::default();

        let id = window.id();
        let resized = false;

        // Initialize wgpu
        let surface = unsafe { instance.create_surface(&window) };
        let (format, device, queue) = init_device(&window, instance, &surface);

        // Initialize staging belt and local pool
        let staging_belt = StagingBelt::new(5 * 1024);
        let format_pool = LocalPool::new();

        let state = ui::Debugger::new(&window, &device, format, soc);

        Self {
            id,
            window,
            surface,
            device,
            queue,
            modifiers,
            state,
            resized,
            staging_belt,
            format,
            format_pool,
        }
    }

    pub fn process_event(&mut self, event: WindowEvent, control_flow: &mut ControlFlow) {
        match event {
            WindowEvent::Resized(physical_size) => {
                self.state.resize(physical_size, self.window.scale_factor());
                self.resized = true;
            }
            WindowEvent::CloseRequested => {
                println!("Request to close on debugger");
                *control_flow = ControlFlow::Exit;
                return;
            }
            WindowEvent::ModifiersChanged(new_modifiers) => {
                self.modifiers = new_modifiers;
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.state.cursor = position;
            }
            _ => (),
        };
        if let Some(event) =
            iced_winit::conversion::window_event(&event, self.window.scale_factor(), self.modifiers)
        {
                self.state.state.queue_event(event);
        }
    }

    pub fn request_redraw(&mut self) {
        self.state.update();
        self.window.request_redraw();
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
            self.resize();
        }

        match self.surface.get_current_texture() {
            Ok(frame) => {
                // Generate the encoder to create the actual commands to send to the gpu.
                let mut encoder = self
                    .device
                    .create_command_encoder(&CommandEncoderDescriptor {
                        label: Some("Render Encoder"),
                    });

                // Generate the view that we will render to.
                let view = frame.texture.create_view(&TextureViewDescriptor::default());

                // Clear the screen to white before drawing
                let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 1.0,
                                g: 1.0,
                                b: 1.0,
                                a: 1.0,
                            }),
                            store: true,
                        },
                    }],
                    depth_stencil_attachment: None,
                });
                drop(_render_pass);

                // And then iced on top
                let mouse_action = self.state.renderer.backend_mut().draw(
                    &self.device,
                    &mut self.staging_belt,
                    &mut encoder,
                    &view,
                    &self.state.viewport,
                    self.state.state.primitive(),
                    &self.state.debug.overlay(),
                );

                // Then we submit the work
                self.staging_belt.finish();
                self.queue.submit(Some(encoder.finish()));
                frame.present();

                // Update the mouse cursor
                self.window.set_cursor_icon(mouse_interaction(mouse_action));

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
                    println!("Couldn't get current texture");
                    self.window.request_redraw();
                }
            },
        }
    }
}
