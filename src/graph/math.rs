use evalexpr::math_consts_context;
use evalexpr::ContextWithMutableFunctions;
use evalexpr::ContextWithMutableVariables;
use evalexpr::EvalexprError;
use evalexpr::Function;
use evalexpr::HashMapContext;
use evalexpr::Value;

use crate::error::GraphingResult;

pub fn function_from<F>(function: F) -> Function
where
    F: Fn(f64) -> f64,
    F: Send + Sync + 'static,
    F: Clone,
{
    Function::new(move |argument| {
        let argument = match argument {
            Value::Float(value) => *value,
            Value::Int(value) => *value as f64,
            _ => {
                return Err(EvalexprError::ExpectedFloat {
                    actual: argument.to_owned(),
                })
            }
        };

        let result = function(argument);

        Ok(Value::Float(result))
    })
}

pub fn base_context() -> GraphingResult<HashMapContext> {
    let mut context = math_consts_context!()?;

    context.set_value(
        String::from("PHI"),
        Value::Float((1.0 + 5_f64.sqrt()) / 2.0),
    )?;

    context.set_function(
        String::from("sign"),
        function_from(|value| {
            if value == 0.0 {
                0.0
            } else if value > 0.0 {
                1.0
            } else {
                -1.0
            }
        }),
    )?;

    Ok(context)
}
