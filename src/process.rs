use bio::io::fasta;
use regex::RegexSet;
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::Path;

// REGEX Option
pub fn regex_match<P: AsRef<Path> + AsRef<OsStr>>(
    filename: P,
    list: P,
    assume_unique: bool,
) -> bool {
    //FASTA file related
    let file = Path::new(&filename).to_str().unwrap();
    let mut records = fasta::Reader::from_file(file).unwrap().records();
    let mut writer = fasta::Writer::new(io::stdout());

    // Search items
    let search_ids_to_parse =
        fs::read_to_string(list).expect("Something went wrong reading the file");
    let mut search_id_vector: Vec<&str> = search_ids_to_parse
        .split('\n')
        .into_iter()
        .filter(|id| !id.is_empty())
        .collect();
    let id_regex_set = RegexSet::new(&search_id_vector).unwrap();

    // Gather data from every record
    while let Some(record) = records.next() {
        let seqrec = record.unwrap();
        let sequence_id_bytes = seqrec.id();
        let matches: bool = id_regex_set.matches(&sequence_id_bytes).matched_any();

        // if match add seqrec to vector
        if matches == true {
            // Write to FASTA file
            // writer.write("random", None, seq.as_slice()).expect("Error writing record.");
            writer
                .write(sequence_id_bytes, None, seqrec.seq())
                .expect("Error writing record.");

            // If assume_unique is turned on, remove search id from vector and exit if empty
            if assume_unique == true {
                search_id_vector.retain(|&x| x != sequence_id_bytes);
                if search_id_vector.len() == 0 {
                    return true;
                }
            }
        }
    }

    return true;
}

// String Match Option
pub fn string_match<P: AsRef<Path> + AsRef<OsStr>>(
    filename: P,
    list: P,
    assume_unique: bool,
) -> bool {
    //FASTA file related
    let file = Path::new(&filename).to_str().unwrap();
    let mut records = fasta::Reader::from_file(file).unwrap().records();
    let mut writer = fasta::Writer::new(io::stdout());

    // Search items
    let search_ids_to_parse =
        fs::read_to_string(list).expect("Something went wrong reading the file");
    let mut search_id_vector: Vec<&str> = search_ids_to_parse
        .split('\n')
        .into_iter()
        .filter(|id| !id.is_empty())
        .collect();

    // Gather data from every record
    while let Some(record) = records.next() {
        let seqrec = record.unwrap();

        // Need to concatenate the description and header

        let sequence_id_bytes = seqrec.id();
        let sequence_description_bytes = seqrec.desc().unwrap_or("");

        let entire_header_raw = [sequence_id_bytes, sequence_description_bytes].join(" ");
        let entire_header = entire_header_raw.trim();
        //println!("{0}", entire_header);

        let matches: bool = search_id_vector.iter().any(|&x| x == entire_header);

        // if match add seqrec to vector
        if matches == true {
            // Write to FASTA file
            // writer.write("random", None, seq.as_slice()).expect("Error writing record.");
            writer
                .write(&entire_header, None, seqrec.seq())
                .expect("Error writing record.");

            // If assume_unique is turned on, remove search id from vector and exit if empty
            if assume_unique == true {
                search_id_vector.retain(|&x| x != entire_header);
                if search_id_vector.len() == 0 {
                    return true;
                }
            }
        }
    }

    return true;
}

pub(crate) fn process<P: AsRef<Path> + AsRef<OsStr>>(
    filename: P,
    list: P,
    use_regex: bool,
    assume_unique: bool,
) -> Result<(), Box<dyn Error>> {
    if use_regex {
        regex_match(filename, list, assume_unique)
    } else {
        string_match(filename, list, assume_unique)
    };

    Ok(())
}
