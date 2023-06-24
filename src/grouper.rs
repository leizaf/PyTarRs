use std::io::{Result, Error};
use crate::sample::{Sample, SampleFile};


pub struct GroupBy<I: Iterator<Item = Result<SampleFile>>> {
    inner: I,
    buffer: Option<I::Item>,
    done: bool,
}


impl<I: Iterator<Item = Result<SampleFile>>> GroupBy<I> {
    pub fn new(inner: I) -> Self {
        GroupBy {
            inner,
            buffer: None,
            done: false,
        }
    }

    fn get_next_file(&mut self) -> Option<Result<SampleFile>> {
        match self.buffer.take() {
            Some(value) => Some(value),
            None => self.inner.next(),
        }
    }

    fn collection_loop(&mut self, mut sample: Sample) -> Result<Sample> {
        let mut res: Option<Error> = None;

        loop {
            if let Some(next) = self.get_next_file() {
                let next = match next {
                    Err(err) => { res = Some(err); continue; }
                    Ok(next) => { next }
                };
                let (key, file_name) = next.path_info();
                if key != sample.key {
                    self.buffer = Some(Ok(next));
                    break;
                }
                sample.update(&file_name, next.data);

            } else {
                self.done = true;
                break;
            }
        }

        match res {
            Some(err) => Err(err),
            None => Ok(sample),
        }
    }

    fn create_sample(first: SampleFile) -> Sample {
        let (key, file_name) = first.path_info();
        let mut sample = Sample::new(key);
        sample.update(&file_name, first.data);
        sample
    }
}


impl<I: Iterator<Item = Result<SampleFile>>> Iterator for GroupBy<I> {
    type Item = Result<Sample>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let first = match self.get_next_file() {
            Some(Err(err)) => return Some(Err(err)),
            Some(Ok(first)) => first,
            None => return None,
        };

        let sample = Self::create_sample(first);
        Some(self.collection_loop(sample))
    }
}
