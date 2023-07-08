use crate::blocs::{Bloc, Skeleton};
use nalgebra::Vector2;
use std::collections::HashMap;

pub struct Print {
	skeleton: Skeleton,
}

impl Bloc for Print {
	fn get_size(&self, blocs: &HashMap<u32, Box<dyn Bloc>>) -> Vector2<f64> {
		self.skeleton.slots.get(0).unwrap().get_size(blocs) + Vector2::new(2.0, 2.0) // * Self::MARGIN
	}
}
