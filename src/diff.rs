use result::Result; // Result directly is our result.
use std::collections::{HashMap, HashSet};
use std::fs::read;
use std::path::Path;

/// Given a file, return statistical information about the file as if it were a diff file.
/// Returns Some Result if the file could be read and None if the reading failed.
///
/// # Arguments
///
/// `file` - The path to the file to be analized
///
/// # Advice
///
/// This will be the main function per file.
/// It is adviced to create a handler around this as this function is not taking exclusive
/// rights to the path.  The path should be secured by the handler for the sake of returning
/// an error message if the path was invalid and no data could be read.
pub fn diffStats(file: &Path) -> Option<Result> {
    if let Ok(file_data) = read(file) {
        let mut retVal = Result::new(HashSet::new(), 0, 0, 0, HashMap::new());
        let file_data = match String::from_utf8(file_data) {
            Ok(data) => data,
            Err(_) => {
                eprintln!("Error: Could not encode file as utf8");
                return None;
            }
        };
        file_data
            .lines()
            .for_each(|line| {
                match diff_type(line) {
                    DiffType::Header => {}
                    DiffType::Index => {}
                    DiffType::OriginalFile => {}
                    DiffType::NewFile => {}
                    DiffType::NewRegion => retVal.add_region(),
                    DiffType::FileLine => {}
                    DiffType::Addition => {}
                    DiffType::Subtraction => {}
                };
            });

        Some(retVal)
    } else {
        None // During file error, we simply return nothing to indicate that the file has no contents instead of valid contents with nothing in it.
    }
}


enum DiffType {
    Header,
    Index,
    OriginalFile,
    NewFile,
    NewRegion,
    FileLine,
    Addition,
    Subtraction,
}

/// Returns the diff status of a line.  There are a few states, including the different header
/// lines, normal file lines, additions, and subtractions.
///
/// This will do the minimum work required to figure out what type a line is because it assumes the
/// string slice it is given starts at the begining of the line and that the file is a valid diff
/// file.  Any unknown first letter of the line is treated as FileLine, as are empty lines.
///
/// # Known Issues
///
/// Currently, OriginalFile and NewFile are treeted as Addition and Subtraction lines.
/// This is an issue of only using the first letter.  While it could be improved, there are edge
/// cases that cannot be resolved such as an additon line adding a line with 2 or more '+'
/// characters.  To avoid this, this system uses a blind approch that is not attempting to solve
/// this issue.
///
/// # Arguments
///
/// `line` - A string slice of a line of a diff file.
///
/// # Example
/// ```
/// assert!(diff_type("diff --git ...") == DiffType::Header)
fn diff_type(line: &str) -> DiffType {
    match line.chars().next().unwrap_or(' ') {
        'd' => DiffType::Header,
        'i' => DiffType::Index,
        '@' => DiffType::NewRegion,
        '+' => DiffType::Addition,
        '-' => DiffType::Subtraction,
        ' ' => DiffType::FileLine,
        _ => DiffType::FileLine,
    }
}



#[cfg(test)]
mod tests {
    use std::path::Path;

    use result::Result;
    use super::diffStats;

    #[test]
    fn invaldFile() {
        let path = Path::new("/INVALID");
        assert!(match diffStats(&path) {
                    None => true,
                    _ => false,
                });
    }

    #[test]
    fn emptyFile() {
        use std::fs::{write, remove_file};
        let filename = "test.tmp"; // We create an empty file just to be sure it exists.
        write(&filename, "");
        let path = Path::new(&filename);
        assert!(match diffStats(&path) {
                    Some(_) => true,
                    _ => false,
                });
        // TODO implementing PartialEq on result to improve this test to the next line.
        // assert!(match diffStats(&path){ Some(data) => data == Result::empty(), _ => false});
        remove_file(&filename); // Cleanup that file so we don't keep creating different files.

        // TODO crate tempfile should be introduced in test setup to improve this test by
        // removing the issue with creating and removing a file.
    }
}
