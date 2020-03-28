#![deny(clippy::all)]
#![forbid(unsafe_code)]

use pixels::{wgpu::Surface, Error, Pixels, SurfaceTexture};
use std::time::Instant;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
extern crate num_cpus;
extern crate scoped_threadpool;
mod mandelbrot;

#[derive(PartialEq)]
enum ButtonState {
    Releassed,
    JustPressed,
    Pressed,
}

#[derive(Copy, Clone, Debug)]
struct Point {
    x: f64,
    y: f64,
}

struct Context {
    up_left: Point,
    down_right: Point,

    width: u32,
    height: u32,

    k_x: f64,
    k_y: f64,

    left_button: ButtonState,
    coordenate_clicked: Point,
    delta: Point,

    iterations: i64,
    threshold: i64,
    low_res_scale: usize,

    zoom_factor: f64,
}

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let mut world = Context::new();
    let cpus = num_cpus::get();
    let mut deepness: f64 = 0.0;

    let window = {
        let size =
            LogicalSize::new(world.width as f64, world.height as f64).to_physical::<f64>(1.0);
        WindowBuilder::new()
            .with_title("Rusty Mandelbrot")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .with_resizable(false)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let surface = Surface::create(&window);
        let surface_texture = SurfaceTexture::new(world.width, world.height, surface);
        Pixels::new(world.width, world.height, surface_texture)?
    };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => match world.left_button {
                ButtonState::JustPressed => {
                    world.coordenate_clicked.x = position.x;
                    world.coordenate_clicked.y = position.y;
                    world.left_button = ButtonState::Pressed;
                }
                ButtonState::Pressed => {
                    world.delta.x = world.k_x * (position.x - world.coordenate_clicked.x);
                    world.delta.y = world.k_y * (world.coordenate_clicked.y - position.y);
                    window.request_redraw();
                }
                _ => (),
            },
            Event::WindowEvent {
                event: WindowEvent::MouseInput { state, button, .. },
                ..
            } => {
                if button == winit::event::MouseButton::Left {
                    if state == winit::event::ElementState::Pressed {
                        world.left_button = ButtonState::JustPressed;
                    } else {
                        world.left_button = ButtonState::Releassed;
                        world.up_left = Point {
                            x: world.up_left.x - world.delta.x,
                            y: world.up_left.y - world.delta.y,
                        };
                        world.down_right = Point {
                            x: world.down_right.x - world.delta.x,
                            y: world.down_right.y - world.delta.y,
                        };
                        window.request_redraw();
                    }
                }
            }
            Event::WindowEvent {
                event: WindowEvent::MouseWheel { delta, .. },
                ..
            } => {
                let width_excursion = (world.down_right.x - world.up_left.x) / world.zoom_factor;
                let height_excursion = (world.up_left.y - world.down_right.y) / world.zoom_factor;

                if delta == winit::event::MouseScrollDelta::LineDelta(0.0, 1.0) {
                    deepness = width_excursion / world.zoom_factor;
                    world.up_left = Point {
                        x: world.up_left.x + width_excursion / world.zoom_factor,
                        y: world.up_left.y - height_excursion / world.zoom_factor,
                    };
                    world.down_right = Point {
                        x: world.down_right.x - height_excursion / world.zoom_factor,
                        y: world.down_right.y + height_excursion / world.zoom_factor,
                    };
                } else {
                    deepness = width_excursion * world.zoom_factor;
                    world.up_left = Point {
                        x: world.up_left.x - width_excursion * world.zoom_factor,
                        y: world.up_left.y + height_excursion * world.zoom_factor,
                    };
                    world.down_right = Point {
                        x: world.down_right.x + height_excursion * world.zoom_factor,
                        y: world.down_right.y - height_excursion * world.zoom_factor,
                    };
                }
                world.k_x = (world.down_right.x - world.up_left.x) / (world.width as f64);
                world.k_y = (world.up_left.y - world.down_right.y) / (world.width as f64);
                println!("Deepness: {}", deepness);
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                let start = Instant::now();
                if world.left_button == ButtonState::Pressed {
                    let snapshot_up_left = Point {
                        x: world.up_left.x - world.delta.x,
                        y: world.up_left.y - world.delta.y,
                    };
                    let snapshot_donw_right = Point {
                        x: world.down_right.x - world.delta.x,
                        y: world.down_right.y - world.delta.y,
                    };

                    world.parallel_draw_low_res(
                        pixels.get_frame(),
                        snapshot_up_left,
                        snapshot_donw_right,
                        cpus as u32,
                    );
                } else {
                    world.parallel_draw(pixels.get_frame(), cpus as u32);
                }
                let duration = start.elapsed();
                pixels.render();
                println!("Frame time: {:?}", duration);
            }
            _ => (),
        }
    });
}

