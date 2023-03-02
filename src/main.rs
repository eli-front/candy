use clap::{Parser, Subcommand};
use image::{ImageBuffer, Rgb};
use std::{
    fs::File,
    io::{Read, Write},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true, name = "encode")]
    Encode {
        #[arg(name = "input", required = true)]
        input: String,

        #[arg(name = "output", required = true)]
        output: String,
    },
    #[command(arg_required_else_help = true, name = "decode")]
    Decode {
        #[arg(name = "input", required = true)]
        input: String,

        #[arg(name = "output", required = true)]
        output: String,
    },
}

fn encode(input: String, output: String) {
    let mut input = File::open(input)
        .expect("Could not open input file")
        .bytes()
        .map(|x| x.expect("Could not read input file"))
        .collect::<Vec<u8>>();

    let size = input.len();

    let sqrt = ((size as f64) / 3.0).sqrt().ceil() as u32;

    let img = ImageBuffer::from_fn(sqrt, sqrt, |_, _| {
        // TODO: find a better way to idicate end of file and not to require the file's byte count to be a multiple of 3

        if input.is_empty() {
            return Rgb([255, 255, 255]);
        }

        let r = input.pop().expect("Could not pop r");
        let g = input.pop().expect("Could not pop g");
        let b = input.pop().expect("Could not pop b");

        Rgb([r, g, b])
    });

    img.save(output).unwrap();
}

fn decode(input: String, output: String) {
    let binding = image::open(input).unwrap().into_rgb8();
    let pixels = binding.pixels();

    // get all pixels until a pixel with all 0 values is found
    let file_bytes = pixels
        .take_while(|x| x.0 != [255, 255, 255])
        .map(|x| x.0)
        .flatten()
        .collect::<Vec<u8>>();

    let file_bytes = file_bytes.iter().rev().copied().collect::<Vec<u8>>();

    println!("File size: {} bytes", file_bytes.len());

    let mut output = File::create(output).unwrap();
    output.write_all(&file_bytes).unwrap();
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Encode { input, output } => {
            println!("Encoding {} -> {}", input, output);
            encode(input, output);
        }
        Commands::Decode { input, output } => {
            println!("Decoding {} -> {}", input, output);
            decode(input, output);
        }
    }
}
