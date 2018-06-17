extern crate clap;
extern crate ndimage;

use clap::{App, Arg};
use ndimage::core::padding::*;
use ndimage::io::png::{Decoder, Encoder8};

use std::fs::File;

fn main() {
    let matches = App::new("ndimage padding example")
        .version("0.0.1")
        .author("Robin C. <r.chavignat@gmail.com>")
        .about("Shows how to add padding to images")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input image file")
                .required(true)
                .index(1),
        )
        .get_matches();

    let img_path = matches.value_of("INPUT").unwrap();
    let in_file = File::open(img_path).unwrap();
    let decoder = Decoder::new(&in_file).unwrap();
    let img = decoder.read_luma_u8().unwrap();

    let padded_zeros = pad_zeros(&img, 10);
    let padded_repl = pad_replicate(&img, 10);
    let padded_wrap = pad_wrap(&img, 10);
    let padded_mirror = pad_mirror(&img, 10);

    let zeros_file = File::create("padded_zeros.png").unwrap();
    let repl_file = File::create("padded_replication.png").unwrap();
    let wrap_file = File::create("padded_wrap.png").unwrap();
    let mirror_file = File::create("padded_mirror.png").unwrap();
    let encoder = Encoder8::new(&padded_zeros, zeros_file).unwrap();
    encoder.write().unwrap();
    let encoder = Encoder8::new(&padded_repl, repl_file).unwrap();
    encoder.write().unwrap();
    let encoder = Encoder8::new(&padded_wrap, wrap_file).unwrap();
    encoder.write().unwrap();
    let encoder = Encoder8::new(&padded_mirror, mirror_file).unwrap();
    encoder.write().unwrap();
}
