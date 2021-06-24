//Gobble by Sebastien MacDougall-Landry
//License is available at
//https://github.com/EmperorPenguin18/gobble/blob/main/LICENSE

use std::{env, fmt, process};

fn main() -> Result<(), anyhow::Error> {
    let (conn, _screen_num) = xcb::Connection::connect(None)?;
    let win = xcb::get_input_focus(&conn).get_reply()?.focus();
    let args: Vec = env::args().collect();

    xcb::unmap_window_checked(&conn, win).request_check()?;
    conn.flush();

    let exit_code = command(args);

    xcb::map_window_checked(&conn, win).request_check()?;
    conn.flush();

    match exit_code {
        Ok(s) => process::exit(s),
        Err(e) => return Err(e),
    };
}

fn command(args: Vec) -> Result {
    let stat: i32 = process::Command::new(&args[1])
        .args(&args[2..])
        .status()?
        .code()
        .ok_or(fmt::Error)?;
    Ok(stat)
}
