use evalexpr::build_operator_tree;
use evalexpr::context_map;
use evalexpr::HashMapContext;
use evalexpr::Node;

use crate::errors::JaneratorResult;

pub fn base_context(x: f64, y: f64, z: f64) -> JaneratorResult<HashMapContext> {
    let context = context_map! {
        "phi" => (1.0 + 5_f64.sqrt()) / 2.0,
        "x" => x,
        "y" => y,
        "z" => z
    }?;

    Ok(context)
}

pub struct GraphingFunction {
    statement: Node,
}

impl GraphingFunction {
    pub fn new(statement: &str) -> JaneratorResult<Self> {
        let statement = build_operator_tree(&statement)?;

        Ok(GraphingFunction { statement })
    }

    pub fn is_shaded(&self, x: f64, y: f64, z: f64) -> JaneratorResult<bool> {
        let context = base_context(x, y, z)?;

        let shading = self.statement.eval_boolean_with_context(&context)?;
        return Ok(shading);
    }
}
