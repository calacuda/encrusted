// extern crate atty;
extern crate base64;
// extern crate clap;
extern crate rand;
// extern crate regex;
extern crate serde_json;
// extern crate term_size;
extern crate anyhow;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate enum_primitive;

#[macro_use]
extern crate serde_derive;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process;
use anyhow::Result;

// use clap::{App, Arg};

mod buffer;
mod frame;
mod instruction;
mod options;
mod quetzal;
mod traits;
// mod ui_terminal;
mod ui_code;
mod zmachine;

use options::Options;
use traits::UI;
// use ui_terminal::TerminalUI;
use ui_code::CodeUI;
use zmachine::Zmachine;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn build_game(path: &Path) -> Result<Zmachine> {
    // let matches = App::new("encrusted")
    //     .version(VERSION)
    //     .about("A zmachine interpreter")
    //     .arg(
    //         Arg::with_name("FILE")
    //             .help("Sets the story file to run")
    //             .required(true),
    //     )
    //     .get_matches();
    //
    // let path = Path::new(matches.value_of("FILE").unwrap());

    if !path.is_file() {
        println!(
            "\nCouldn't find game file: \n   {}\n",
            path.to_string_lossy()
        );
        process::exit(1);
    }

    let mut data = Vec::new();
    let mut file = File::open(path)?;  // .expect("Error opening file");
    file.read_to_end(&mut data)?;      // .expect("Error reading file");

    let version = data[0];

    if version == 0 || version > 8 {
        println!(
            "\n\
             \"{}\" has an nsupported game version: {}\n\
             Is this a valid game file?\n",
            path.to_string_lossy(),
            version
        );
        process::exit(1);
    }

    let ui = CodeUI::new();

    let mut opts = Options::default();
    opts.save_dir = path.parent().unwrap().to_string_lossy().into_owned();
    opts.save_name = path.file_stem().unwrap().to_string_lossy().into_owned();

    let rand32 = || rand::random();
    opts.rand_seed = [rand32(), rand32(), rand32(), rand32()];

    let zvm = Zmachine::new(data, ui, opts);

    Ok(zvm)
}
