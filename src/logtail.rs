use std::io;
use std::fs::File;
use std::path::Path;

/// Iterates over a file line by line, blocking until new lines are appended.
pub struct LogTail<'a> {
    path: &'a Path,
    file: Option<File>
}

impl<'a> From<&'a Path> for LogTail<'a> {
    fn from(path: &'a Path) -> LogTail<'a> {
        LogTail {
            path: path.as_ref(),
            file: None
        }
    }
}

impl<'a> Iterator for LogTail<'a> {
    type Item = io::Result<String>;

    fn next(&mut self) -> Option<io::Result<String>> {
        if self.file.is_none() {
            match File::open(self.path) {
                Ok(f) => { self.file = Some(f); }
                Err(e) => { return Some(Err(e)); }
            }
            unimplemented!(); //TODO seek to current line index
        }
        unimplemented!(); //TODO yield next line
    }
}
