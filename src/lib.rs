pub mod error;
pub mod graph;
#[cfg(feature = "image_gen")]
pub mod image_gen;
pub mod layer;

#[cfg(test)]
mod tests {
    use crate::graph::point::GraphPoint;

    use super::graph::GraphingFunction;

    #[test]
    fn test_shading() {
        let function = GraphingFunction::new("x > z").unwrap();

        assert_eq!(
            function
                .is_shaded(&GraphPoint::new(-1.0, 0.0, 1.0))
                .unwrap(),
            false
        ); // -1 > 1 is false

        assert_eq!(
            function
                .is_shaded(&GraphPoint::new(10.0, 0.0, -10.0))
                .unwrap(),
            true
        ) // 10 > -10 is true
    }
}
