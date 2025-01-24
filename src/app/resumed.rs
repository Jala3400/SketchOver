use super::App;
use crate::canvas::Canvas;
use pixels::{wgpu, PixelsBuilder, SurfaceTexture};
use winit::event_loop::ActiveEventLoop;

impl App {
    // This is a common indicator that you can create a window.
    pub fn resumed_func(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(event_loop.create_window(self.attributes.clone()).unwrap());
        let window = self.window.as_ref().unwrap();

        let monitor = window
            .current_monitor()
            .unwrap_or_else(|| window.primary_monitor().unwrap());

        self.assign_monitor(&monitor);

        self.update_circle_cursor(event_loop);

        let window_phisical = self.window.as_ref().unwrap().inner_size();
        self.window_size = (window_phisical.width, window_phisical.height);
        self.canvas = Canvas::new(self.window_size);

        let surface_texture = SurfaceTexture::new(
            self.window_size.0,
            self.window_size.1,
            self.window.as_ref().unwrap(),
        );

        self.pixels = Some(
            PixelsBuilder::new(self.window_size.0, self.window_size.1, surface_texture)
                .surface_texture_format(wgpu::TextureFormat::Bgra8UnormSrgb)
                .clear_color(wgpu::Color::TRANSPARENT)
                .blend_state(wgpu::BlendState::ALPHA_BLENDING)
                .build()
                .unwrap(),
        );
    }
}
