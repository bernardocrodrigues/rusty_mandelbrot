#![deny(clippy::all)]
#![forbid(unsafe_code)]

use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    window::Window,
};
extern crate scoped_threadpool;

use pixels::{wgpu::Surface, Error, Pixels, SurfaceTexture};

mod mandelbrot;

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 1000;
const BOX_SIZE: i16 = 128;

struct World {
    box_x: i16,
    box_y: i16,
    velocity_x: i16,
    velocity_y: i16,
}

fn main() -> Result<(), Error> {
    
    let event_loop = EventLoop::new();

    let mut pressed = false;
    let mut just_pressed = false;

    let mut point_clicked = (0.0, 0.0);
    let mut delta = (0.0, 0.0);

    let mut up_left = (-2.2, 1.5);
    let mut down_right = (0.8, -1.5);

    let mut step_x = (down_right.0 - up_left.0)/(WIDTH as f64);
    let mut step_y = (up_left.1 - down_right.1)/(WIDTH as f64);

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let surface = Surface::create(&window);
        let surface_texture = SurfaceTexture::new(WIDTH, HEIGHT, surface);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let mut world = World::new();
    
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        // println!("{:?}\n", event);
        // if (pressed) {
        // }

        // window.request_redraw();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                *control_flow = ControlFlow::Exit
            },

            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {

                if (pressed) {
                    if(!just_pressed){
                        point_clicked.0 = position.x;
                        point_clicked.1 = position.y;
                        just_pressed = true;
                        // println!("point_clicked: {:?}\n", point_clicked);
                    }
                    delta.0 = (position.x - point_clicked.0) * step_x;
                    delta.1 = (point_clicked.1 - position.y) * step_y;
                    // println!("{:?}\n", delta);
                    window.request_redraw();
                }
                
                // aux = position.x as u8;
                // aux2 = position.y as u8;
                // *control_flow = ControlFlow::Exit
            },

            Event::WindowEvent {
                event: WindowEvent::MouseInput { state, button, .. },
                ..
            } => {

                if (button == winit::event::MouseButton::Left){

                    if(state == winit::event::ElementState::Pressed){
                        pressed = true;
                    }else{
                        pressed = false;
                        just_pressed = false;
                        up_left = (up_left.0 - delta.0, up_left.1 - delta.1);
                        down_right = (down_right.0 - delta.0, down_right.1 - delta.1);
                        window.request_redraw();
                    }
                    
                    // println!("{:?}\n", state);
                }

            },

            Event::WindowEvent {
                event: WindowEvent::MouseWheel { delta, .. },
                ..
            } => {

                let factor = 2.0;

                let w = (down_right.0 - up_left.0)/2.0;
                let h = (up_left.1 - down_right.1)/2.0;

                    if delta == winit::event::MouseScrollDelta::LineDelta(0.0, 1.0) {
                        up_left = (up_left.0 + w/factor, up_left.1 - h/factor);
                        down_right = (down_right.0 - h/factor, down_right.1 + h/factor);
                        step_x = (down_right.0 - up_left.0)/(WIDTH as f64);
                        step_y = (up_left.1 - down_right.1)/(WIDTH as f64);
                        // println!("{:?} {:?}", up_left, down_right);
                        println!("IN");
                    }else{
                        up_left = (up_left.0 - w*factor, up_left.1 + h*factor);
                        down_right = (down_right.0 + h*factor, down_right.1 - h*factor);
                        step_x = (down_right.0 - up_left.0)/(WIDTH as f64);
                        step_y = (up_left.1 - down_right.1)/(WIDTH as f64);
                        println!("OUT");
                    }
                window.request_redraw();

            },


            // WindowEvent { window_id: WindowId(X(WindowId(102760449))), event: MouseWheel { device_id: DeviceId(X(DeviceId(2))), delta: LineDelta(0.0, -1.0), phase: Moved, modifiers: (empty) } }
            // Event::WindowEvent {
            //     event: WindowEvent::MouseInput { state: Released, button: Left, .. },
            //     ..
            // } => {
            //     println!("Released\n");
            //     pressed = false;
            // },

            // MouseInput { device_id: DeviceId(X(DeviceId(2))), state: Pressed, button: Left

            Event::MainEventsCleared => {
                // Application update code.
                // world.update();
    
                // Queue a RedrawRequested event.
                // window.request_redraw();
            },
            Event::RedrawRequested(_) => {

                if pressed {
                    let up_left_mod = (up_left.0 - delta.0, up_left.1 - delta.1);
                    let down_right_mod = (down_right.0 - delta.0, down_right.1 - delta.1);
                    world.parallel_draw_low_res(pixels.get_frame(), up_left_mod, down_right_mod, step_x, step_y);
                }else{
                    world.parallel_draw(pixels.get_frame(), up_left, down_right, step_x, step_y);
                }
                
                pixels.render();

                // Redraw the application.
                //
                // It's preferrable to render in this event rather than in MainEventsCleared, since
                // rendering in here allows the program to gracefully handle redraws requested
                // by the OS.
            },
            _ => ()
            
        }
    });

}



