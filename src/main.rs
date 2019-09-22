use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use ralf::elfheader::ElfHeader;
use ralf::objdump::objdump;
use ralf::strings::strings;
use std::fs::File;
use std::io::Read;

const MIN_LEN: u32 = 4;

fn main() {
    let matches = App::new("Ralf")
        .author("Markus Akesson")
        .about("Program to handle binary files")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("readelf")
                .about("Reads a ELF header")
                .setting(AppSettings::ArgRequiredElseHelp)
                .arg(Arg::with_name("file").required(true).takes_value(true)),
        )
        .subcommand(
            SubCommand::with_name("strings")
                .about("Prints strings found in the input file")
                .setting(AppSettings::ArgRequiredElseHelp)
                .arg(Arg::with_name("file").required(true).takes_value(true))
                .arg(
                    Arg::with_name("bytes")
                        .long("bytes")
                        .short("n")
                        .required(false)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("objdump")
                .about("prints the binarys instructions")
                .setting(AppSettings::ArgRequiredElseHelp)
                .arg(
                    Arg::with_name("file")
                        .long("file")
                        .short("f")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("readelf", Some(args)) => run_readelf(args),
        ("strings", Some(args)) => run_strings(args),
        ("objdump", Some(args)) => run_objdump(args),
        _ => unreachable!(),
    }
}

fn run_readelf(args: &ArgMatches) {
    let path = args.value_of("file").unwrap();
    let mut buffer = Vec::new();
    read_from_file(path, &mut buffer);
    let header = ElfHeader::new(&buffer).unwrap();
    println!("{}", header);
}

fn run_strings(args: &ArgMatches) {
    let path = args.value_of("file").unwrap();
    let mut buffer = Vec::new();
    read_from_file(path, &mut buffer);

    let min_len = match args.value_of("bytes") {
        Some(len) => len
            .parse::<u32>()
            .expect("Argument for bytes must be an integer"),
        None => MIN_LEN,
    };

    strings(min_len, &buffer);
}

fn run_objdump(args: &ArgMatches) {
    let path = args.value_of("file").unwrap();
    let mut buffer = Vec::new();
    read_from_file(path, &mut buffer);
    objdump(&buffer);
}

fn read_from_file(path: &str, buffer: &mut Vec<u8>) {
    let mut file = File::open(path).expect("Failed to open file");
    file.read_to_end(buffer).expect("Error reading from file");
}
