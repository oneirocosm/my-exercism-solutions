use failure::Error;
use std::collections::HashSet;
use std::fs;
use std::io::{BufRead, BufReader};

/// While using raw slice of str to handle flags is convenient,
/// in the real-world projects it is customary to use a struct,
/// that contains flags-related logic. So in this exercise
/// we ask you to implement a custom struct.
///
/// If you are curious about real-world implementation, refer to the `clap-rs` crate:
/// https://github.com/kbknapp/clap-rs/blob/master/src/args/arg_matches.rs
#[derive(Debug)]
pub struct Flags<'a> {
    in_use: HashSet<&'a str>,
}

impl<'a> Flags<'a> {
    pub fn new(flags: &'a [&str]) -> Self {
        Self {
            in_use: flags.into_iter().cloned().collect(),
        }
    }

    fn contains(&self, flag: &str) -> bool {
        self.in_use.contains(flag)
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let pattern = if flags.contains("-i") {
        pattern.to_uppercase()
    } else {
        pattern.to_string()
    };

    let num_files = files.len();

    let file_results = files
        .into_iter()
        .map(|filename| filter_file(filename, &pattern, flags))
        .collect::<Result<Vec<_>, Error>>()?;

    if flags.contains("-l") {
        Ok(file_results
            .into_iter()
            .filter_map(|file_result| file_result.get(0).map(|result| result.0.clone()))
            .collect::<Vec<_>>())
    } else {
        Ok(files
            .into_iter()
            .map(|filename| filter_file(filename, &pattern, flags))
            .collect::<Result<Vec<_>, Error>>()?
            .into_iter()
            .flatten()
            .map(|result| format_result(result, flags, num_files))
            .collect::<Vec<_>>())
    }
}

fn format_result(result: (String, usize, String), flags: &Flags, num_files: usize) -> String {
    let fname_fmt = if num_files > 1 {
        format!("{}:", result.0)
    } else {
        String::new()
    };

    let line_num_fmt = if flags.contains("-n") {
        format!("{}:", result.1)
    } else {
        String::new()
    };

    let line_fmt = if flags.contains("-l") {
        String::new()
    } else {
        result.2
    };

    format!("{}{}{}", fname_fmt, line_num_fmt, line_fmt)
}

fn filter_file(
    filename: &str,
    pattern: &String,
    flags: &Flags,
) -> Result<Vec<(String, usize, String)>, Error> {
    let file = fs::File::open(filename).or_else(|err| Err(Error::from(err)))?;
    let reader = BufReader::new(file);

    Ok(reader
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .or_else(|err| Err(Error::from(err)))?
        .into_iter()
        .enumerate()
        .filter_map(|(num, line)| {
            let cmp_line = if flags.contains("-i") {
                line.to_uppercase()
            } else {
                line.to_string()
            };

            let matches = if flags.contains("-x") {
                cmp_line == *pattern
            } else {
                cmp_line.contains(pattern)
            };

            if !flags.contains("-v") == matches {
                Some((filename.to_string(), num + 1, line))
            } else {
                None
            }
        })
        .collect::<Vec<_>>())
}
