mod ascii;
mod colors;

use ascii::ARCH;
use colors::*;
use nixinfo::*;


fn main() {
    let distro = nixinfo::distro();
    let shell = nixinfo::env("SHELL");
    let uptime = nixinfo::uptime();
    let kernel = nixinfo::kernel();
    let wm = nixinfo::environment();

    println!(
        "\n{}  {}  {}",
        CYAN,
        " ",
        &distro.unwrap().replace("\"", "")
    );
    println!(
        "  {}  {}  {}",
        GREEN,
        " ",
        &shell.unwrap().replace("/usr/bin/", "")
    );
    println!("  {}  {}  {}", BLUE, "󱑇 ", &uptime.unwrap());
    println!("  {}  {}  {}", YELLOW, " ", &kernel.unwrap());
    println!("  {}  {}  {}", MAGENTA, "󱀜 ", &wm.unwrap());
    
    println!("{}", RESET);
    println!("{}", ARCH);
}
