mod window;
mod snake;

use pixels::Error;
use winit::{event_loop::EventLoop, event::{Event, WindowEvent}};

fn main() -> Result<(), Error>{
    let event_loop = EventLoop::new();
    let mut gw = window::GameWindow::new("snake", &event_loop)?;

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::RedrawRequested(_) => {
                gw.redraw().unwrap();
            }

            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = winit::event_loop::ControlFlow::Exit;
            }

            Event::WindowEvent { event: WindowEvent::Resized(size), .. } => {
                gw.resize((size.width, size.height));
            }

            _ => {}
        }
    });
}
