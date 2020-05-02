use getopts::Options;
use std::cmp::min;
use std::io::{stdout, BufRead, BufWriter, Write};

// -h, --help
pub fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

// -c, --bytes
pub fn do_head_c(reader: &mut dyn BufRead, n: usize) {
    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    let mut count = 0;
    while count < n {
        let mut buf: [u8; 8192] = [0; 8192];
        let size = reader.read(&mut buf).unwrap();
        out.write_all(&mut buf[0..min(n - count, size)])
            .expect("can't write buffer");
        count += size;
    }
}

// -n, --lines
pub fn do_head_n(reader: &mut dyn BufRead, n: usize) {
    let mut count = 0;
    while count < n {
        count += 1;
        let mut buf = String::new();
        reader.read_line(&mut buf).expect("can't read_line");
        if buf.is_empty() {
            break;
        }
        print!("{}", buf);
    }
}
