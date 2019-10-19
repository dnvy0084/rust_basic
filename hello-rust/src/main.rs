use ferris_says::say;
use std::io::{BufWriter, stdout};

fn main() {
    let out = stdout();
    let str = b"hello rust world, I`m jh. I`m new to cargo";
    let width = 30;

    let mut writer = BufWriter::new(out.lock());
    
    say(str, width, &mut writer).unwrap();
}

