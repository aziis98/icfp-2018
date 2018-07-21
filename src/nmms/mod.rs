
use std::ops::Add;

#[derive(Debug, Copy, Clone)]
pub struct Coordinate {
	pub x: u8,
	pub y: u8,
	pub z: u8
}

#[derive(Debug, Copy, Clone)]
pub struct CoordinateDifference {
	pub dx: i16,
	pub dy: i16,
	pub dz: i16
}

impl Add<CoordinateDifference> for Coordinate {
	type Output = Coordinate;

	fn add(self, d: CoordinateDifference) -> Coordinate {
		Coordinate {
			x: (self.x as i16 + d.dx) as u8,
			y: (self.y as i16 + d.dy) as u8,
			z: (self.z as i16 + d.dz) as u8,
		}
	}
}
