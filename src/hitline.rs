use std::iter::Iterator;
use osu_format::HitObject;
use std::borrow::Borrow;

use difficulty::{OverallDifficulty, HitAccuracy};

const PLAYFIELD_WIDTH: f32 = 512.0;

pub struct HitLine<'a> {
	overall_difficulty: OverallDifficulty,
	hit_objects: Vec<Box<Iterator<Item=&'a HitObject> + 'a>>,
	current: Vec<Option<&'a HitObject>>,
	time: u32,
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

		HitLine{
			overall_difficulty: od,
			hit_objects: iterators,
			current: current,
			time: 0,
		}
	}

	pub fn forward(&mut self, t: u32) -> Vec<&HitObject> {
		self.time = t;
		// TODO: return missed notes
		Vec::new()
	}

	pub fn press(&mut self, key: u32) -> Option<HitAccuracy> {
		None
	}

	pub fn release(&mut self, key: u32) -> Option<HitAccuracy> {
		None
	}
}
