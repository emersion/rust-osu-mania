use std::iter::Iterator;
use osu_format::HitObject;

use difficulty::{OverallDifficulty, HitAccuracy};

const PLAYFIELD_WIDTH: f32 = 512.0;

pub struct HitLine<'a> {
	overall_difficulty: OverallDifficulty,
	hit_objects: Vec<Box<Iterator<Item=&'a HitObject> + 'a>>,
	current: Vec<Option<&'a HitObject>>,
	last: Vec<Option<HitAccuracy>>,
	time: f32,
}

impl<'a> HitLine<'a> {
	pub fn new(keys_count: u32, od: OverallDifficulty, hit_objects: &'a Vec<HitObject>) -> HitLine<'a> {
		let key_width = 1.0/(keys_count as f32);

		let mut iterators = (0..keys_count).map(|filter_key| {
			Box::new(
				hit_objects.iter().filter(move |object| {
					let base = object.base();
					let key = ((base.x as f32) / PLAYFIELD_WIDTH / key_width) as u32;
					(key == filter_key)
				})
			) as Box<Iterator<Item=_>>
		}).collect::<Vec<_>>();

		let current = iterators.iter_mut()
		.map(|iter| iter.next())
		.collect::<Vec<_>>();

		let last = iterators.iter().map(|_| None).collect::<Vec<_>>();

		HitLine{
			overall_difficulty: od,
			hit_objects: iterators,
			current: current,
			last: last,
			time: 0.0,
		}
	}

	fn press_delay(&self, t: f32, object: &HitObject) -> f32 {
		let mut dt = self.time - t;
		if let HitObject::LongNote{..} = *object {
			dt /= 1.2
		}
		dt
	}

	fn release_delay(&self, t: f32, object: &HitObject) -> f32 {
		let mut dt = self.time - t;
		if let HitObject::LongNote{..} = *object {
			dt /= 2.4
		}
		dt
	}

	pub fn at(&mut self, t: f32) -> Vec<HitAccuracy> {
		self.time = t;

		let mut missed = Vec::new();
		for key in 0..self.current.len() {
			let current = self.current[key];

			let object = match current {
				Some(object) => object,
				None => continue,
			};

			let end_time = match *object {
				HitObject::LongNote{end_time, ..} => end_time,
				_ => object.base().time,
			};
			let dt = self.release_delay(end_time as f32, object);
			if dt < 0.0 {
				continue; // Object in the future
			}

			let mut acc = self.overall_difficulty.hit_accuracy(dt);
			if acc != HitAccuracy::Miss {
				continue; // Object in the past, but can still be hit
			}

			if let Some(last_acc) = self.last[key].take() {
				acc = last_acc.hold_note(acc);
			}
			missed.push(acc);
			self.current[key] = self.hit_objects[key].next();
		}

		missed
	}

	pub fn press(&mut self, key: u32) -> Option<HitAccuracy> {
		let key = key as usize;
		let current = match self.current[key] {
			None => return None,
			Some(current) => current,
		};

		let dt = self.press_delay(current.base().time as f32, current);

		match current {
			&HitObject::Circle{..} => {
				match self.overall_difficulty.hit_accuracy(dt) {
					HitAccuracy::Miss => None,
					acc @ _ => {
						self.current[key] = self.hit_objects[key].next();
						Some(acc)
					},
				}
			},
			&HitObject::LongNote{..} => {
				if let Some(_) = self.last[key] {
					// Pressed, released, pressed again
					None
				} else {
					// Pressed for the first time, maybe on time
					self.last[key] = Some(self.overall_difficulty.hit_accuracy(dt));
					None
				}
			},
			_ => None,
		}
	}

	pub fn release(&mut self, key: u32) -> Option<HitAccuracy> {
		let key = key as usize;
		let end_time = match self.current[key] {
			Some(&HitObject::LongNote{end_time, ..}) => end_time,
			_ => return None,
		};
		let last = match self.last[key].take() {
			Some(last) => last,
			None => return None,
		};

		let dt = self.release_delay(end_time as f32, self.current[key].unwrap());
		match self.overall_difficulty.hit_accuracy(dt) {
			HitAccuracy::Miss => {
				// Released too early
				self.last[key] = Some(HitAccuracy::Miss);
				None
			},
			acc @ _ => {
				// Released on time
				self.current[key] = self.hit_objects[key].next();
				Some(last.hold_note(acc))
			},
		}
	}
}
