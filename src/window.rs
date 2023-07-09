use pixels::{Pixels, SurfaceTexture, Error};
use winit::{window::{Window, WindowBuilder}, event_loop::EventLoop, dpi::LogicalSize};

pub struct GameWindow {
    pub window: Window,
    pub size: (u32, u32),
    pub pixels: Pixels,
}

impl GameWindow {
    pub fn new(title: &str, event_loop: &EventLoop<()>) -> Result<Self, Error> {
        let size = LogicalSize::new(400.0, 300.0);
        let window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(size)
            .build(event_loop)
            .unwrap();

        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        let pixels = Pixels::new(window_size.width, window_size.height, surface_texture)?;

        Ok(Self {
            window,
            size: (window_size.width, window_size.height),
            pixels,
        })
    }

    pub fn resize(&mut self, new_size: (u32, u32)) {
        self.pixels.resize_surface(new_size.0, new_size.1).unwrap();
        self.size = new_size;
    }

    pub fn redraw(&mut self) -> Result<(), pixels::Error> {
        let frame: &mut [u8] = self.pixels.frame_mut();

        for pixel in frame.chunks_exact_mut(4) {
            pixel[0] = 0x00; // Red
            pixel[1] = 0xff; // Green
            pixel[2] = 0xaf; // Blue
            pixel[3] = 0xff; // Alpha
        }

        self.pixels.render()
    }
}
