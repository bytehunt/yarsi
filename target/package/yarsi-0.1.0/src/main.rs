mod ascii;
mod colors;
mod wm;

use ascii::STUX;
use colors::*;
use columns::Columns;

fn main() {
    // Get the uptime
    let uptime = nixinfo::uptime().unwrap_or_else(|_| "N/A".to_string());
    // Get the kernel version
    let kernel = nixinfo::kernel().unwrap_or_else(|_| "N/A".to_string());
    // Get the window manager
    // let wm = nixinfo::environment().unwrap_or_else(|_| "N/A".to_string());//didn't work for me
    let wm = wm::get_wm().unwrap_or_else(|| String::from("N/A"));
    // Get the CPU information
    let cpu = nixinfo::cpu().unwrap_or_else(|_| "N/A".to_string());
    // Get the memory information
    let mem = nixinfo::memory().unwrap_or_else(|_| "N/A".to_string());

    // Create Columns struct with the information
    let fetch_info = Columns::from(vec![
        format!("{BLUE}{STUX}").split('\n').collect::<Vec<&str>>(),
        vec![
            // Add the username and hostname
            &format!(
                "     {WHITE}{}{RED}@{RESET}{}{BLUE}",
                whoami::username(),
                whoami::hostname()
            ),
            // Add the distro name
            &format!("{CYAN}os{WHITE}  ~ {CYAN}{}{BLUE}", whoami::distro()),
            // Add the uptime
            &format!("{YELLOW}upt{WHITE} ~ {YELLOW}{}{BLUE}", uptime),
            // Add the window manager
            &format!("{GREEN}wm {WHITE} ~ {GREEN}{wm}{BLUE}"),
            // Add the memory information
            &format!("{MAGENTA}mem{WHITE} ~ {RED}{}{BLUE}", mem),
            // Add the kernel version
            &format!("{GREEN}kr{WHITE}  ~ {YELLOW_BRIGHT}{}{BLUE}", kernel),
            // Add the CPU information
            &format!("{YELLOW}cpu{WHITE} ~ {BLUE}{}{BLUE}", cpu),
            // Add the line of dots
            &format!("{RED}● {YELLOW}● {CYAN}● {BLUE}● {WHITE}●"),
        ],
    ])
    // Set the tabsize for the columns
    .set_tabsize(15)
    // Create the columns
    .make_columns();

    // Print the columns
    println!("{fetch_info}");
}
