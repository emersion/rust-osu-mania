use std::slice::Iter;
use osu_format::TimingPoint;

pub struct Timeline<'a> {
	timing_points: Iter<'a, TimingPoint>,
	current: TimingPoint,
	last_parent: TimingPoint,
	next: Option<&'a TimingPoint>,
}

impl<'a> Timeline<'a> {
	pub fn new(timing_points: &Vec<TimingPoint>) -> Timeline {
		let mut iter = timing_points.iter();
		let next = iter.next();

		Timeline{
			timing_points: iter,
			current: next.unwrap().clone(),
			last_parent: next.unwrap().clone(),
			next: next,
		}
	}

	pub fn at(&mut self, t: i32) -> &TimingPoint {
		let t = if t < 0 {
			0
		} else {
			t as u32
		};

		while let Some(next) = self.next {
			if next.offset > t {
				break;
			}
			self.current = next.inherit(&self.last_parent);
			if !next.inherited {
				self.last_parent = next.clone();
			}
			self.next = self.timing_points.next();
		}

		&self.current
	}
}
