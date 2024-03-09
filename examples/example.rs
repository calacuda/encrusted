extern crate encrusted;

use encrusted::build_game;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let mut zvm = build_game(Path::new("./assets/zork1.z3"))?;
    println!("Intro -> {:?}\n{}", zvm.get_score(), zvm.start_game());
    println!("====================");
    println!("{} -> {:?}\n{}", zvm.get_location(), zvm.get_score(), zvm.exec_input("open mailbox"));
    println!("====================");
    println!("{} -> {:?}\n{}", zvm.get_location(), zvm.get_score(), zvm.exec_input("read leaflet"));
    println!("** DONE **");

    Ok(())
}
