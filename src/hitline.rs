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

	pub fn at(&mut self, t: f32) -> Vec<&HitObject> {
		let mut missed = Vec::new();
		for (key, current) in self.current.iter_mut().enumerate() {
			if let &mut Some(object) = current {
				let dt = t - (object.base().time as f32);
				if dt > 0.0 {
					let acc = self.overall_difficulty.hit_accuracy(dt);
					if acc == HitAccuracy::Miss {
						missed.push(object);
						*current = self.hit_objects[key].next();
					}
				}
			}
		}

		self.time = t;

		missed
	}

	pub fn press(&mut self, key: u32) -> Option<HitAccuracy> {
		let key = key as usize;
		match self.current[key] {
			None => None,
			Some(object) => {
				let dt = self.time - (object.base().time as f32);
				let acc = self.overall_difficulty.hit_accuracy(dt);

				match acc {
					HitAccuracy::Miss => None,
					_ => {
						self.current[key] = self.hit_objects[key].next();
						Some(acc)
					},
				}
			}
		}
	}

	pub fn release(&mut self, key: u32) -> Option<HitAccuracy> {
		None // TODO: hold notes
	}
}
