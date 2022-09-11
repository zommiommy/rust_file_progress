use crate::*;

pub struct MarkdownFileProgress {
    path: String,
    file_progress: BasicProgress,
}

impl<'a> From<&'a str> for MarkdownFileProgress {
    fn from(project_name: &'a str) -> Self {
        Self::from_project_name(project_name)
    }
}

impl MarkdownFileProgress {
    pub fn from_project_name<T: ToString>(project_name: T) -> Self {
        Self {
            path: format!("{}.md", project_name.to_string()),
            file_progress: BasicProgress::from_project_name(project_name),
        }
    }
}

impl ToString for MarkdownFileProgress {
    fn to_string(&self) -> String {
        format!(
            concat!(
                "# {project_name}\n",
                "* Status: {status_class}\n",
                "* Completion: {number_of_completed_steps}/{total_number_of_steps}, {completion_percentage}%\n",
                "* Elapsed time: {elapsed_time}\n",
                "* Remaining time: {remaining_time}\n",
            ),
            project_name = self.file_progress.get_project_name(),
            status_class = if self.is_complete() {
                "complete"
            } else {
                "in progress"
            },
            number_of_completed_steps = self.file_progress.get_count(),
            total_number_of_steps = self.file_progress.len(),
            completion_percentage = self.file_progress.get_completion_percentage(),
            elapsed_time = self.file_progress.get_humanized_elapsed_time(),
            remaining_time = self.file_progress.get_humanized_remaining_time(),
        )
    }
}

impl FileProgress for MarkdownFileProgress {
    fn tick(&mut self) {
        self.file_progress.tick()
    }

    fn get_path(&self) -> &str {
        &self.path
    }

    fn is_complete(&self) -> bool {
        self.file_progress.is_complete()
    }

    fn set_len(&mut self, len: usize) {
        self.file_progress.set_len(len)
    }

    fn get_count(&self) -> usize {
        self.file_progress.get_count()
    }

    fn is_verbose(&self) -> bool {
        self.file_progress.is_verbose()
    }

    fn set_verbose(&mut self, verbose: bool) {
        self.file_progress.set_verbose(verbose)
    }
}
