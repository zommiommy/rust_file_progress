use file_progress::{FileProgressIterator, HTMLFileProgress, MarkdownFileProgress};
use std::{thread, time::Duration};

extern crate file_progress;

#[test]
fn test_file_progress_html() {
    let progress = HTMLFileProgress::from_project_name("simple_test");
    (0..100).progress_with_file(progress).for_each(|_| {
        thread::sleep(Duration::from_millis(100));
    });
}


#[test]
fn test_file_progress_markdown() {
    let progress = MarkdownFileProgress::from_project_name("simple_test");
    (0..100).progress_with_file(progress).for_each(|_| {
        thread::sleep(Duration::from_millis(100));
    });
}
