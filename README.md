# snake
It's rusty snake built the hard way

![Demo picture](https://github.com/manorajesh/snake/blob/master/images/demo.png)

## Installation
```
git clone https://github.com/manorajesh/snake.git && cd snake
cargo run
```

## Usage
Use the arrow keys. It's snake.

## Why?
I wanted to get familiar with [`pixels`](https://docs.rs/pixels/latest/pixels/) and [`winit`](https://github.com/rust-windowing/winit) before starting on my [raycasting](https://github.com/manorajesh/raycaster)/gamedev journey. I wanted access to a framebuffer and was thinking about [SDL2](https://github.com/Rust-SDL2/rust-sdl2), but there is minor setup on Windows, and I was lazy. `pixels` did look like a nice crate as well given that it is simple and hardware accelerated (and cross-platform might I add, looking at you Windows GDI). 