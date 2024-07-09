use std::fmt::Debug;

use dyn_eq::DynEq;

use crate::prelude::*;

pub trait Pattern: DynEq + Debug + Transformable {
    fn color_at(&self, point: &Point) -> Color;
}

dyn_eq::eq_trait_object!(Pattern);
