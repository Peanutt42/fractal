use minifb::{Key, Window, WindowOptions};

pub mod colorizers;
use colorizers::*;

pub mod fractals;
use fractals::*;

pub mod visualizer;
use visualizer::Visualizer;

fn main() {
	if cfg!(debug_assertions) {
		println!("Running in debug build will be much slower as in a release build!");
	}

	let mut visualizer = Visualizer::new(800, 800, 100);

	let mut window = Window::new(
		"Fractal",
		visualizer.width, visualizer.height,
		WindowOptions {
			resize: true,
			..WindowOptions::default()
		})
		.unwrap_or_else(|e| { panic!("Failed to create window: {e}"); });

	let mut fractal: fn(f64, f64, usize) -> f64 = mandelbrot;
	let mut colorizer: fn(f64, usize) -> u32 = greyscale;

	let mut last_mouse_pos = window.get_mouse_pos(minifb::MouseMode::Clamp).unwrap();

	while window.is_open() && !window.is_key_down(Key::Escape) {
		let (new_width, new_height) = window.get_size();
		visualizer.resize(new_width, new_height);

		let new_mouse_pos = window.get_mouse_pos(minifb::MouseMode::Clamp).unwrap();
		if window.get_mouse_down(minifb::MouseButton::Left) {
			visualizer.pan((new_mouse_pos.0 - last_mouse_pos.0) as f64, (new_mouse_pos.1 - last_mouse_pos.1) as f64);
		}
		last_mouse_pos = new_mouse_pos;

		let (_, mouse_wheel) = window.get_scroll_wheel().unwrap_or((0.0, 0.0));
		if mouse_wheel != 0.0 {															// mouse_wheel on a physical wheel is +-12
			visualizer.zoom(new_mouse_pos.0 as f64, new_mouse_pos.1 as f64, mouse_wheel as f64 / 12.0);
		}

		window.get_keys_pressed(minifb::KeyRepeat::No).iter().for_each(|key| {
			match key {
				// fractals
				Key::Key1 => fractal = mandelbrot,
				Key::Key2 => fractal = julia,
				Key::Key3 => fractal = burning_ship,
				Key::Key4 => fractal = tricorn,
				// colorizers
				Key::Key7 => colorizer = greyscale,
				Key::Key8 => colorizer = blue,
				Key::Key9 => colorizer = colorful,
				// others
				Key::A => visualizer.max_iterations += 50,
				Key::D => visualizer.max_iterations -= 50,
				_ => {}
			}
		});
		
		visualizer.update(fractal, colorizer);		

		window.update_with_buffer(&visualizer.buffer, visualizer.width, visualizer.height)
			.unwrap();
	}
}