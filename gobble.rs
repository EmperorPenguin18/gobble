//Gobble by Sebastien MacDougall-Landry
//License is available at
//https://github.com/EmperorPenguin18/gobble/blob/main/LICENSE

use std::{env, process};

fn main() -> Result<(), anyhow::Error> {
    let mut args: Vec<String> = env::args().collect();
    let wayland = env::var_os("WAYLAND_DISPLAY");

    //Interpret flags
    let mut flag_overlap = false;
    let mut flag_version = false;
    let mut flag_help = false;
    while args.len() > 1 {
        if args[1].chars().nth(0).unwrap() == '-' {
            if args[1].chars().nth(1).unwrap() == 'o' {
                flag_overlap = true;
            } else if args[1].chars().nth(1).unwrap() == 'v' {
                flag_version = true;
            } else {
                flag_help = true;
            }
        } else {
            break;
        }
        args.remove(1);
    }

    let exit_code = if flag_help == true {
        println!("gobble - hide your current window while using an external program");
        println!("");
        println!("USAGE:");
        println!("  gobble [OPTIONS] CMD ...");
        println!("");
        println!("OPTIONS:");
        println!("");
        println!("  -h      Displays the help message you're seeing now");
        println!("  -v      Displays the software version");
        println!("  -o      Uses overlap mode");
        println!("");
        println!("See the manual for more information");
        0
    } else if flag_version == true {
        println!("gobble v1.2");
        println!("See https://github.com/EmperorPenguin18/gobble/releases for more info");
        0
    } else if wayland == None {
        // X11
        let (conn, screen_num) = xcb::Connection::connect(None)?;
        let win = xcb::get_input_focus(&conn).get_reply()?.focus();
        if flag_overlap == false {
            // ensure child was spawned before we hide the window
            let mut child: process::Child;
            if args.len() > 1 {
                child = command(&args)?;
            } else {
                process::exit(0);
            }
            
            xcb::unmap_window_checked(&conn, win).request_check()?;
            conn.flush();

            let exit_code = child.wait()?.code().unwrap_or(1);

            xcb::map_window_checked(&conn, win).request_check()?;
            conn.flush();
            exit_code
        } else {
            let mut child: process::Child;
            if args.len() > 1 {
                child = command(&args)?;
            } else {
                process::exit(0);
            }

            let translate = xcb::translate_coordinates(
                &conn,
                win,
                conn.get_setup()
                    .roots()
                    .nth(screen_num as usize)
                    .unwrap()
                    .root(),
                0,
                0,
            )
            .get_reply()?; //Translates relative position to absolute position
            let geometry = xcb::get_geometry(&conn, win).get_reply()?;
            let values = &vec![
                (xcb::CONFIG_WINDOW_X as u16, translate.dst_x() as u32),
                (xcb::CONFIG_WINDOW_Y as u16, translate.dst_y() as u32),
                (xcb::CONFIG_WINDOW_WIDTH as u16, geometry.width() as u32),
                (xcb::CONFIG_WINDOW_HEIGHT as u16, geometry.height() as u32),
            ];

            let mut newwin = xcb::get_input_focus(&conn).get_reply()?.focus();
            while win == newwin {
                newwin = xcb::get_input_focus(&conn).get_reply()?.focus();
            }
            xcb::configure_window_checked(&conn, newwin, values).request_check()?;

            child.wait()?.code().unwrap_or(1)
        }
    } else {
        // Wayland
        command(&args)?.wait()?.code().unwrap_or(1)
    };
    
    process::exit(exit_code);
}

fn command(args: &[String]) -> Result<process::Child, anyhow::Error> {
    let child = process::Command::new(&args[1])
        .args(&args[2..])
        .spawn()?;
    
    Ok(child)
}
