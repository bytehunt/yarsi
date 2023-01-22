mod ascii;
mod colors;

use ascii::ARCH;
use colors::*;
use columns::Columns;
use nixinfo::distro;

fn main() {
  println!("{}", ARCH);
    if let Ok(distro_info) = distro() {
        println!("Distribution: {}", distro_info);
    } else {
        println!("Error getting distribution information");
    }
}

