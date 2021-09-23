mod ui;

use iced_wgpu::wgpu::util::StagingBelt;
use iced_wgpu::wgpu::{
    CommandEncoderDescriptor, Device, SurfaceFrame, TextureFormat, TextureViewDescriptor,
};
use iced_wgpu::{Backend, Renderer, Settings, Viewport};
use iced_winit::winit::dpi::PhysicalSize;
use iced_winit::winit::{dpi::PhysicalPosition, window::Window};
use iced_winit::Clipboard;
use iced_winit::{mouse::Interaction, program, Debug, Point, Size};
use ui::UserInterface;

pub struct Debugger {
    pub state: program::State<UserInterface>,
    pub clipboard: Clipboard,
    pub debug: Debug,
    pub viewport: Viewport,
    pub renderer: Renderer,
    pub cursor: PhysicalPosition<f64>,
}

impl Debugger {
    pub fn new(window: &Window, device: &Device, format: TextureFormat) -> Self {
        let user_interface = UserInterface::new();
        let mut debug = Debug::new();
        let clipboard = Clipboard::connect(window);

        let physical_size = window.inner_size();
        let size = Size::new(physical_size.width, physical_size.height);
        let viewport = Viewport::with_physical_size(size, window.scale_factor());

        let cursor = PhysicalPosition::new(-1.0, -1.0);
        let logical_cursor = cursor.to_logical(window.scale_factor());
        let point = Point::new(logical_cursor.x, logical_cursor.y);

        let mut renderer = Renderer::new(Backend::new(device, Settings::default(), format));

        let state = program::State::new(
            user_interface,
            viewport.logical_size(),
            point,
            &mut renderer,
            &mut debug,
        );

        Self {
            state,
            clipboard,
            debug,
            cursor,
            viewport,
            renderer,
        }
    }

    pub fn update(&mut self, scale_factor: f64) {
        println!("In debugger update");
        let logical_cursor = self.cursor.to_logical(scale_factor);
        let point = Point::new(logical_cursor.x, logical_cursor.y);

        let _ = self.state.update(
            self.viewport.logical_size(),
            point,
            &mut self.renderer,
            &mut self.clipboard,
            &mut self.debug,
        );
    }

    pub fn resize(&mut self, physical_size: PhysicalSize<u32>, scale_factor: f64) {
        let size = Size::new(physical_size.width, physical_size.height);
        self.viewport = Viewport::with_physical_size(size, scale_factor);
    }

    pub fn draw(
        &mut self,
        frame: &SurfaceFrame,
        device: &mut Device,
        staging_belt: &mut StagingBelt,
    ) -> Interaction {
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor { label: None });
        let view = frame
            .output
            .texture
            .create_view(&TextureViewDescriptor::default());
        self.renderer.backend_mut().draw(
            device,
            staging_belt,
            &mut encoder,
            &view,
            &self.viewport,
            self.state.primitive(),
            &self.debug.overlay(),
        )
    }
}
