fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

pub fn greyscale(iterations: f64, max_iterations: usize) -> u32 {
	let value = ((iterations / max_iterations as f64) * 255.0) as u8;
	from_u8_rgb(value, value, value)
}