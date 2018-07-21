
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

impl CoordinateDifference {
	pub fn mlen(&self) -> u16 {
		(self.dx.abs() + self.dz.abs() + self.dz.abs()) as u16
	}

	pub fn clen(&self) -> u16 {
		self.dx.abs().max(self.dy.abs()).max(self.dz.abs()) as u16
	}
}

impl CoordinateDifference {
    pub fn is_ld(&self) -> bool {
        if self.dx == 0 && self.dy == 0 && self.dz != 0 {
            return true;
        }
        else if self.dx == 0 && self.dy != 0 && self.dz == 0 {
            return true
        }
        else if self.dx != 0 && self.dy == 0 && self.dz == 0 {
            return true
        }
        else { false }
    }

    pub fn is_sld(&self) -> bool {
        if self.is_ld() {
            if self.clen() < 5 { return true }
            else { return false }
        }
        else { false }
    }

    pub fn is_lld(&self) -> bool {
        if self.is_ld() {
            if self.clen() < 15 && self.clen() > 5 { return true }
                else { return false }
        }
            else { false }
    }
}

impl Sub for Coordinate {
	type Output = CoordinateDifference;

	fn sub(self, other: Coordinate) -> CoordinateDifference {
		CoordinateDifference {
			dx: other.x as i16 - self.x as i16,
			dy: other.y as i16 - self.y as i16,
			dz: other.z as i16 - self.z as i16,
		}
	}
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
