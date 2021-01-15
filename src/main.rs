use torrent::*;
use clap::{Arg, App};

fn main() {
    let matches = App::new("fetch torrent magnet")
        .version("0.1.0")
        .author("daite <blueskykind02@yahoo.co.jp>")
        .about("search torrent files")
        .arg(Arg::with_name("search")
                 .short("s")
                 .long("search")
                 .takes_value(true)
                 .help("search torrent magnet file"))
        .get_matches();
    let myfile = matches.value_of("search").unwrap_or("동상이몽");
   if let Err(e) = ttobogo::run(myfile) {
       println!("{}", e);
   }
}