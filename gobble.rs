//Gobble by Sebastien MacDougall-Landry
//License is available at
//https://github.com/EmperorPenguin18/gobble/blob/main/LICENSE

use std::{env, process};

fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().skip(1).collect();
    let wayland = env::var_os("WAYLAND_DISPLAY");

    let exit_code = if wayland == None {
        // X11
        let (conn, _screen_num) = xcb::Connection::connect(None)?;
        let win = xcb::get_input_focus(&conn).get_reply()?.focus();

        // ensure child was spawned before we hide the window
        let mut child: process::Child;
        if args.len() > 1 { child = command(&args)?; }
        else { process::exit(0); }

        xcb::unmap_window_checked(&conn, win).request_check()?;
        conn.flush();

        let exit_code = child.wait()?.code().unwrap_or(1);

        xcb::map_window_checked(&conn, win).request_check()?;
        conn.flush();
        exit_code
    } else {
        // Wayland
        command(&args)?.wait()?.code().unwrap_or(1)
    };

    process::exit(exit_code);
}

fn command(args: &[String]) -> Result<process::Child, anyhow::Error> {
    let child = process::Command::new(&args[0])
        .args(&args[1..])
        .spawn()?;

    Ok(child)
}
