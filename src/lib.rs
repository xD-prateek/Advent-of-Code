pub struct Hail{
	fall: Vec<HailStone>,
	limit_start: f64,
	limit_end: f64,
}

impl Hail {
	pub fn new_from_str(content: &str, limit_start: usize, limit_end: usize) -> Self {
		Self {
			fall: content.lines().enumerate().map(|(_, line)| HailStone::new_from_str(line)).collect(),
			limit_start: limit_start as f64,
			limit_end: limit_end as f64,
		}
	}

	pub fn determine_collisions(&self) -> usize {
		self.fall.iter().enumerate().fold(0, |acc, (i, hailstone_a)| {
			self.fall.iter().skip(i + 1).filter(|hailstone_b| hailstone_a.velocity.0 * hailstone_b.velocity.1 != hailstone_a.velocity.1 * hailstone_b.velocity.0).fold(0, |acc, hailstone_b| {
				match self.path_intersects(hailstone_a, hailstone_b) {
					true => acc + 1,
					false => acc,
				}
			}) + acc
		})
	}

	fn path_intersects(&self, a: &HailStone, b: &HailStone) -> bool {
		let denom = b.velocity.0 * a.velocity.1 - a.velocity.0 * b.velocity.1;
		let x = (a.position.0 * b.velocity.0 * a.velocity.1 - b.position.0 * a.velocity.0 * b.velocity.1 + (b.position.1 - a.position.1) * a.velocity.0 * b.velocity.0) / denom;
		let y = ((a.position.0 - b.position.0) * b.velocity.1 * a.velocity.1 + b.position.1 * a.velocity.1 * b.velocity.0 - a.position.1 * b.velocity.1 * a.velocity.0) / denom;
		[a, b].into_iter().all(|HailStone { position: (sx, sy), velocity: (vx, vy) }| ((x - sx) * vx).is_sign_positive() && ((y - sy) * vy).is_sign_positive()) && x >= self.limit_start && x <= self.limit_end && y >= self.limit_start && y <= self.limit_end
	}
}

struct HailStone {
    position: (f64, f64),
    velocity: (f64, f64),
}

impl HailStone {
	fn new_from_str(hailstone: &str) -> Self {
		let (start_str, velocity_str) = hailstone.split_once(" @ ").unwrap_or_else(|| panic!("unable to read hailstone: {hailstone}"));
		let mut start_nums = start_str.split(", ").take(2).map(|coor| coor.parse::<f64>().unwrap_or_else(|_| panic!("unable to parse coordinate: {coor}")));
		let mut velocity_nums = velocity_str.split(", ").take(2).map(|val| val.parse::<f64>().unwrap_or_else(|_| panic!("unable to parse velocity: {val}")));
		Self {
			position: (start_nums.next().unwrap(), start_nums.next().unwrap()),
			velocity: (velocity_nums.next().unwrap(), velocity_nums.next().unwrap()),
		}
	}
}