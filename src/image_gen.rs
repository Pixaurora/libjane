use image::{ColorType, DynamicImage, Rgba};

use crate::{error::GraphingResult, graph::point::GraphPoint, layer::LayerStack};

pub trait GraphsImage {
    fn graph_to_image(&self, size: (u32, u32)) -> GraphingResult<DynamicImage>;
}

impl GraphsImage for LayerStack<Rgba<u8>> {
    fn graph_to_image(&self, size: (u32, u32)) -> GraphingResult<DynamicImage> {
        let image = DynamicImage::new(size.0, size.1, ColorType::Rgba8);
        let mut image = image.into_rgba8();

        let (size_x, size_z) = size;
        let (center_x, center_z) = (size_x as f64 / 2.0, size_z as f64 / 2.0);

        for z in 0..size_z {
            for x in 0..size_x {
                let point = GraphPoint::new(x as f64 - center_x, 0.0, -(z as f64 - center_z));
                let color = self.shade_at(&point)?;
                image.put_pixel(x, z, color.clone());
            }
        }

        Ok(DynamicImage::ImageRgba8(image))
    }
}
