
use std::ops::*;

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

#[derive(Debug, Copy, Clone)]
pub struct Region {
	pub c1: Coordinate,
	pub c2: Coordinate
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

impl Sub for Coordinate {
	type Output = CoordinateDifference;

	fn sub(self, other: Coordinate) -> CoordinateDifference {
		CoordinateDifference {
			dx: self.x as i16 - other.x as i16,
			dy: self.y as i16 - other.y as i16,
			dz: self.z as i16 - other.z as i16,
		}
	}
}

impl Coordinate {
	
	pub fn new(x: u8, y: u8, z: u8) -> Coordinate {
		Coordinate { 
			x, y, z
		}
	}

	pub fn is_adjacent(self, other: Coordinate) -> bool {
		(self - other).mlen() == 1
	}
	
}

impl CoordinateDifference {
	pub fn mlen(&self) -> u16 {
		(self.dx.abs() + self.dz.abs() + self.dz.abs()) as u16
	}

	pub fn clen(&self) -> u16 {
		self.dx.abs().max(self.dy.abs()).max(self.dz.abs()) as u16
	}
}

impl Region {
	pub fn new(c1: Coordinate, c2: Coordinate) -> Region {
		let min_c = Coordinate::new(
			c1.x.min(c2.x),
			c1.y.min(c2.y),
			c1.z.min(c2.z)
		);
		let max_c = Coordinate::new(
			c1.x.max(c2.x),
			c1.y.max(c2.y),
			c1.z.max(c2.z)
		);

		Region {
			c1: min_c,
			c2: max_c
		}
	}

	pub fn is_member(self, c: Coordinate) -> bool {
		(self.c1.x <= c.x && c.x <= self.c2.x) &&
			(self.c1.y <= c.y && c.y <= self.c2.y) &&
			(self.c1.z <= c.z && c.z <= self.c2.z)
	}

	pub fn dimensions(self) -> u8 {
		(if self.c1.x == self.c2.x { 0 } else { 1 }) 
			+ (if self.c1.y == self.c2.y { 0 } else { 1 })
			+ (if self.c1.z == self.c2.z { 0 } else { 1 })
	}
}