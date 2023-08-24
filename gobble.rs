//Gobble by Sebastien MacDougall-Landry
//License is available at
//https://github.com/EmperorPenguin18/gobble/blob/main/LICENSE
extern crate xcb;

use std::{env, process};
use xcb::{x, Connection};

fn main() -> Result<(), anyhow::Error> {
    let mut args: Vec<String> = env::args().collect();
    let wayland = env::var_os("WAYLAND_DISPLAY");

    //Interpret flags
    let mut flag_overlap = false;
    let mut flag_version = false;
    let mut flag_help = false;
    while args.len() > 1 {
        if args[1].starts_with('-') {
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

    let exit_code = if flag_help {
        println!("gobble - hide your current window while using an external program");
        println!();
        println!("USAGE:");
        println!("  gobble [OPTIONS] CMD ...");
        println!();
        println!("OPTIONS:");
        println!();
        println!("  -h      Displays the help message you're seeing now");
        println!("  -v      Displays the software version");
        println!("  -o      Uses overlap mode");
        println!();
        println!("See the manual for more information");
        0
    } else if flag_version {
        println!("gobble v1.2");
        println!("See https://github.com/EmperorPenguin18/gobble/releases for more info");
        0
    } else if wayland.is_none() {
        gobble_on_x11(flag_overlap, &args)?
    } else {
        gobble_on_wayland(&args)?
    };

    process::exit(exit_code);
}

fn gobble_on_wayland(args: &[String]) -> Result<i32, anyhow::Error> {
    Ok(command(args)?.wait()?.code().unwrap_or(1))
}

fn gobble_on_x11(flag_overlap: bool, args: &[String]) -> Result<i32, anyhow::Error> {
    let (conn, screen_num) = Connection::connect(None)?;
    let parent_window = conn
        .wait_for_reply(conn.send_request(&x::GetInputFocus {}))?
        .focus();
    Ok(if flag_overlap {
        let mut child: process::Child;
        if args.len() > 1 {
            child = command(args)?;
        } else {
            process::exit(0);
        }

        let translate = conn.wait_for_reply(
            conn.send_request(&x::TranslateCoordinates {
                src_window: parent_window,
                dst_window: conn
                    .get_setup()
                    .roots()
                    .nth(screen_num as usize)
                    .unwrap()
                    .root(),
                src_x: 0,
                src_y: 0,
            }),
        )?; //Translates relative position to absolute position
        let geometry = conn.wait_for_reply(conn.send_request(&x::GetGeometry {
            drawable: x::Drawable::Window(parent_window),
        }))?;
        let values = [
            x::ConfigWindow::X(i32::from(translate.dst_x())),
            x::ConfigWindow::Y(i32::from(translate.dst_y())),
            x::ConfigWindow::Width(geometry.width().into()),
            x::ConfigWindow::Height(geometry.height().into()),
        ];

        let mut child_window = conn
            .wait_for_reply(conn.send_request(&x::GetInputFocus {}))?
            .focus();
        while parent_window == child_window {
            child_window = conn
                .wait_for_reply(conn.send_request(&x::GetInputFocus {}))?
                .focus();
        }
        conn.send_request_checked(&x::ConfigureWindow {
            window: child_window,
            value_list: &values,
        });

        child.wait()?.code().unwrap_or(1)
    } else {
        // ensure child was spawned before we hide the window
        let mut child: process::Child;
        if args.len() > 1 {
            child = command(args)?;
        } else {
            process::exit(0);
        }

        let unmap_attempt = conn.send_request_checked(&x::UnmapWindow {
            window: parent_window,
        });
        conn.check_request(unmap_attempt)?;

        let exit_code = child.wait()?.code().unwrap_or(1);

        let map_attempt = conn.send_request_checked(&x::MapWindow {
            window: parent_window,
        });
        println!("Un-Gobbling.. Bye!");
        conn.check_request(map_attempt)?;

        exit_code
    })
}

fn command(args: &[String]) -> Result<process::Child, anyhow::Error> {
    let child = process::Command::new(&args[1]).args(&args[2..]).spawn()?;

    Ok(child)
}
