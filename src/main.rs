mod window;
mod snake;

use std::time::{Instant, Duration};
use pixels::Error;
use winit::{event_loop::EventLoop, event::{Event, WindowEvent, VirtualKeyCode, ElementState}};

pub const WIDTH: u32 = 200;
pub const HEIGHT: u32 = 200;

fn main() -> Result<(), Error>{
    let event_loop = EventLoop::new();
    let mut gw = window::GameWindow::new("snake", &event_loop)?;
    let mut snake = snake::Game::new();

    let mut last_update = Instant::now();
    let update_rate = Duration::from_millis(100);

    event_loop.run(move |event, _, control_flow| {
        let now = Instant::now();
        let dt = now - last_update;

        // Update game state if enough time has passed
        if dt >= update_rate {
            last_update = now;

            if snake.update() {
                *control_flow = winit::event_loop::ControlFlow::Exit;
            }
        }

        match event {
            Event::RedrawRequested(_) => {
                println!("Redraw requested");
                snake.draw(gw.pixels.frame_mut(), gw.size.0);
                gw.pixels.render().unwrap();
            }

            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                println!("Window closed");
                *control_flow = winit::event_loop::ControlFlow::Exit;
            }

            Event::WindowEvent { event: WindowEvent::Resized(size), .. } => {
                println!("Window resized to {:?}", size);
                gw.resize((size.width, size.height));
                snake.resize(size.width, size.height);
            }

            Event::WindowEvent { event: WindowEvent::KeyboardInput { input, .. }, .. } => {
                println!("Keyboard input detected");
                match input.virtual_keycode {
                    Some(VirtualKeyCode::Up) if input.state == ElementState::Pressed => {
                        snake.change_direction(snake::Direction::Up);
                    }
                    Some(VirtualKeyCode::Down) if input.state == ElementState::Pressed => {
                        snake.change_direction(snake::Direction::Down);
                    }
                    Some(VirtualKeyCode::Left) if input.state == ElementState::Pressed => {
                        snake.change_direction(snake::Direction::Left);
                    }
                    Some(VirtualKeyCode::Right) if input.state == ElementState::Pressed => {
                        snake.change_direction(snake::Direction::Right);
                    }
                    _ => {}
                }
            }

            _ => {}
        }

        gw.window.request_redraw();
    });
}

