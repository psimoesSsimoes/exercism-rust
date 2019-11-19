use failure::Error;
use std::fs;

#[derive(Debug)]
enum Flag {
    MatchLineNumbers,
    MatchFileNames,
    MatchCaseInsensitive,
    MatchInvert,
    MatchFull,
}

/// While using raw slice of str to handle flags is convenient,
/// in the real-world projects it is customary to use a struct,
/// that contains flags-related logic. So in this exercise
/// we ask you to implement a custom struct.
///
/// If you are curious about real-world implementation, refer to the `clap-rs` crate:
/// https://github.com/kbknapp/clap-rs/blob/master/src/args/arg_matches.rs
#[derive(Debug)]
pub struct Flags(Vec<Flag>);

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        Flags(
            flags
                .iter()
                .map(|x| match *x {
                    "-n" => crate::Flag::MatchLineNumbers,
                    "-l" => crate::Flag::MatchFileNames,
                    "-i" => crate::Flag::MatchCaseInsensitive,
                    "-v" => crate::Flag::MatchInvert,
                    "-x" => crate::Flag::MatchFull,
                    _ => crate::Flag::MatchFull,
                })
                .collect(),
        )
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut matches = Vec::new();
    files.into_iter().map(|filename| {
        fs::read_to_string(filename).map(|text| {
            let enumerated = text.lines().enumerate();

            for (_, line) in enumerated {
                if is_match(line, pattern, flags) {
                    matches.push(String::from(line));
                }
            }
        });
    });
    Ok(matches)
}
fn is_match(line: &str, pattern: &str, flags: &Flags) -> bool {
    let mut jury: Vec<bool> = Vec::new();
    for f in flags {
        match f {
            MatchFull => return,
        }
    }
}
