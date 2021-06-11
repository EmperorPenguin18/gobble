//Gobble by Sebastien MacDougall-Landry
//License is available at
//https://github.com/EmperorPenguin18/gobble/blob/main/LICENSE

extern crate xcb;
use std::{env, process, thread, time};

fn main() {
    let (conn, _screen_num) = xcb::Connection::connect(None).unwrap();
    let win = xcb::get_input_focus(&conn).get_reply().unwrap().focus();
    let args: Vec = env::args().collect();
    let mut command = process::Command::new(&args[1]);
    command.args(&args[2 .. args.len()]);

    xcb::unmap_window(&conn, win);
    conn.flush();
    let stat: i32 = command.status().unwrap().code().unwrap();
    xcb::map_window(&conn, win);
    conn.flush();

    thread::sleep(time::Duration::from_millis(10));
    //Makes cli commands work

    process::exit(stat);
}
