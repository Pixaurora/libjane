use crate::{error::GraphingResult, graph::point::GraphPoint};

pub mod graph;

pub trait Layer<T> {
    fn shade_at(&self, point: &GraphPoint) -> GraphingResult<Option<&T>>;
}

pub struct LayerStack<T> {
    layers: Vec<Box<dyn Layer<T>>>,
    default_shade: T,
}

impl<T> LayerStack<T> {
    pub fn new(layers: Vec<Box<dyn Layer<T>>>, default_shade: T) -> Self {
        LayerStack {
            layers,
            default_shade,
        }
    }

    pub fn shade_at(&self, point: &GraphPoint) -> GraphingResult<&T> {
        for layer in &self.layers {
            let shade = layer.shade_at(point)?;

            if let Some(shade) = shade {
                return Ok(shade);
            }
        }

        Ok(&self.default_shade)
    }
}
