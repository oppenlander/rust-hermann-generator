#![crate_type = "bin"]
#![feature(old_io)]
#![feature(old_path)]

extern crate image;
extern crate "rustc-serialize" as rustc_serialize;
extern crate docopt;

mod hermann;

use docopt::Docopt;

static USAGE: &'static str = "
Generate a black/white Hermann Grid

Usage: hermann  <dest> <imgx> <imgy> [--inverted --centered]
       hermann --help

Options:
    -h, --help        Show this message.
    -i, --inverted    Invert the black/white colors.
    -c, --centered    Center the image.
";

#[derive(RustcDecodable, Debug)]
struct Args {
    arg_dest: String,
    arg_imgx: u32,
    arg_imgy: u32,
    flag_inverted: bool,
    flag_centered: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());
    println!("Outputting to {}", args.arg_dest);
    hermann::gen(args.arg_dest, args.arg_imgx, args.arg_imgy, args.flag_inverted, args.flag_centered);
    println!("All done!");
}
