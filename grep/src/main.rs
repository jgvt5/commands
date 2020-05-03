extern crate getopts;
extern crate regex;
use getopts::Options;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn make_opt() -> Options {
    let mut opts = Options::new();
    opts.optflag("", "help", "print this help menu");
    opts.optflag("i", "ignore-case", "ignore case distinctions");
    opts.optflag("v", "invert-match", "select non-matching lines");
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
    if matches.opt_present("help") || matches.free.len() < 2 {
        print_usage(&program, opts);
        return;
    }
    let mut regbuilder = regex::RegexBuilder::new(&matches.free[0]);
    let mut is_inv = false;
    let rg: regex::Regex;
    if matches.opt_present("v") {
        is_inv = true;
    }
    if matches.opt_present("i") {
        rg = regbuilder.case_insensitive(true).build().unwrap();
    } else {
        rg = regbuilder.build().unwrap();
    }
    for i in 1..matches.free.len() {
        do_grep(&matches.free[i], &rg, is_inv);
    }
}

fn do_grep(name: &str, reg: &Regex, is_inv: bool) {
    let reader = BufReader::new(File::open(name).unwrap());
    for line in reader.lines().map(|l| l.unwrap()) {
        if reg.is_match(&line) ^ is_inv {
            println!("{}:{}", name, line);
        }
    }
}
