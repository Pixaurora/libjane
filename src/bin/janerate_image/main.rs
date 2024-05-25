use clap::Parser;
use hex::FromHex;
use image::Rgba;
use libjane::error::{GraphingError, GraphingResult};
use libjane::graph::GraphingFunction;
use libjane::image_gen::GraphsImage;
use libjane::layer::{graph::BasicLayer, LayerStack};

pub fn parse_color(text: &str) -> GraphingResult<Rgba<u8>> {
    let color = <[u8; 4]>::from_hex(text)?;

    Ok(Rgba(color))
}

#[derive(Parser)]
struct Args {
    output_file: String,
    #[arg(value_parser = GraphingFunction::new)]
    function: GraphingFunction,
    #[arg(value_parser = parse_color)]
    background_color: Rgba<u8>,
    #[arg(value_parser = parse_color)]
    foreground_color: Rgba<u8>,
}

fn main() -> Result<(), GraphingError> {
    let args = Args::parse();

    let layer_stack = LayerStack::new(
        vec![Box::new(BasicLayer::new(
            args.function,
            args.foreground_color,
        ))],
        args.background_color,
    );

    let image = layer_stack.graph_to_image((1024, 1024))?;

    image.save(args.output_file)?;

    println!("Success!");
    Ok(())
}
