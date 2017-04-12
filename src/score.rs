use std::f32;

use difficulty::HitAccuracy;

impl HitAccuracy {
	fn bonus_value(&self) -> u32 {
		match *self {
			HitAccuracy::ThreeHundredMax => 32,
			HitAccuracy::ThreeHundred => 32,
			HitAccuracy::TwoHundred => 16,
			HitAccuracy::OneHundred => 8,
			HitAccuracy::Fifty => 4,
			HitAccuracy::Miss => 0,
		}
	}

	fn bonus(&self) -> f32 {
		match *self {
			HitAccuracy::ThreeHundredMax => 2.0,
			HitAccuracy::ThreeHundred => 1.0,
			HitAccuracy::TwoHundred => -8.0,
			HitAccuracy::OneHundred => -24.0,
			HitAccuracy::Fifty => -44.0,
			HitAccuracy::Miss => -f32::INFINITY,
		}
	}
}

pub struct Score {
	mod_multiplier: f32,
	mod_diviser: f32,
	hit_count: u32,
	hit_points: u32,
	bonus_value: u32,
	bonus: f32,
	combo: u32,
	max_combo: u32,
}

const MAX_SCORE: u32 = 1_000_000;

impl Score {
	pub fn new() -> Score {
		Score{
			mod_multiplier: 1.0,
			mod_diviser: 1.0,
			hit_count: 0,
			hit_points: 0,
			bonus_value: 0,
			bonus: 100.0,
			combo: 0,
			max_combo: 0,
		}
	}

	pub fn push(&mut self, hit: HitAccuracy) {
		self.hit_count += 1;
		self.hit_points += hit.score() as u32;

		self.bonus_value += hit.bonus_value();
		self.bonus += hit.bonus() / self.mod_diviser;
		if self.bonus < 0.0 {
			self.bonus = 0.0;
		}
		if self.bonus > 100.0 {
			self.bonus = 100.0;
		}

		if hit != HitAccuracy::Miss {
			self.combo += 1;
			if self.combo > self.max_combo {
				self.max_combo = self.combo;
			}
		} else {
			self.combo = 0;
		}
	}

	pub fn accuracy(&self) -> f32 {
		if self.hit_count == 0 {
			return 0.0;
		}

		self.hit_points as f32 / (self.hit_count * 300) as f32
	}

	pub fn score(&self) -> u32 {
		if self.hit_count == 0 {
			return 0;
		}

		let a = (MAX_SCORE as f32) * self.mod_multiplier * 0.5 / (self.hit_count as f32);
		let base_score = a * (self.hit_points as f32 / 320.0);
		let bonus_score = a * (self.bonus_value as f32 * self.bonus.sqrt() / 320.0);

		(base_score as u32) + (bonus_score as u32)
	}

	pub fn combo(&self) -> u32 {
		self.max_combo
	}
}
