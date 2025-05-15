const SCALING_FACTOR: f32 = 0.1;

pub fn pixels_to_meters(pixels: f32) -> f32 {
	pixels * SCALING_FACTOR
}

pub fn meters_to_pixels(meters: f32) -> f32 {
	meters / SCALING_FACTOR
}
