use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::fs::OpenOptions;

use clap::{arg, command, value_parser, ArgAction, Arg};

fn main() {
    let command = command!()
        .arg(arg!([note] "Note to save"))
        .arg(arg!(-r --remove <NUMBER> "Remove note with specified number (0 means all)")
        .required(false)
        .value_parser(value_parser!(usize)),)
        .arg(
            Arg::new("list")
            .short('l')
            .long("list")
            .action(ArgAction::SetTrue)
        )
        .get_matches();

    let home = match std::env::var("HOME") {
        Err(err) => panic!("Couldn't find home directory: {}", err),
        Ok(home) => home,
    };
    let pathstr = home + "/.noted";
    let path = Path::new(&pathstr);
    let display = path.display();

    // list
    if command.get_flag("list") {
        let mut file = match OpenOptions::new().write(false).read(true).open(path) {
            Err(_) => match File::create(path) {
                Err(err) => panic!("Couldn't create file {}: {}", display, err),
                Ok(file) => file,
            },
            Ok(file) => file,
        };

        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(err) => panic!("Couldn't read {}: {}", display, err),
            Ok(_) => (),
        }

        for (idx, line) in s.lines().enumerate() {
            println!("{}.\t{}", idx + 1, line);
        }

        return;
    }
    
    // remove
    if let Some(rid) = command.get_one::<usize>("remove") {
        let mut file = match OpenOptions::new().write(true).read(true).open(path) {
            Err(_) => match File::create(path) {
                Err(err) => panic!("Couldn't create file {}: {}", display, err),
                Ok(file) => file,
            },
            Ok(file) => file,
        };

        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(_) => s = String::from(""),
            Ok(_) => (),
        }

        file = match File::create(path) {
            Err(err) => panic!("Couldn't create file {}: {}", display, err),
            Ok(file) => file,
        };

        if *rid == 0 {
            s = String::from("");
            match file.write_all(s.as_bytes()) {
                Err(err) => panic!("Couldn't write to file {}: {}", display, err),
                Ok(_) => (),
            }
            return;
        }

        s = s.lines().take(*rid - 1)
            .map(|line| String::from(line) + "\n").collect::<Vec<String>>().concat()
            + &s.lines().skip(*rid)
            .map(|line| String::from(line) + "\n").collect::<Vec<String>>().concat();

        match file.write_all(s.as_bytes()) {
            Err(err) => panic!("Couldn't write to file {}: {}", display, err),
            Ok(_) => (),
        }

        return;
    }

    if let Some(note) = command.get_one::<String>("note") {
        let mut file = match OpenOptions::new().write(true).read(true).open(path) {
            Err(_) => match File::create(path) {
                Err(err) => panic!("Couldn't create file {}: {}", display, err),
                Ok(file) => file,
            },
            Ok(file) => file,
        };

        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(_) => s = String::from(""),
            Ok(_) => (),
        }

        file = match File::create(path) {
            Err(err) => panic!("Couldn't create file {}: {}", display, err),
            Ok(file) => file,
        };

        s.push_str(&format!("{}\n", note));
        match file.write_all(s.as_bytes()) {
            Err(err) => panic!("Couldn't write to file {}: {}", display, err),
            Ok(_) => ()
        }

        return;
    }
}
