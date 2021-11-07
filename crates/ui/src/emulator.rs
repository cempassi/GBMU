mod ui;

use self::ui::UserInterface;

use iced_wgpu::wgpu::util::StagingBelt;
use iced_wgpu::wgpu::{CommandEncoder, Device, TextureView};
use iced_wgpu::{Backend, Renderer, Settings, Viewport};
use iced_winit::winit::dpi::PhysicalSize;
use iced_winit::winit::{dpi::PhysicalPosition, window::Window};
use iced_winit::Clipboard;
use iced_winit::{mouse::Interaction, program, Debug, Point, Size};
use pixels::Pixels;

pub struct Emulator {
    pub state: program::State<UserInterface>,
    pub clipboard: Clipboard,
    pub debug: Debug,
    pub viewport: Viewport,
    pub renderer: Renderer,
    pub cursor: PhysicalPosition<f64>,
}

impl Emulator {
    pub fn new(window: &Window, pixels: &Pixels) -> Self {
        let user_interface = UserInterface::default();
        let mut debug = Debug::new();
        let clipboard = Clipboard::connect(window);

        let physical_size = window.inner_size();
        let size = Size::new(physical_size.width, physical_size.height);
        let viewport = Viewport::with_physical_size(size, window.scale_factor());

        let cursor = PhysicalPosition::new(-1.0, -1.0);
        let logical_cursor = cursor.to_logical(window.scale_factor());
        let point = Point::new(logical_cursor.x, logical_cursor.y);

        let mut renderer = Renderer::new(Backend::new(
            pixels.device(),
            Settings::default(),
            pixels.render_texture_format(),
        ));

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

    pub fn update(&mut self) {
        let scale_factor = self.viewport.scale_factor();
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
        encoder: &mut CommandEncoder,
        view: &TextureView,
        device: &Device,
        staging_belt: &mut StagingBelt,
    ) -> Interaction {
        self.renderer.backend_mut().draw(
            device,
            staging_belt,
            encoder,
            view,
            &self.viewport,
            self.state.primitive(),
            &self.debug.overlay(),
        )
    }
}
