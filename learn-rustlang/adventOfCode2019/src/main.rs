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
    
    match opt.input {
        // filename from the command args
        Some(path) => {
            // read the file
            let file = fs::File::open(path).expect("Could not read the file.\n");
            let reader = io::BufReader::new(file);
            let answer = adventOfCode2019::day01::run(reader);
            println!("The answer is {}\n", answer);
        }
        // file piped into stdin as no filename passed in
        None => {
            let stdin = io::stdin();
            let guard = stdin.lock();
            let answer = adventOfCode2019::day01::run(guard);
            println!("The answer is {}\n", answer);
        },
    }
}
