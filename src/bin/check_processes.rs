extern crate clap;
extern crate termcolor;
extern crate procfs;

mod constants;

use std::path::Path;
use clap::{Arg, App};
use self::termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::io::Write;
use procfs::Process;

fn main() {
    
    let matches = App::new(constants::NAME)
        .version(constants::VERSION)
        .author(constants::AUTHOR)
        .about(constants::DESCRIPTION)
        .arg(Arg::with_name("PID")
            .short("p")
            .long("pid")
            .value_name("PID")
            .takes_value(true)
            .multiple(true)
            .required(true)
            .help("pid values of processes to check"))
        .get_matches();

    //Collect the pids
    let pids: Vec<_> = matches.values_of("PID").unwrap().collect();

    //Write header
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(0, 255, 255))));
    let _ = write!(&mut stdout, "My PIDSs:\n");

    //Iterate over pids
    for p in pids {

        //Write the cuurent pid
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(0, 255, 0))));
        let _ = write!(&mut stdout, "--> [{}]", p);

        //Try to create a proccess from the cuurent pid
        let pid_string = p.to_string();
        let pid_i32: i32 = pid_string.parse().unwrap();
        let process_res: bool = match Process::new(pid_i32) {
            procfs::ProcResult::NotFound => { false },
            procfs::ProcResult::PermissionDenied => { false },
            _ => { true },  
        };
        if !process_res {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(255, 0, 0))));
            let _ = write!(&mut stdout, " \t\t[NotFound / PermissionDenied]\n");
            continue;
        }
        let _ = write!(&mut stdout, "\n");
        let process: Process = Process::new(pid_i32).unwrap();

        //Print exe info
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(0, 255, 0))));
        let _ = write!(&mut stdout, "|");
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(255, 51, 204))));
        let mut exe_path = String::from("");
        let exe = process.exe();
        exe_path = match exe {
            procfs::ProcResult::NotFound => String::from("NotFound"),
            procfs::ProcResult::PermissionDenied => String::from("PermissionDenied"),
            _ => String::from(""),
        };
        if !exe_path.contains("d") { 
            exe_path = match exe.unwrap().to_str() {
                Some(path) => String::from(path),
                _ => String::from(""),
            };
        }
        let _ = write!(&mut stdout, "\texe: \t\t");
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(255, 255, 255))));
        let _ = write!(&mut stdout, "{}\n", exe_path);

        //Print out cmdline info
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(0, 255, 0))));
        let _ = write!(&mut stdout, "|");
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(255, 51, 204))));
        let cmdline = process.cmdline().unwrap();
        let _ = write!(&mut stdout, "\tcmdline: \t");
        for c in cmdline.iter() {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(255, 255, 255))));
            let _ = write!(&mut stdout, "{} ", c);
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(255, 51, 204))));
        }
        let _ = write!(&mut stdout, "\n");

        //Print out if it is alive
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(0, 255, 0))));
        let _ = write!(&mut stdout, "|");
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(255, 51, 204))));
        let _ = write!(&mut stdout, "\talive: \t\t");
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(255, 255, 255))));
        let _ = write!(&mut stdout, "{}\n", process.is_alive());

    }
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)));
}
