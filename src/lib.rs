use std::{error::Error, fmt::Display};
use nalgebra::{Matrix6, Matrix6x1};

pub struct Hail(Vec<HailStone>);

impl Hail {
	pub fn new_from_str(content: &str) -> Self {
		Self(content.lines().enumerate().map(|(_, line)| HailStone::new_from_str(line)).collect())
	}

	pub fn determine_position(&self) -> i64 {
		let equations_count = 4;
		let mut window_iter = self.0.windows(equations_count);

		loop {
			if let Some(hailstones) = window_iter.next() {
				match self.determine_position_for_using_hailstones(hailstones, equations_count) {
					Ok(hs) => {
						break hs.get_coordinates_sum();
					},
					Err(e) => println!("{e}"),
				}
			}
			else {
				eprintln!("No solution.");
			}
		}
	}

	fn determine_position_for_using_hailstones(&self, hailstones: &[HailStone], equations: usize) -> Result<HailStone, InconsistentSystem>{
		if hailstones.len() == equations {
			// V1 = velocities[0]
			// V2 = velocities[1]
			// V3 = velocities[2]
			// V4 = velocities[3]
			// P1 = positions[0]
			// P2 = positions[1]
			// P3 = positions[2]
			// P4 = positions[3]
			let (velocities, positions): (Vec<_>, Vec<_>) = hailstones.iter().map(|hailstone| (Self::get_f64_tuple(hailstone.velocity), Self::get_f64_tuple(hailstone.position))).unzip();
			// V21 = rel_velocities[0]
			// V32 = rel_velocities[1]
			// V43 = rel_velocities[2]
			// V54 = rel_velocities[3]
			let rel_velocities = velocities.windows(2).map(|v| Self::get_f64_tuple_difference(*v.get(1).unwrap(), *v.first().unwrap())).collect::<Vec<(f64, f64, f64)>>();
			// P21 = rel_positions[0]
			// P32 = rel_positions[1]
			// P43 = rel_positions[2]
			// P54 = rel_positions[3]
			let rel_positions = positions.windows(2).map(|p| Self::get_f64_tuple_difference(*p.get(1).unwrap(), *p.first().unwrap())).collect::<Vec<(f64, f64, f64)>>();

			let coefficient_matrix = Matrix6::new(rel_velocities[0].1, -rel_velocities[0].0, 0f64, -rel_positions[0].1, rel_positions[0].0, 0f64, rel_velocities[1].1, -rel_velocities[1].0, 0f64, -rel_positions[1].1, rel_positions[1].0, 0f64, rel_velocities[2].1, -rel_velocities[2].0, 0f64, -rel_positions[2].1, rel_positions[2].0, 0f64, 0f64, rel_velocities[0].2, -rel_velocities[0].1, 0f64, -rel_positions[0].2, rel_positions[0].1, 0f64, rel_velocities[1].2, -rel_velocities[1].1, 0f64, -rel_positions[1].2, rel_positions[1].1, 0f64, rel_velocities[2].2, -rel_velocities[2].1, 0f64, -rel_positions[2].2, rel_positions[2].1);
			match coefficient_matrix.try_inverse() {
				Some(inv) => {
					let constant_matrix = Matrix6x1::new(positions[0].1 * velocities[0].0 - positions[1].1 * velocities[1].0 - positions[0].0 * velocities[0].1 + positions[1].0 * velocities[1].1, positions[1].1 * velocities[1].0 - positions[2].1 * velocities[2].0 - positions[1].0 * velocities[1].1 + positions[2].0 * velocities[2].1, positions[2].1 * velocities[2].0 - positions[3].1 * velocities[3].0 - positions[2].0 * velocities[2].1 + positions[3].0 * velocities[3].1, positions[0].2 * velocities[0].1 - positions[1].2 * velocities[1].1 - positions[0].1 * velocities[0].2 + positions[1].1 * velocities[1].2, positions[1].2 * velocities[1].1 - positions[2].2 * velocities[2].1 - positions[1].1 * velocities[1].2 + positions[2].1 * velocities[2].2, positions[2].2 * velocities[2].1 - positions[3].2 * velocities[3].1 - positions[2].1 * velocities[2].2 + positions[3].1 * velocities[3].2);
					let variable_matrix = inv * constant_matrix;
					Ok(HailStone { position: Self::get_i64_tuple((variable_matrix.x, variable_matrix.y, variable_matrix.z)), velocity: Self::get_i64_tuple((variable_matrix.w, variable_matrix.a, variable_matrix.b)) })
				},
				None => Err(InconsistentSystem),
			}
		}
		else {
			Err(InconsistentSystem)
		}
	}

	fn get_i64_tuple(t: (f64, f64, f64)) -> (i64, i64, i64) {
		(t.0.round() as i64, t.1.round() as i64, t.2.round() as i64)
	}

	fn get_f64_tuple_difference(a: (f64, f64, f64), b: (f64, f64, f64)) -> (f64, f64, f64) {
		// a - b
		(a.0 - b.0, a.1 - b.1, a.2 - b.2)
	}

	fn get_f64_tuple(t: (i64, i64, i64)) -> (f64, f64, f64) {
		(t.0 as f64, t.1 as f64, t.2 as f64)
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

struct HailStone {
    position: (i64, i64, i64),
    velocity: (i64, i64, i64),
}

impl HailStone {
	fn new_from_str(hailstone: &str) -> Self {
		let hailstone = hailstone.replace("  ", " ").replace(" @", ",");
		let mut data_nums = hailstone.split(", ").map(|coor| coor.parse::<i64>().unwrap_or_else(|_| panic!("unable to parse coordinate: {coor}")));
		Self {
			position: (data_nums.next().unwrap_or_else(|| panic!("error parsing position input x for line {hailstone}")), data_nums.next().unwrap_or_else(|| panic!("error parsing position input y for line {hailstone}")), data_nums.next().unwrap_or_else(|| panic!("error parsing position input z for line {hailstone}"))),
			velocity: (data_nums.next().unwrap_or_else(|| panic!("error parsing velocity input x for line {hailstone}")), data_nums.next().unwrap_or_else(|| panic!("error parsing velocity input x for line {hailstone}")), data_nums.next().unwrap_or_else(|| panic!("error parsing velocity input x for line {hailstone}"))),
		}
	}

	fn get_coordinates_sum(&self) -> i64 {
		self.position.0 + self.position.1 + self.position.2
	}
}