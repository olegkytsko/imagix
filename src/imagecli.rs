mod imagix;
use ::imagix::error::ImagixError;
use ::imagix::resize::{process_resize_request, Mode, SizeOption};
use ::imagix::stats::get_stats;
use std::path::PathBuf;
use std::str::FromStr;
use std::process;
use clap::{Command, Arg, ArgAction};

fn cli() -> Command {
    Command::new("imagecli")
        .about("This is a tool for image resizing and stats")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("resize")
                .about("Resizes provided image(-s)")
                .long_about("Specify size(small/medium/large), mode(single/all) and srcfolder")
                .arg(Arg::new("size")
                    .long("size")
                    .value_parser(clap::builder::NonEmptyStringValueParser::new())
                    .required(true)
                    .action(ArgAction::Set))
                .arg(Arg::new("mode")
                    .long("mode")
                    .value_parser(clap::builder::NonEmptyStringValueParser::new())
                    .required(true)
                    .action(ArgAction::Set))
                .arg(Arg::new("srcfolder")
                    .long("srcfolder")
                    .value_parser(clap::builder::NonEmptyStringValueParser::new())
                    .required(true)
                    .action(ArgAction::Set))
        )
        .subcommand(
            Command::new("stats")
                .about("Provides statistics on image(-s)")
                .arg(Arg::new("srcfolder")
                    .long("srcfolder")
                    .required(true)
                    .action(ArgAction::Set))
        )
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("resize", sub_matches)) => {
            let size = match SizeOption::from_str(sub_matches.get_one::<String>("size").unwrap()) {
                Ok(s) => s,
                Err(e) => {
                    match e {
                        ImagixError::UserInputError(err_msg) => {
                            eprintln!("{}", err_msg);
                            process::exit(exitcode::USAGE);   
                        },
                        _ => {
                            eprintln!("{}","An unknown error occurred".to_string());
                            process::exit(exitcode::USAGE);   
                        },
                    }
                }
            };
            
            let mode = match Mode::from_str(sub_matches.get_one::<String>("mode").unwrap()) {
                Ok(m) => m,
                Err(e) => {
                    match e {
                        ImagixError::UserInputError(err_msg) => {
                            eprintln!("{}", err_msg);
                            process::exit(exitcode::USAGE);   
                        },
                        _ => {
                            eprintln!("{}","An unknown error occurred".to_string());
                            process::exit(exitcode::USAGE);   
                        },
                    }
                }
            };
            
            let mut srcfolder = PathBuf::from(
                sub_matches.get_one::<String>("srcfolder").unwrap()
            );
            
            match process_resize_request(size, mode, &mut srcfolder) {
                Ok(_) => {
                    println!("Image resized successfully");
                    process::exit(exitcode::OK);
                },
                Err(e) => match e {
                    ImagixError::FileIOError(e) => {
                        eprintln!("{}", e);
                        process::exit(exitcode::USAGE);
                    },
                    ImagixError::UserInputError(e) => {
                        eprintln!("{}", e);
                        process::exit(exitcode::USAGE);
                    },
                    ImagixError::ImageResizingError(e) => {
                        eprintln!("{}", e);
                        process::exit(exitcode::USAGE);
                    },
                    _ => {
                        eprintln!("Error in processing: {}", e);
                        process::exit(exitcode::USAGE);
                    }
                }
            }
        },
        Some(("stats", sub_matches)) => {
            let srcfolder = PathBuf::from(
                sub_matches.get_one::<String>("srcfolder").unwrap()
            );
            match get_stats(srcfolder) {
                Ok((count, size)) => println!(
                    "Found {:?} image files with aggregate size of {:?} MB", count, size),
                Err(e) => match e {
                    ImagixError::FileIOError(e) => println!("{}", e),
                    ImagixError::UserInputError(e) => println!("{}", e),
                    _ => println!("Error in processing: {}", e),
                }
            }
        },
        _ => unreachable!(),
    }
    
    
}