use std::io;

pub fn run<R>(mut input: R) -> usize where R: io::BufRead, {

    //buffer used for reading in the file
    let mut buffer = String::new();
    // total numbers added up
    let mut total = 0;
    // loop to read the lines from the file
    loop {
        if input.read_line(&mut buffer).unwrap() == 0 {
            break;
        }
        // convert string into a integer number and remove the new line
        let n = buffer.trim().parse::<usize>().expect("Could not parse the number and trim the new line");
        // On the number do the calculation which is (number / 3 - 2)
        let m = match (n / 3).checked_sub(2) {
            Some(m) => m,
            None => 0,
        };
    total += m;
        print!("{}", n);
        buffer.clear();
    }
    total
}
