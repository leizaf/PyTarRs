use std::fs::File;
use std::io::{Read, Result};
use tar::{Archive, Entries, Entry};
use crate::sample::SampleFile;


trait ReadEntry<R: Read> {
    fn read(self) -> Result<SampleFile>;
}


impl<R: Read> ReadEntry<R> for Entry<'_, R> {
    fn read(mut self) -> Result<SampleFile> {
        let mut data = Vec::new();
        let path = self.path()?.to_str().unwrap().to_string();
        self.read_to_end(&mut data)?;
        Ok(SampleFile::new(path, data))
    }
}


pub struct TarReader<'a, File: std::io::Read> {
    pub entries: Entries<'a, File>,
    archive_ptr: *mut Archive<File>
}


impl TarReader<'_, File> {
    pub fn new(file: File) -> Self {
        let archive = Box::new(Archive::new(file));
        let archive_ptr = Box::into_raw(archive);
        let ptr_copy = archive_ptr.clone();
        unsafe {
            let entries = archive_ptr.as_mut().unwrap().entries().unwrap();
            TarReader { entries, archive_ptr: ptr_copy}
        }
    }

    fn close(&self) {
        let archive_ptr = self.archive_ptr;
        unsafe {
            let _ = Box::from_raw(archive_ptr);
        }
    }
}


impl Iterator for TarReader<'_, File> {
    type Item = Result<SampleFile>;

    fn next(&mut self) -> Option<Self::Item> {
        self.entries.next().map_or_else(
            || {
            self.close();
            None
        },
        |entry| {
            match entry {
                Ok(entry) => Some(entry.read()),
                Err(e) => Some(Err(e))
            }
        }
        )
    }
}