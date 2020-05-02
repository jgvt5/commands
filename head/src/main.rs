extern crate getopts;
use getopts::Options;
use std::env;
use std::fs::File;
use std::io::BufReader;

mod do_head;

const DEFAULT_N_LINES: i32 = 10;

fn make_opt() -> Options {
    let mut opts = Options::new();
    opts.optflag("", "help", "print this help menu");
    opts.optopt(
        "c",
        "bytes",
        "print the first NUM bytes of each file",
        "NUM",
    );
    opts.optopt(
        "n",
        "lines",
        "print the first NUM lines of each file",
        "NUM",
    );
    opts.optflag("q", "quiet", " never print headers giving file names");
    opts.optflag("v", "verbose", "always print headers giving file names");
    opts
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let opts = make_opt();
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("help") {
        do_head::print_usage(&program, opts);
        return;
    }
    let mut is_verbose = false;
    let mut is_line = true;
    let mut num = DEFAULT_N_LINES;

    if let Some(argn) = matches.opt_str("n") {
        if let Ok(lines) = argn.parse::<i32>() {
            num = lines;
        } else {
            println!("Invailed argument {}", argn);
            do_head::print_usage(&program, opts);
            return;
        }
    }
    if let Some(argc) = matches.opt_str("c") {
        if let Ok(bytes) = argc.parse::<i32>() {
            num = bytes;
            is_line = false;
        } else {
            println!("Invailed argument {}", argc);
            do_head::print_usage(&program, opts);
            return;
        }
    }

    if matches.opt_present("v") {
        is_verbose = true;
    }

    for input in matches.free.iter() {
        let mut reader = BufReader::new(File::open(input).unwrap());
        if is_verbose {
            println!("==> {} <==", input);
        }
        if is_line {
            do_head::do_head_n(&mut reader, num as usize);
        } else {
            do_head::do_head_c(&mut reader, num as usize);
        }
    }
}
