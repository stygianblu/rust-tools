extern crate clap;
extern crate termcolor;

mod constants;

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
            .help("Args of the function"))
        .arg(Arg::with_name("RETURN")
            .short("r")
            .long("return")
            .value_name("RETURN")
            .takes_value(true)
            .multiple(true)
            .help("Returns of the function"))
        .get_matches();

    let publish = matches.occurrences_of("PUBLISH");
    let publish = match publish {
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

    let mut args: Vec<&str> = Vec::new();
    let mut returns: Vec<&str> = Vec::new();
    if has_args { args = matches.values_of("ARGS").unwrap().collect(); }
    if has_rets { returns = matches.values_of("RETURN").unwrap().collect(); }

    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)));
    let _ = write!(&mut stdout, "/**\n{}\n\n### Parameters:\n", desc);
    for a in args.iter() {
        let _ = write!(&mut stdout, "* {} - *[TYPE]*: Description...\n", a);
    }
    let _ = write!(&mut stdout, "\n### Returns:\n");
    for r in returns.iter() {
        let _ = write!(&mut stdout, "* {}: Description...\n", r);
    }
    let _ = write!(&mut stdout, "*/\n");
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)));
    if publish {
        let _ = write!(&mut stdout, "pub ");
    }
    let _ = write!(&mut stdout, "fn ");
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue)));
    let _ = write!(&mut stdout, "{}(", name);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)));
    if method {
        let _ = write!(&mut stdout, "&self");
    }
    if has_args { 
        for a in args.iter() {
            let _ = write!(&mut stdout, ", ");
            let _ = write!(&mut stdout, "{}: ", a);
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)));
            let _ = write!(&mut stdout, "[TYPE]");
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)));
        }
    }
    let _ = write!(&mut stdout, ") ");
    if has_rets {
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)));
        let _ = write!(&mut stdout, "-> ");
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)));
        for r in returns.iter() {
            let _ = write!(&mut stdout, "{} ", r);
        }
    }
    let _ = write!(&mut stdout, "{{\n\t//TODO\n}}\n");
}
