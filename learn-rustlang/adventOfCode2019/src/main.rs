use std::fs;
use std::io;
use std::path::PathBuf;
use structopt::StructOpt;
use adventOfCode2019;
//TODO 35 minutes into the video.

// Youtube coding with Brian Myers.
// TYoutube video: Rust live coding: Advent of code 2019 - Day 1
#[derive(Debug, StructOpt)]
struct Opt{
    /// Day
    day: usize,

    /// Option path to input file; If not supplied will read from stdin.
    input: Option<PathBuf>,
}
fn main() {
    let opt = Opt::from_args();
    let day_number = opt.day;
    
    match opt.input {
        // filename from the command args
        Some(path) => {
            // read the file
            let file = fs::File::open(path).expect("Could not read the file.\n");
            let reader = io::BufReader::new(file);
            day_challenge(reader,day_number);

        }
        // file piped into stdin as no filename passed in
        None => {
            let stdin = io::stdin();
            let guard = stdin.lock();
            let reader = io::BufReader::new(guard);
            day_challenge(reader,day_number);
        }
    }

fn day_challenge<R>(mut input: R,day_number:usize) where R: io::BufRead,{
       match day_number {
        1 => adventOfCode2019::day01::run(input),
        _ => panic!("Day must be between 1 and 25, inclusive "),
    };
}
}
