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

//　↓無みたいなコード
fn rm(filename: &str) {
    fs::remove_file(filename).expect("Can't remove file")
}

fn rmdir(dirname: &str) {
    fs::remove_dir(dirname).expect("Can't remove directory")
}

fn ln(linkname: &str, dstname: &str) {
    fs::hard_link(linkname, dstname).expect("Can't make hard link")
}
