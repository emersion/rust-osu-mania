pub enum HitAccuracy {
	ThreeHundred,
	OneHundred,
	Fifty,
	Missed,
}

impl HitAccuracy {
	pub fn score(self) -> i32 {
		match self {
			HitAccuracy::ThreeHundred => 300,
			HitAccuracy::OneHundred => 100,
			HitAccuracy::Fifty => 50,
			HitAccuracy::Missed => 0,
		}
	}
}

pub struct OverallDifficulty {
	three_hundred: f32,
	one_hundred: f32,
	fifty: f32,
}

impl OverallDifficulty {
	pub fn new(od: f32) -> OverallDifficulty {
		OverallDifficulty{
			three_hundred: 79.5 - od*6.0,
			one_hundred: 139.5 - od*8.0,
			fifty: 199.5 - od*10.0,
		}
	}

	pub fn hit_accuracy(&self, delay: f32) -> HitAccuracy {
		if delay < self.three_hundred {
			HitAccuracy::ThreeHundred
		} else if delay < self.one_hundred {
			HitAccuracy::OneHundred
		} else if delay < self.fifty {
			HitAccuracy::Fifty
		} else {
			HitAccuracy::Missed
		}
	}
}
