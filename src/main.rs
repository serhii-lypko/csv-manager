use crate::config::cli::parse_args;
use crate::file_reader::file_reader::read_file;
use crate::parser::parser::parse;

mod config;
mod file_reader;
mod parser;

use anyhow::Result;

// TODO: implement if the program should check header or not

/*
    Features:

    - essential:
        - parse csv file to rust structs
        - build header based of arg flag
        - handling different data types (requied for query tool and filters)
        - query tool and filters (simplest possible) -> use traits for filters
        - abstraction for different input sources (files, strings, streams) -> use traits

    - transformers:
        - csv2json
        - markdown?

    advanced:
        - merge two csv files
*/

fn main() -> Result<()> {
    let config = parse_args();
    let file_contents = read_file(config.file_path)?;

    let parse_res = parse(file_contents, config.with_header);

    Ok(())
}
