/// Represents a kind of diff line.  
/// The diff is split into 2 different parts:
///     Header information
///     File information
/// The problem is that this information is not split clearly.  The header is not constant and may
/// lack any part of the header or body.  
///
/// In this enum, all posible types of lines are sorted.  This is important even if the line is not
/// needed now, because if data is needed from a line later, we have the infromation labeled.
    /// The begining of a diff of the form: 'diff --git a/path/to/file b/path/tofile'
    /// Index information about a file: 'index 0123456..789abcd 100644'
    /// The path to the original file, the first file in the header: '--- a/path/to/file'
    /// The path to the new file, the second file in the header: '+++ b/path/to/file'
    /// Optional header to indicate that a files mode differs changed: 'new file mode 100644'
    /// The start of a file region: '@@ -888,12 +1002,33 @@ part of the file here'
    /// The indication that the file was deleted: 'deleted file mode 100644'
    /// A line of a file which is the same between the given files: ' always with a space'
    /// A line that exists in the new file, but not the original: '+the line of the file'
    /// A line that exists in the original file, but not the new: '-the line of the file'

impl DiffType {
    /// Returns if a type is part of the file body, or has file body parts to it.  
    /// This is useful as the slowest part of the process would be parcing the entire file for
    /// functions.
    pub fn is_file_body(&self) -> bool {
        match self {
            DiffType::FileLine | DiffType::Addition | DiffType::Subtraction |
            DiffType::NewRegion => true,
            _ => false,
        }
    }
}

/// Stateful Diff reading to avoid confusion over header and body.
/// Place all stateful information here.
///
/// TODO: If you would want to know how many functions came from a given file,
/// putting a current file calculation here would be how to do it.
    /// A new reader that should be started at the top of the file, however, it can start at any
    /// header line.
    /// A state machine that uses the guess of diff_type to create the actual type of a line and to
    /// fail when it is not sure.  
    ///
    /// Many just check that this line is expected and return it.  Some however take the guess, do
    /// a string compare to the minimum degree required and return the updated value.  
    ///
    /// # Returns
    ///
    /// A value of Some(data) is considered a correct responce from the function.  
    /// None is the universal error.  We also reset the state machine on error, and will continue
    /// to output correct responces at the next header. 
    ///
    /// A line to STDERR is sent indicating the error based on the string given and the current
    /// state of the machine.  
    ///
    /// # Note
    ///
    /// This section is messy and should be replaced with macros.
            eprintln!("Error: Diff file is not formated correctly.  Line \"{}\" after {:?} was unexpected.",