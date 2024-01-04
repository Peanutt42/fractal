use rayon::prelude::*;

pub struct Visualizer {
	pub buffer: Vec<u32>,
	pub width: usize,
	pub height: usize,
	pub max_iterations: usize,
	pub offset: (f64, f64),
	pub zoom: f64,
}

impl Visualizer {
	pub fn new(width: usize, height: usize, max_iterations: usize) -> Self {
		Self {
			buffer: vec![0u32; width * height],
			width,
			height,
			max_iterations,
			offset: (0.0, 0.0),
			zoom: 500.0
		}
	}

	pub fn resize(&mut self, width: usize, height: usize) {
		if self.width == width && self.height == height {
			return;
		}

		self.width = width;
		self.height = height;
		self.buffer.resize(width * height, 0u32);
	}

	pub fn zoom(&mut self, x: f64, y: f64, amount: f64) {
		let factor = 1.0 + (amount * 0.1);
		self.offset.0 -= (x - (self.width as f64 / 2.0)) / self.zoom * ((1.0 / factor) - 1.0);
		self.offset.1 -= (y - (self.height as f64 / 2.0)) / self.zoom * ((1.0 / factor) - 1.0);
		self.zoom *= factor;
	}

	pub fn pan(&mut self, delta_x: f64, delta_y: f64) {
		self.offset.0 -= delta_x / self.zoom;
		self.offset.1 -= delta_y / self.zoom;
	}

	pub fn update(&mut self, fractal: fn(x: f64, y: f64, max_iterations: usize) -> f64, colorizer: fn(v: f64, max_iterations: usize) -> u32) {
		assert!(self.buffer.len() == self.width * self.height);

		self.buffer
			.par_chunks_exact_mut(self.width)
			.enumerate()
			.for_each(|(y, row)| {
				for x in 0..self.width {	
					let new_x = ((x as f64 - self.width as f64 / 2.0) / self.zoom) + self.offset.0;
					let new_y = ((y as f64 - self.height as f64 / 2.0) / self.zoom) + self.offset.1;
	
					let result = fractal(new_x, new_y, self.max_iterations);
					if result as usize == self.max_iterations {
						row[x] = 0xFFFFFFFF;
					}
					else {
						row[x] = colorizer(f64::abs(result), self.max_iterations);
					}
				}
			});
	}
}