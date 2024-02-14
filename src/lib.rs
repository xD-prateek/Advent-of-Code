use std::{error::Error, fmt::Display};
use nalgebra::{Matrix1x6, Matrix6, Matrix6x1, Point2, Point3, Vector3};

#[derive(Debug)]
pub struct Hail(Vec<HailStone>);

impl Hail {
	pub fn new_from_str(content: &str) -> Self {
		Self(content.lines().enumerate().map(|(_, line)| HailStone::new_from_str(line)).collect())
	}

	pub fn determine_position(&self) -> f64 {
		// 19, 13, 30 @ -2, 1, -2
		// 18, 19, 22 @ -1, -1, -2
		// 20, 25, 34 @ -2, -2, -4
		// 12, 31, 28 @ -1, -2, -1
		// 20, 19, 15 @ 1, -5, -3


		// calculate x, y, z, vx, vy, vz for 3 Hailstones
		let mut option_rock: Option<HailStone> = None;
		self.0.windows(3).fold(true, |mut acc, hailstones| {
			if let Ok(potential_rock) = self.determine_position_for_using_hailstones(hailstones) {
				println!("potential_rock: {potential_rock:?}");
				match &mut option_rock {
					None => option_rock = Some(potential_rock),
					Some(rock) => acc = acc && potential_rock == *rock,
				}
			}
			else {
				println!("error!");
			}
			acc
		});

		match option_rock {
			None => f64::default(),
			Some(rock) => rock.get_coordinates_sum(),
		}
	}

	fn determine_position_for_using_hailstones(&self, hailstones: &[HailStone]) -> Result<HailStone, InconsistentSystem>{
		if hailstones.len() == 3 {
			let v1 = hailstones.get(0).unwrap_or_else(|| panic!("error getting vector 1.")).velocity;
			let v2 = hailstones.get(1).unwrap_or_else(|| panic!("error getting vector 2.")).velocity;
			let v3 = hailstones.get(2).unwrap_or_else(|| panic!("error getting vector 3.")).velocity;
			// relative vel
			let v_21 = v2 - v1;
			let v_32 = v3 - v2;
			let v_13 = v1 - v3;

			let p1 = hailstones.get(0).unwrap_or_else(|| panic!("error getting point 1.")).position;
			let p2 = hailstones.get(1).unwrap_or_else(|| panic!("error getting point 2.")).position;
			let p3 = hailstones.get(2).unwrap_or_else(|| panic!("error getting point 3.")).position;
			// relative pos
			let p_21 = p2 - p1;
			let p_32 = p3 - p2;
			let p_13 = p1 - p3;

			let coefficient_matrix = Matrix6::new(v_21.y, -v_21.x, 0f64, -p_21.y, p_21.x, 0f64, v_32.y, -v_32.x, 0f64, -p_32.y, p_32.x, 0f64, v_13.y, -v_13.x, 0f64, -p_13.y, p_13.x, 0f64, 0f64, v_21.z, -v_21.y, 0f64, -p_21.z, -p_21.y, 0f64, v_32.z, -v_32.y, 0f64, -p_32.z, p_32.y, 0f64, v_13.z, -v_13.y, 0f64, -p_13.z, p_13.y);
			let constant_matrix = Matrix6x1::new(p1.y * v1.x + p2.x * v2.y - p2.y * v2.x - p1.x * v1.y, p2.y * v2.x + p3.x * v3.y - p3.y * v3.x - p2.x * v2.y, p3.y * v3.x + p1.x * v1.y - p1.y * v1.x - p3.x * v3.y, p1.z * v1.y + p2.y * v2.z - p2.z * v2.y - p1.y * v1.z, p2.z * v2.y + p3.y * v3.z - p3.z * v3.y - p2.y * v2.z, p3.z * v3.y + p1.y * v1.z - p1.z * v1.y - p3.y * v3.z);
			// println!("coefficient_matrix: {coefficient_matrix}");
			// println!("constant_matrix: {constant_matrix}");
			if let Some(inv) = coefficient_matrix.try_inverse() {
				// println!("inv: {inv}");
				let variable_matrix = inv * constant_matrix;
				Ok(HailStone { position: Point3::new(variable_matrix.x, variable_matrix.y, variable_matrix.z), velocity: Vector3::new(variable_matrix.w, variable_matrix.a, variable_matrix.b) })
			}
			else {
				Err(InconsistentSystem)
			}
		}
		else {
			Err(InconsistentSystem)
		}
	}
}

#[derive(Debug)]
struct InconsistentSystem;

impl Error for InconsistentSystem {}

impl Display for InconsistentSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    	write!(f, "hailstones create inconsistent system.")
    }
}

#[derive(Debug)]
struct HailStone {
    position: Point3<f64>,
    velocity: Vector3<f64>,
}

impl PartialEq for HailStone {
    fn eq(&self, other: &Self) -> bool {
    	self.position.eq(&other.position) && self.velocity.eq(&other.velocity)
    }
}

impl HailStone {
	fn new_from_str(hailstone: &str) -> Self {
		let hailstone = hailstone.replace(" @", ",");
		let mut data_nums = hailstone.split(", ").map(|coor| coor.parse::<f64>().unwrap_or_else(|_| panic!("unable to parse coordinate: {coor}")));
		Self {
			position: Point3::new(data_nums.next().unwrap_or_else(|| panic!("error parsing position input x for line {hailstone}")), data_nums.next().unwrap_or_else(|| panic!("error parsing position input y for line {hailstone}")), data_nums.next().unwrap_or_else(|| panic!("error parsing position input z for line {hailstone}"))),
			velocity: Vector3::new(data_nums.next().unwrap_or_else(|| panic!("error parsing velocity input x for line {hailstone}")), data_nums.next().unwrap_or_else(|| panic!("error parsing velocity input x for line {hailstone}")), data_nums.next().unwrap_or_else(|| panic!("error parsing velocity input x for line {hailstone}"))),
		}
	}

	fn get_coordinates_sum(&self) -> f64 {
		self.position.coords.iter().sum()
	}
}