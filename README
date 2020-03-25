![Rusty Mandelbrot](https://static.wixstatic.com/media/7afe00_7daf6ef888774e2d9f39c39235b55e7f~mv2.jpg "Rusty Mandelbrot")

# Rusty Mandelbrot
<p align="left">
<img alt="GitHub tag (latest by date)" src="https://img.shields.io/github/v/tag/bernardocrodrigues/rusty_mandelbrot">
<img alt="CircleCI" src="https://img.shields.io/circleci/build/github/bernardocrodrigues/rusty_mandelbrot?token=70cb80ab47897f052a0bf0dc9eb09d4f2bb8e442">
<img alt="GitHub" src="https://img.shields.io/github/license/bernardocrodrigues/rusty_mandelbrot">
</p>

A Mandelbrot fractal interactive viewer fully implemented in Rust. Supports multi-thread CPU computing and GPU rendering through [pixels](https://crates.io/crates/pixels) , [scoped_threadpool](https://crates.io/crates/scoped_threadpool) and [winit](https://crates.io/crates/winit).

![rusty](https://static.wixstatic.com/media/7afe00_b9eaf070285f4f879c175985d067e1d9~mv2.gif)

## Dependencies

- Rust 1.42.0

## Run it

1. Clone it.

```
git clone git@github.com:bernardocrodrigues/rusty_mandelbrot.git
```

2. Build and run it.

```
cargo run
```

## Changing some parameters

In `main.rs`, you'll find some hard-coded that can be changed to better performance or alter functionality. 

- **width_** & **height_**: Window resolution.
- **iterations**: How many iterations a point will be ran until considered inside Mandelbrot set.
- **zoom_factor**:  Determines the size of zoom steps.
- **low_res_scale**: Level of downscaling applied when user is panning.
