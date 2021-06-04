use flate2::bufread::GzDecoder;
use std::fs::File;
use std::io::BufReader;
use std::{env, io};

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let f = File::open(filename)?;
    let buf_reader = BufReader::new(f);
    let mut d = GzDecoder::new(buf_reader);
    let mut stdout = io::stdout();

    io::copy(&mut d, &mut stdout)?;
    Ok(())
}