impl Context {
    fn new() -> Self {
        let up_left_ = Point { x: -2.2, y: 1.5 };
        let down_right_ = Point { x: 0.8, y: -1.5 };

        let width_ = 1000;
        let height_ = 1000;

        Self {
            up_left: up_left_,
            down_right: down_right_,
            width: width_,
            height: height_,
            k_x: (down_right_.x - up_left_.x) / (width_ as f64),
            k_y: (up_left_.y - down_right_.y) / (height_ as f64),
            left_button: ButtonState::Releassed,
            coordenate_clicked: Point { x: 0.0, y: 0.0 },
            delta: Point { x: 0.0, y: 0.0 },
            iterations: 255,
            threshold: 4,
            low_res_scale: 16,
            zoom_factor: 2.0,
        }
    }
    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let pixel_x = (i % self.width as usize) as f64;
            let pixel_y = (i / self.width as usize) as f64;

            let x = self.up_left.x + (pixel_x * self.k_x);
            let y = self.up_left.y - (pixel_y * self.k_y);

            let number = mandelbrot::ComplexNumber { real: x, img: y };
            let degree: u8 =
                mandelbrot::mandebrot_set_degree(number, self.iterations, self.threshold) as u8;
            let channel = 0xff - degree;
            let color = [channel, channel, channel, 0xff];

            pixel.copy_from_slice(&color);
        }
    }
    fn draw_slice(&self, frame: &mut [u8], up_left: Point) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let pixel_x = (i % self.width as usize) as f64;
            let pixel_y = (i / self.width as usize) as f64;

            let x = up_left.x + (pixel_x * self.k_x);
            let y = up_left.y - (pixel_y * self.k_y);

            let number = mandelbrot::ComplexNumber { real: x, img: y };
            let degree: u8 =
                mandelbrot::mandebrot_set_degree(number, self.iterations, self.threshold) as u8;
            let channel = 0xff - degree;
            let color = [channel, channel, channel, 0xff];

            pixel.copy_from_slice(&color);
        }
    }
    fn draw_low_res(&self, frame: &mut [u8], up_left: Point) {
        let mut stored_degree: u8 = 0;

        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            if i % self.low_res_scale == 0 {
                let pixel_x = (i % self.width as usize) as f64;
                let pixel_y = (i / self.width as usize) as f64;

                let x = up_left.x + (pixel_x * self.k_x);
                let y = up_left.y - (pixel_y * self.k_y);

                let number = mandelbrot::ComplexNumber { real: x, img: y };
                let degree: u8 =
                    mandelbrot::mandebrot_set_degree(number, self.iterations, self.threshold) as u8;
                stored_degree = degree;
                let channel = 0xff - degree;
                let color = [channel, channel, channel, 0xff];

                pixel.copy_from_slice(&color);
            } else {
                let channel = 0xff - stored_degree;
                let color = [channel, channel, channel, 0xff];
                pixel.copy_from_slice(&color);
            }
        }
    }
    fn parallel_draw(&self, frame: &mut [u8], thread_num: u32) {
        let mut pool = scoped_threadpool::Pool::new(thread_num);
        let height_slice = (self.up_left.y - self.down_right.y) / thread_num as f64;
        let total_pixel = self.width * self.height * 4;
        let slice_size = (total_pixel / thread_num) as usize;

        pool.scoped(|scope| {
            for (i, slice) in frame.chunks_mut(slice_size).enumerate() {
                let local_up_left = Point {
                    x: self.up_left.x,
                    y: self.up_left.y - (i as f64) * height_slice,
                };

                scope.execute(move || self.draw_slice(slice, local_up_left));
            }
        });
    }
    fn parallel_draw_low_res(
        &self,
        frame: &mut [u8],
        up_left: Point,
        down_right: Point,
        thread_num: u32,
    ) {
        let mut pool = scoped_threadpool::Pool::new(thread_num);
        let height_slice = (up_left.y - down_right.y) / thread_num as f64;
        let total_pixel = self.width * self.height * 4;
        let slice_size = (total_pixel / thread_num) as usize;

        pool.scoped(|scope| {
            for (i, slice) in frame.chunks_mut(slice_size).enumerate() {
                let local_up_left = Point {
                    x: up_left.x,
                    y: up_left.y - (i as f64) * height_slice,
                };
                scope.execute(move || self.draw_low_res(slice, local_up_left));
            }
        });
    }
}