impl World {
    /// Create a new `World` instance that can draw a moving box.
    fn new() -> Self {
        Self {
            box_x: 24,
            box_y: 16,
            velocity_x: 1,
            velocity_y: 1,
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self) {
        if self.box_x <= 0 || self.box_x + BOX_SIZE - 1 >= WIDTH as i16 {
            self.velocity_x *= -1;
        }
        if self.box_y <= 0 || self.box_y + BOX_SIZE - 1 >= HEIGHT as i16 {
            self.velocity_y *= -1;
        }

        self.box_x += self.velocity_x;
        self.box_y += self.velocity_y;
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: [`wgpu::TextureFormat::Rgba8UnormSrgb`]
    fn draw(&self, frame: &mut [u8], up_left: (f64,f64), down_right:(f64,f64), step_x: f64, step_y: f64) {

        // println!("{:?} {:?}", up_left, down_right);

        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {

            let pixel_x = (i % WIDTH as usize) as i16;
            let pixel_y = (i / WIDTH as usize) as i16;

            let x = up_left.0 + (pixel_x as f64 * step_x);
            let y = up_left.1 - (pixel_y as f64 * step_y);

            let aux = mandelbrot::ComplexNumber { real : x, img : y};

            let degree:u8 = mandelbrot::mandebrot_set_degree(aux, 255, 4) as u8;

            let color = [0xff - degree, 0xff - degree, 0xff - degree, 0xff];

            pixel.copy_from_slice(&color);
        }
    }

    fn parallel_draw(&self, frame: &mut [u8], up_left: (f64,f64), down_right:(f64,f64), step_x: f64, step_y: f64){

        let mut pool = scoped_threadpool::Pool::new(8);

        let w = (down_right.0 - up_left.0)/8.0;
        let h = (up_left.1 - down_right.1)/8.0;

        let total_pixel = WIDTH * HEIGHT * 4;

        pool.scoped(|scope| {
            for (i, slice) in frame.chunks_mut((total_pixel/8) as usize).enumerate() {

                let my_up_left = (up_left.0, up_left.1 - (i as f64) * h );

                let my_down_right = (down_right.0, (up_left.1 - h) - ((i as f64) * h) );

                scope.execute(move || self.draw(slice, my_up_left, my_down_right, step_x, step_y));
            }
        });
    }

    fn parallel_draw_low_res(&self, frame: &mut [u8], up_left: (f64,f64), down_right:(f64,f64), step_x: f64, step_y: f64){

        let mut pool = scoped_threadpool::Pool::new(8);

        let w = (down_right.0 - up_left.0)/8.0;
        let h = (up_left.1 - down_right.1)/8.0;

        let total_pixel = WIDTH * HEIGHT * 4;

        pool.scoped(|scope| {
            for (i, slice) in frame.chunks_mut((total_pixel/8) as usize).enumerate() {

                let my_up_left = (up_left.0, up_left.1 - (i as f64) * h );

                let my_down_right = (down_right.0, (up_left.1 - h) - ((i as f64) * h) );

                scope.execute(move || self.draw_low_res(slice, my_up_left, my_down_right, step_x, step_y));
            }
        });
    }

    fn draw_low_res(&self, frame: &mut [u8], up_left: (f64,f64), down_right:(f64,f64), step_x: f64, step_y: f64) {

        let mut stored_degree:u8 = 0;
        
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {

            if i % 8 == 0 {

                let pixel_x = (i % WIDTH as usize) as i16;
                let pixel_y = (i / WIDTH as usize) as i16;
    
                let x = up_left.0 + (pixel_x as f64 * step_x);
                let y = up_left.1 - (pixel_y as f64 * step_y);
    
                let aux = mandelbrot::ComplexNumber { real : x, img : y};
    
                let degree:u8 = mandelbrot::mandebrot_set_degree(aux, 255, 4) as u8;
    
                let color = [0xff - degree, 0xff - degree, 0xff - degree, 0xff];
                stored_degree = degree;
                
                pixel.copy_from_slice(&color);

            }else{
                let color = [0xff - stored_degree, 0xff - stored_degree, 0xff - stored_degree, 0xff];
                pixel.copy_from_slice(&color);
            }

            


        }
    }
}
