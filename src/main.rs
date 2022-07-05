mod process;

use clap::{App, Arg};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("seqsplitter")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Steven Weaver")
        .about("Splits large FASTA file based on list")
        .arg(
            Arg::new("fasta")
                .help("The input FASTA file (gzip acceptable).")
                .takes_value(true)
                .required(true)
                .short('f')
                .long("fasta"),

        )
        .arg(
            Arg::new("list")
                .help("The id list.")
                .takes_value(true)
                .required(true)
                .short('l')
                .long("list"),

        )
        .arg(
            Arg::with_name("regex")
                .long("regex")
                .required(false)
                .takes_value(false)
                .short('r')
                .help("interpret list as regex"),
        )
        .arg(
            Arg::with_name("assume_unique")
                .long("assume_unique")
                .required(false)
                .takes_value(false)
                .short('u')
                .help("assume list is unique and exit when all seqs in list are found"),
        )
        .get_matches();

    crate::process::process(matches.value_of("fasta").unwrap(), matches.value_of("list").unwrap(), matches.is_present("regex"), matches.is_present("assume_unique"))
}
