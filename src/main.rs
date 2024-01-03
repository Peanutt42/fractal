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

	let mut last_mouse_pos = window.get_mouse_pos(minifb::MouseMode::Clamp).unwrap();

	while window.is_open() && !window.is_key_down(Key::Escape) {
		let new_size = window.get_size();
		visualizer.resize(new_size.0, new_size.1);

		let new_mouse_pos = window.get_mouse_pos(minifb::MouseMode::Clamp).unwrap();
		if window.get_mouse_down(minifb::MouseButton::Left) {
			visualizer.pan((new_mouse_pos.0 - last_mouse_pos.0) as f64, (new_mouse_pos.1 - last_mouse_pos.1) as f64);
		}
		last_mouse_pos = new_mouse_pos;

		let mouse_wheel = window.get_scroll_wheel().unwrap_or((0.0, 0.0)).1;
		if mouse_wheel != 0.0 {
			visualizer.zoom(new_mouse_pos.0 as usize, new_mouse_pos.1 as usize, mouse_wheel as f64);
		}
		println!("{mouse_wheel}");

		visualizer.update(julia, greyscale);		

		window.update_with_buffer(&visualizer.buffer, visualizer.width, visualizer.height)
			.unwrap();
	}
}