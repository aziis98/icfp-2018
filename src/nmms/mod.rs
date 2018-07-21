use std::ops::*;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub struct Coordinate {
	pub x: u8,
	pub y: u8,
	pub z: u8,
}

#[derive(Debug, Copy, Clone)]
pub struct CoordinateDifference {
	pub dx: i16,
	pub dy: i16,
	pub dz: i16,
}

#[derive(Debug, Copy, Clone)]
pub struct Region {
	pub c1: Coordinate,
	pub c2: Coordinate,
}

impl Add<CoordinateDifference> for Coordinate {
	// adds coordinate and a difference
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
	// subtracts 2 coordinates
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
		Coordinate { x, y, z }
	}

	pub fn is_adjacent(self, other: Coordinate) -> bool {
		(self - other).mlen() == 1
	}
}

impl Add<[i16; 3]> for Coordinate {
	type Output = Coordinate;

	fn add(self, vec3: [i16; 3]) -> Coordinate {
		Coordinate::new(
			(self.x as i16 + vec3[0]) as u8,
			(self.y as i16 + vec3[1]) as u8,
			(self.z as i16 + vec3[2]) as u8,
		)
	}
}

impl CoordinateDifference {
	pub fn mlen(&self) -> u16 {
		(self.dx.abs() + self.dz.abs() + self.dz.abs()) as u16
	}

	pub fn clen(&self) -> u16 {
		self.dx.abs().max(self.dy.abs()).max(self.dz.abs()) as u16
	}

	pub fn is_ld(&self) -> bool {
		if self.dx == 0 && self.dy == 0 && self.dz != 0 {
			return true;
		} else if self.dx == 0 && self.dy != 0 && self.dz == 0 {
			return true;
		} else if self.dx != 0 && self.dy == 0 && self.dz == 0 {
			return true;
		} else {
			false
		}
	}

	pub fn is_sld(&self) -> bool {
		if self.is_ld() {
			if self.clen() < 5 {
				return true;
			} else {
				return false;
			}
		} else {
			false
		}
	}

	pub fn is_lld(&self) -> bool {
		if self.is_ld() {
			if self.clen() < 15 {
				return true;
			} else {
				return false;
			}
		} else {
			false
		}
	}

	pub fn is_nd(&self) -> bool {
		if self.mlen() <= 2 && self.clen() == 1 {
			return true;
		} else {
			return false;
		}
	}
}

impl Region {
	pub fn new(c1: Coordinate, c2: Coordinate) -> Region {
		let min_c = Coordinate::new(c1.x.min(c2.x), c1.y.min(c2.y), c1.z.min(c2.z));
		let max_c = Coordinate::new(c1.x.max(c2.x), c1.y.max(c2.y), c1.z.max(c2.z));

		Region {
			c1: min_c,
			c2: max_c,
		}
	}

	pub fn is_member(self, c: Coordinate) -> bool {
		(self.c1.x <= c.x && c.x <= self.c2.x)
			&& (self.c1.y <= c.y && c.y <= self.c2.y)
			&& (self.c1.z <= c.z && c.z <= self.c2.z)
	}

	pub fn dimensions(self) -> u8 {
		(if self.c1.x == self.c2.x { 0 } else { 1 })
			+ (if self.c1.y == self.c2.y { 0 } else { 1 })
			+ (if self.c1.z == self.c2.z { 0 } else { 1 })
	}
}

#[derive(Debug, Clone)]
pub struct Matrix {
	pub resolution: u8,
	pub voxels: Vec<u8>
}

impl Matrix {

	pub fn from_file(path: &str) -> Matrix {
		use std::fs;

		let mut data = fs::read(path).expect("Unable to read file");

		let resolution = data.remove(0);
		let voxels = data;

		Matrix {
			resolution,
			voxels
		}
	}

	pub fn new(resolution: u8) -> Matrix {
		let byte_count = (resolution as u32).pow(3) / 8;

		Matrix {
			resolution,
			voxels: vec![0; byte_count as usize]
		}
	}

	pub fn get_voxel(&self, c: Coordinate) -> bool {
		let res = self.resolution as u32;
		let index = (c.x as u32) * res * res + (c.y as u32) * res + (c.z as u32);

		let byte_index = index / 8;
		let bit_index = (index % 8) as u8;

		let voxel_word = self.voxels[byte_index as usize];

		(voxel_word >> bit_index) & 1 != 0
	}

	pub fn count_filled(&self) -> u32 {
		let mut acc = 0u32;

		for mut voxel_byte in &self.voxels {
			acc += voxel_byte.count_ones();
		}

		acc
	}

	/// Questa funzione se un voxel è _grounded_, al massimo questa operazione
	/// richiede {numero di voxel pieni nel modello} passi o al meglio `y` se è direttamente in linea con
	/// il terreno.
	pub fn is_grounded(&self, c: Coordinate) -> bool {

		use std::collections::HashSet;

		let mut tail: HashSet<Coordinate> = HashSet::new();

		let mut is_grounded_tail = |c: Coordinate| -> bool {
			if tail.contains(&c) {
				false
			}
			else {
				tail.insert(c);

				c.y == 0 || (
					is_grounded_tail(c + [0, 0, 0])
				)
			}
		};

        is_grounded_tail(c)
    }

    pub fn is_empty(&self, r: Region) -> bool {
        unimplemented!();
    }
}