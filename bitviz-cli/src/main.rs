use std::io::{stdin, stdout};

use bitviz::{
    reader_to_bits,
    render::{render_bits_doc, RenderStyle},
    WordSize,
};
use byteorder::{BigEndian, LittleEndian};
use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum Endianness {
    Big,
    Little,
}

#[derive(Parser, Debug)]
struct Command {
    #[arg(short, long, value_enum, default_value_t = Endianness::Big)]
    endian: Endianness,

    #[arg(short, long)]
    word_size: usize,

    #[arg(short, long)]
    group_size: Option<usize>,

    #[arg(short, long)]
    line_size: usize,

    #[arg(long, default_value_t = 5)]
    line_padding: usize,

    #[arg(long, default_value_t = 5)]
    group_padding: usize,

    #[arg(long, default_value_t = 10)]
    box_size: usize,

    #[arg(long)]
    style: Option<String>,

    #[arg(short, long)]
    output: Option<String>,
}
fn main() {
    let args = Command::parse();

    let bits = match args.endian {
        Endianness::Big => either::Left(reader_to_bits::<_, BigEndian>(
            stdin(),
            WordSize::new(args.word_size),
        )),
        Endianness::Little => either::Right(reader_to_bits::<_, LittleEndian>(
            stdin(),
            WordSize::new(args.word_size),
        )),
    };

    let doc = render_bits_doc(
        bits,
        &RenderStyle {
            group_size: args.group_size.unwrap_or(args.word_size * 8),
            line_size: args.line_size,
            group_padding: args.group_padding,
            line_padding: args.line_padding,
            box_size: args.box_size,
            style: args
                .style
                .map(|s| String::from_utf8(std::fs::read(s).unwrap()).unwrap()),
        },
    );

    if let Some(output) = args.output {
        svg::save(output, &doc).unwrap();
    } else {
        svg::write(stdout(), &doc).unwrap();
    }
}
