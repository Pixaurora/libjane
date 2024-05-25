use crate::{
    error::GraphingResult,
    graph::{point::GraphPoint, GraphingFunction},
};

use super::Layer;

pub struct BasicLayer<T> {
    function: GraphingFunction,
    color: T,
}

impl<T> BasicLayer<T> {
    pub fn new(function: GraphingFunction, color: T) -> Self {
        BasicLayer { function, color }
    }
}

impl<T> Layer<T> for BasicLayer<T> {
    fn shade_at(&self, point: &GraphPoint) -> GraphingResult<Option<&T>> {
        Ok(if self.function.is_shaded(point)? {
            Some(&self.color)
        } else {
            None
        })
    }
}
