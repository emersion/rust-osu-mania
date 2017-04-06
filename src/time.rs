use std::time::Duration;

pub trait AsMillis {
	fn as_millis(&self) -> u64;
}

impl AsMillis for Duration {
	fn as_millis(&self) -> u64 {
		return self.as_secs()*1_000 + (self.subsec_nanos() as u64)/1_000_000;
	}
}
