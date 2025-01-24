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
        self.window_size = (window_phisical.width as i32, window_phisical.height as i32);
        self.canvas = Canvas::new(self.window_size);

        let surface_texture = SurfaceTexture::new(
            window_phisical.width,
            window_phisical.height,
            self.window.as_ref().unwrap(),
        );

        self.pixels = Some(
            PixelsBuilder::new(
                window_phisical.width,
                window_phisical.height,
                surface_texture,
            )
            .surface_texture_format(wgpu::TextureFormat::Bgra8UnormSrgb)
            .clear_color(wgpu::Color::TRANSPARENT)
            .blend_state(wgpu::BlendState::ALPHA_BLENDING)
            .build()
            .unwrap(),
        );
    }
}
