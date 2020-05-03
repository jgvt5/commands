use std::{fs, io};

fn main() {
    match fs::read_dir(".") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => {
            for path in paths.map(|p| p.unwrap()) {
                println!("{:?} ", path.path());
            }
        }
    }
}

fn ls(dirname: &str) {
    match fs::read_dir(dirname) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => {
            for path in paths.map(|p| p.unwrap()) {
                println!("{:?} ", path.path());
            }
        }
    }
}

fn mkdir(dirname: &str) {
    match fs::create_dir(dirname) {
        Ok(_) => return,
        Err(e) => println!("{}", e),
    }
}
