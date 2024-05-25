use evalexpr::build_operator_tree;
use evalexpr::context_map;
use evalexpr::HashMapContext;
use evalexpr::Node;

use crate::error::GraphingResult;

use self::point::GraphPoint;

pub mod point;

pub fn base_context(point: &GraphPoint) -> GraphingResult<HashMapContext> {
    let context = context_map! {
        "phi" => (1.0 + 5_f64.sqrt()) / 2.0,
        "x" => point.x(),
        "y" => point.y(),
        "z" => point.z()
    }?;

    Ok(context)
}

#[derive(Clone)]
pub struct GraphingFunction {
    statement: Node,
}

impl GraphingFunction {
    pub fn new(statement: &str) -> GraphingResult<Self> {
        let statement = build_operator_tree(&statement)?;

        Ok(GraphingFunction { statement })
    }

    pub fn is_shaded(&self, point: &GraphPoint) -> GraphingResult<bool> {
        let mut context = base_context(point)?;

        let shading = self.statement.eval_boolean_with_context_mut(&mut context)?;
        return Ok(shading);
    }
}
