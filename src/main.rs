use flate2::bufread::GzDecoder;
use std::fs::File;
use std::io::BufReader;
use std::{env, io};

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut stdout = io::stdout();

    let mut buffered = open_buffered_gzip(filename)?;

    io::copy(&mut buffered, &mut stdout)?;
    Ok(())
}

fn open_buffered_gzip(filename: &str) -> Result<GzDecoder<BufReader<File>>, std::io::Error> {
    let f = File::open(filename)?;
    let buf_reader = BufReader::new(f);
    Ok(GzDecoder::new(buf_reader))
}
