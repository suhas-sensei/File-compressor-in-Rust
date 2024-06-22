extern crate flate2;

use flate2::write::GzEncoder;
// creating compressed output
// allows you to create GZIP compressed data streams.
use flate2::Compression;
use std::env::args;
//args = This is a specific function within the env module. It provides access to the command-line arguments your program was started with.
use std::fs::File; //ds = file system reading.
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;
//show time it takes to compress file.

fn main(){
    if args().len() != 3 { //supposed to get 3 args
        eprintln!("Usage: 'source' 'target'"); //source and tagret files.
        return;
    }
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    //open a file
    let output = File::create(args().nth(2).unwrap()).unwrap();
    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();
    //encoding started
    copy(&mut input, &mut encoder).unwrap();
    //This line's core purpose is to efficiently transfer data from one source (input) to a destination (encoder). 
    let output = encoder.finish().unwrap();
    // Its primary goal is to finalize the compressed output and potentially handle any errors that might occur during this finalization.
    println!(
        "Source len: {:?}",
        input.get_ref().metadata().unwrap().len()
        );
    println!("Target len:{:?}", output.metadata().unwrap().len());
    println!("Elapsed: {:?}", start.elapsed());





















}