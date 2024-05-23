pub mod errors;
pub mod functions;

#[cfg(test)]
mod tests {
    use super::functions::GraphingFunction;

    #[test]
    fn test_shading() {
        let function = GraphingFunction::new("x > z").unwrap();

        assert_eq!(function.is_shaded(-1.0, 0.0, 1.0).unwrap(), false); // -1 > 1 is false

        assert_eq!(function.is_shaded(10.0, 0.0, -10.0).unwrap(), true) // 10 > -10 is true
    }
}
