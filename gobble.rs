//Gobble by Sebastien MacDougall-Landry
//License is available at
//https://github.com/EmperorPenguin18/gobble/blob/main/LICENSE

use std::{env, process, thread, time, fmt};

fn main() -> Result<(), anyhow::Error> {
    let (conn, _screen_num) = xcb::Connection::connect(None)?;
    let win = xcb::get_input_focus(&conn).get_reply()?.focus();
    let args: Vec = env::args().collect();

    xcb::unmap_window_checked(&conn, win).request_check()?;
    conn.flush();

    let stat: i32 = process::Command::new(&args[1])
        .args(&args[2..])
        .status()?
        .code()
        .ok_or(fmt::Error)?;

    xcb::map_window_checked(&conn, win).request_check()?;
    conn.flush();

    thread::sleep(time::Duration::from_millis(10));
    //Makes cli commands work

    process::exit(stat);
}
