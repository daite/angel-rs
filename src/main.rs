use torrent::*;
use clap::{Arg, App};

fn main() {
    let matches = App::new("fetch torrent magnet")
        .version("0.1.0")
        .author("daite <blueskykind02@yahoo.co.jp>")
        .about("search torrent magnet")
        .arg(Arg::with_name("keyword")
                 .short("s")
                 .long("search")
                 .takes_value(true)
                 .help("search torrent magnet file"))
        .get_matches();
    let myfile = matches.value_of("keyword").unwrap_or("동상이몽2");  
    let result = sites::ttobogo::run(myfile);
    match result {
        Ok(status) => {
            match status {
                DataStatus::Found => (),
                DataStatus::NotFound => {
                    if let Err(e) = sites::torrentmobile::run(myfile) {
                        println!("{:?}", e);
                    }
                }
            }
        }
        Err(e) => println!("{:?}", e),
    }
}
