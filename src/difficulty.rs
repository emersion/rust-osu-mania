use std::cmp;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum HitAccuracy {
	Miss,
	Fifty,
	OneHundred,
	TwoHundred,
	ThreeHundred,
}

impl HitAccuracy {
	pub fn score(&self) -> u32 {
		match *self {
			HitAccuracy::ThreeHundred => 300,
			HitAccuracy::TwoHundred => 200,
			HitAccuracy::OneHundred => 100,
			HitAccuracy::Fifty => 50,
			HitAccuracy::Miss => 0,
		}
	}

	pub fn prev(&self) -> HitAccuracy {
		match *self {
			HitAccuracy::ThreeHundred => HitAccuracy::TwoHundred,
			HitAccuracy::TwoHundred => HitAccuracy::OneHundred,
			HitAccuracy::OneHundred => HitAccuracy::Fifty,
			_ => HitAccuracy::Miss,
		}
	}

	pub fn hold_note(self, released: HitAccuracy) -> HitAccuracy {
		match (self, released) {
			// Pressed (maybe on time) and not released
			(pressed, HitAccuracy::Miss) => pressed.prev(),
			// NG (Not Good): pressed, released, pressed again during the note and released on time
			(HitAccuracy::Miss, _) => HitAccuracy::Fifty,
			(pressed, released) => {
				// Pressed on time and released (maybe on time)
				println!("Hold note: {:?}, {:?} -> {:?}", &pressed, &released, cmp::min(&pressed, &released));
				cmp::min(pressed, released)
			},
		}
	}
}

#[derive(Debug)]
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
		let delay = delay.abs();
		if delay < self.three_hundred {
			HitAccuracy::ThreeHundred
		} else if delay < self.one_hundred {
			HitAccuracy::OneHundred
		} else if delay < self.fifty {
			HitAccuracy::Fifty
		} else {
			HitAccuracy::Miss
		}
	}
}
