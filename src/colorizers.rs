fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | b as u32
}

pub fn greyscale(iterations: f64, max_iterations: usize) -> u32 {
	let value = ((iterations / max_iterations as f64) * 255.0) as u8;
	from_u8_rgb(value, value, value)
}

pub fn blue(iterations: f64, max_iterations: usize) -> u32 {
	let value = iterations / max_iterations as f64;
	from_u8_rgb((value * 65.0) as u8, (value * 90.0) as u8, (value * 160.0) as u8)
}

pub fn colorful(iterations: f64, max_iterations: usize) -> u32 {
	let hue = (iterations * 10.0) % 360.0;
	let lightness = 0.5 + f64::log(1.0 + iterations, 1.0 + max_iterations as f64);
	from_hsl(hue, 1.0, lightness)
}


fn hue_to_rgb(mut t: f64, p: f64, q: f64) -> f64 {
	if t < 0.0 { t += 1.0; }
	if t > 1.0 { t -= 1.0; }
	if t < 1.0 / 6.0 { return p + (q - p) * 6.0 * t; }
	if t < 0.5 { return q; }
	if t < 2.0 / 3.0 { return p + (q - p) * (2.0 / 3.0 - t) * 6.0; }
	p
}
fn from_hsl(mut hue: f64, mut saturation: f64, mut lightness: f64) -> u32 {
	// Ensure hue is in the range [0, 360]
	hue = hue % 360.0;
	if hue < 0.0 {
		hue += 360.0;
	}

	// Ensure saturation and lightness are in the range [0, 1]
	saturation = f64::clamp(saturation, 0.0, 1.0);
	lightness = f64::clamp(lightness, 0.0, 1.0);

	// If the saturation is close to zero, the color is a shade of gray
	if saturation < 0.00001 {
		let gray_value = (lightness * 255.0) as u8;
		return from_u8_rgb(gray_value, gray_value, gray_value);
	}

	let q = if lightness < 0.5 { lightness * (1.0 + saturation) } else { lightness + saturation - lightness * saturation };
	let p = 2.0 * lightness - q;

	let hk = hue / 360.0;

	// Convert HSL to RGB using the specified formula
	let tr = hk + 1.0 / 3.0;
	let tg = hk;
	let tb = hk - 1.0 / 3.0;

	from_u8_rgb((hue_to_rgb(tr, p, q) * 255.0) as u8, (hue_to_rgb(tg, p, q) * 255.0) as u8, (hue_to_rgb(tb, p, q) * 255.0) as u8)
}