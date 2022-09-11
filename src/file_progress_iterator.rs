use crate::*;

pub struct FileProgressIter<I, FP> {
    it: I,
    progress: FP,
}

impl<I, FP: FileProgress> ToString for FileProgressIter<I, FP> {
    fn to_string(&self) -> String {
        self.progress.to_string()
    }
}

impl<I: ExactSizeIterator, FP: FileProgress> FileProgress for FileProgressIter<I, FP> {
    /// Increase by one.
    fn tick(&mut self) {
        if self.get_count() == 0 {
            self.set_len(self.len());
        }
        self.progress.tick();
    }

    /// Get the path to the file.
    fn get_path(&self) -> &str {
        self.progress.get_path()
    }

    fn set_len(&mut self, len: usize) {
        self.progress.set_len(len);
    }

    fn get_count(&self) -> usize {
        self.progress.get_count()
    }

    fn set_verbose(&mut self, verbose: bool) {
        self.progress.set_verbose(verbose)
    }

    fn is_verbose(&self) -> bool {
        self.progress.is_verbose()
    }

    /// Returns whether the process has completed.
    fn is_complete(&self) -> bool {
        self.progress.is_complete()
    }
}

/// Wraps an iterator to display its progress.
pub trait FileProgressIterator<FP>
where
    Self: Sized + Iterator,
    FP: FileProgress,
{
    /// Wrap an iterator with a custom file progress.
    fn progress_with_file(self, progress: FP) -> FileProgressIter<Self, FP>;

    /// Wrap an iterator with a custom file progress created from path.
    fn progress_from_path<S: Into<FP>>(self, path: S) -> FileProgressIter<Self, FP> {
        self.progress_with_file(path.into())
    }
}

impl<S, FP, I: Iterator<Item = S>> FileProgressIterator<FP> for I
where
    FP: FileProgress,
{
    fn progress_with_file(self, progress: FP) -> FileProgressIter<Self, FP> {
        FileProgressIter { it: self, progress }
    }
}

impl<S, FP: FileProgress, I: ExactSizeIterator + Iterator<Item = S>> Iterator for FileProgressIter<I, FP> {
    type Item = S;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.it.next();

        if item.is_some() {
            let _ = self.tick_and_print();
        }

        item
    }
}

impl<I: ExactSizeIterator, FP: FileProgress> ExactSizeIterator for FileProgressIter<I, FP> {
    fn len(&self) -> usize {
        self.it.len()
    }
}

impl<I: ExactSizeIterator + DoubleEndedIterator, FP: FileProgress> DoubleEndedIterator for FileProgressIter<I, FP> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let item = self.it.next_back();

        if item.is_some() {
            let _ = self.tick_and_print();
        }

        item
    }
}
