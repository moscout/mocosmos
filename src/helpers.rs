use rapier2d::na::Point2;

use crate::components::*;

const SCALING_FACTOR: f32 = 0.1;

pub fn pixels_to_meters(pixels: f32) -> f32 {
	pixels * SCALING_FACTOR
}

pub fn meters_to_pixels(meters: f32) -> f32 {
	meters / SCALING_FACTOR
}

pub fn rotate_point(point: &Point2<f32>, cx: f32, cy: f32, angle: f32) -> Point2<f32> {
	Point2::new(
		angle.cos() * (point.x - cx) - angle.sin() * (point.y - cy) + cx,
		angle.sin() * (point.x - cx) + angle.cos() * (point.y - cy) + cy)
}

pub fn is_outside_of_rect(position: &Position, size: &Size, rect: &Rectangle) -> bool {
	position.x + size.width < rect.x  ||
	position.x > rect.x + rect.width  ||
	position.y + size.height < rect.y ||
	position.y > rect.y + rect.height
}
