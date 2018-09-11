extern crate clap;
extern crate termcolor;

mod constants;

use std::process;
use clap::{Arg, App};
use self::termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::io::Write;

fn main() {
    
    let matches = App::new(constants::NAME)
        .version(constants::VERSION)
        .author(constants::AUTHOR)
        .about(constants::DESCRIPTION)
        .arg(Arg::with_name("PUBLISH")
            .short("p")
            .long("publish")
            .value_name("PUBLISH")
            .takes_value(false)
            .help("Function should be published"))
        .arg(Arg::with_name("NAME")
            .short("n")
            .long("name")
            .value_name("NAME")
            .takes_value(true)
            .help("Name of the function"))
        .arg(Arg::with_name("METHOD")
            .short("m")
            .long("method")
            .value_name("METHOD")
            .takes_value(false)
            .help("Is a method"))
        .arg(Arg::with_name("DESCRIPTION")
           .short("d")
           .long("description")
           .value_name("DESCRIPTION")
           .takes_value(true)
           .help("Description of the function"))
        .arg(Arg::with_name("ARGS")
            .short("a")
            .long("args")
            .value_name("ARGS")
            .takes_value(true)
            .multiple(true)
            .help("Args of the function in the form ARG:TYPE"))
        .arg(Arg::with_name("RETURN")
            .short("r")
            .long("return")
            .value_name("RETURN")
            .takes_value(true)
            .multiple(true)
            .help("Returns of the function"))
		.arg(Arg::with_name("IDENT")
            .short("i")
            .long("indent")
            .value_name("IDENT")
            .takes_value(true)
            .help("Tabs to indent stub, defaults to none"))
        .arg(Arg::with_name("COMMENTS")
            .short("c")
            .long("comments")
            .value_name("COMMENTS")
            .takes_value(false)
            .help("Only the comment block needed"))
        .get_matches();

    let publish = matches.occurrences_of("PUBLISH");
    let publish = match publish {
        0 => false,
        1 | _ => true,
    };
	let comments = matches.occurrences_of("COMMENTS");
	let comments = match comments {
		0 => false,
		1 | _ => true,
	};
    let name = matches.value_of("NAME");
    let name = match name {
        Some(v) => v,
        None => constants::DEFAULT_NAME,
    };
    let method = matches.occurrences_of("METHOD");
    let method = match method {
        0 => false,
        1 | _ => true,
    };
    let desc = matches.value_of("DESCRIPTION");
    let desc = match desc {
        Some(v) => v,
        None => constants::DEFAULT_DESCRIPTION,
    };
    let has_args = matches.occurrences_of("ARGS");
    let has_args = match has_args {
        0 => false,
        1 | _ => true,
    };
    let has_rets = matches.occurrences_of("RETURN");
    let has_rets = match has_rets {
        0 => false,
        1 | _ => true,
    };
    let has_indent = matches.occurrences_of("IDENT");
    let has_indent = match has_indent {
        0 => false,
        1 | _ => true,
    };
	let mut TABS = String::from("");
	if has_indent {
		let num_tabs = matches.value_of("IDENT");
		let _ = match num_tabs {
			Some(v) => {
				let num: i32 = v.parse().unwrap();
				for i in 0..num {
					TABS.push_str("\t");
				}
			},
			None => TABS = String::from(""),
		};
	}

    let mut args: Vec<&str> = Vec::new();
    let mut returns: Vec<&str> = Vec::new();
    if has_args { args = matches.values_of("ARGS").unwrap().collect(); }
    if has_rets { returns = matches.values_of("RETURN").unwrap().collect(); }

    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)));
    let _ = write!(&mut stdout, "{}/**\n{}* {}\n{}*\n{}* ### Parameters:\n", TABS, TABS, desc, TABS, TABS);
	
	let mut arg_type: Vec<&str>;
    for a in args.iter() {
		arg_type = a.splitn(2, ":").collect();
        let _ = write!(&mut stdout, "{}* {} - *{}*: Description...\n", TABS, arg_type[0], arg_type[1]);
    }
    let _ = write!(&mut stdout, "{}*\n{}* ### Returns:\n", TABS, TABS);
    for r in returns.iter() {
        let _ = write!(&mut stdout, "{}* {}: Description...\n", TABS, r);
    }
    let _ = write!(&mut stdout, "{}*/\n", TABS);

	//If -COMMENTS flag present then only return the comment block
	if comments {
		process::exit(0);
	}

	let _ = write!(&mut stdout, "{}", TABS);	

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)));
    if publish {
        let _ = write!(&mut stdout, "pub ");
    }
    let _ = write!(&mut stdout, "fn ");
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue)));
    let _ = write!(&mut stdout, "{}(", name);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)));

	let mut arg_line = String::from("");

    if method {
		arg_line.push_str("&self, ");
    }
    if has_args {
		let mut split: Vec<&str>;
        for a in args.iter() {
			split = a.splitn(2, ":").collect();
			arg_line.push_str(format!("{}: {}, ", split[0], split[1]).as_str());
        }
    }

	//Remove the last " ,"
	let tmp_len = arg_line.len();
	arg_line.truncate(tmp_len - 2);
	let _ = write!(&mut stdout, "{}", arg_line);

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue)));
    let _ = write!(&mut stdout, ") ");
    if has_rets {
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)));
        let _ = write!(&mut stdout, "-> ");
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)));

		let mut return_line = String::from("");
		if returns.len() > 1 { return_line.push_str("("); }

        for r in returns.iter() {
			return_line.push_str(format!("{}", r).as_str());
			if returns.len() > 1 { return_line.push_str(", "); }
        }
		if returns.len() > 1 { 
			let tmp_len = return_line.len();
			return_line.truncate(tmp_len - 2);	
			return_line.push_str(")");
		}
	
		let _ = write!(&mut stdout, "{}", return_line);
    }
    let _ = write!(&mut stdout, " {{\n{}\t//TODO\n{}}}\n", TABS, TABS);
}
