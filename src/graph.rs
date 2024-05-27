use evalexpr::build_operator_tree;
use evalexpr::ContextWithMutableVariables;
use evalexpr::Node;
use evalexpr::Value;

use crate::error::GraphingResult;

use self::math::base_context;
use self::point::GraphPoint;

mod math;
pub mod point;

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
        let mut context = base_context()?;

        for (name, value) in [("x", point.x()), ("y", point.y()), ("z", point.z())] {
            context.set_value(String::from(name), Value::Float(value))?;
        }

        let shading = self.statement.eval_boolean_with_context_mut(&mut context)?;
        return Ok(shading);
    }
}
