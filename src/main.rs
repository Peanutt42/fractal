use minifb::{Key, Window, WindowOptions};

fn main() {
	let mut width: usize = 800;
	let mut height: usize = 800;

	let mut window = Window::new(
		"Fractal",
		width, height,
		WindowOptions {
			resize: true,
			..WindowOptions::default()
		})
		.unwrap_or_else(|e| { panic!("Failed to create window: {e}"); });

	let mut buffer: Vec<u32> = Vec::new();
	buffer.resize(width * height, 0u32);

	while window.is_open() && !window.is_key_down(Key::Escape) {
		let new_size = window.get_size();
		if width != new_size.0 || height != new_size.1 {
			width = new_size.0;
			height = new_size.1;
			buffer.resize(width * height, 0u32);
		}

		generate_image(&mut buffer, width, height);

		window.update_with_buffer(&buffer, width, height)
			.unwrap();
	}
}

fn generate_image(buffer: &mut Vec<u32>, width: usize, height: usize) {
	assert!(buffer.len() == width * height);

	for y in 0..height {
		for x in 0..width {
			let index = y * width + x;
			buffer[index] = 0u32; // black
		}
	}
}