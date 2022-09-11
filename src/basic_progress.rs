use std::time::{Duration, SystemTime};

pub struct BasicProgress {
    len: usize,
    count: usize,
    start_time: SystemTime,
    project_name: String,
    verbose: bool
}

impl<'a> From<&'a str> for BasicProgress {
    fn from(project_name: &'a str) -> Self {
        Self::from_project_name(project_name)
    }
}

impl BasicProgress {
    pub(crate) fn from_project_name<T: ToString>(project_name: T) -> Self {
        Self {
            len: 0,
            count: 0,
            start_time: SystemTime::now(),
            project_name: project_name.to_string(),
            verbose: true
        }
    }

    pub(crate) fn get_elapsed_time(&self) -> Duration {
        SystemTime::now()
            .duration_since(self.start_time)
            .expect("Time went backwards")
    }

    pub(crate) fn get_humanized_elapsed_time(&self) -> String {
        rs_humanize::time::format(
            chrono::Utc::now() - chrono::Duration::from_std(self.get_elapsed_time()).unwrap(),
        )
    }

    pub(crate) fn get_remaining_time(&self) -> Duration {
        Duration::from_millis(
            (self.get_elapsed_time().as_millis() as f32 / self.count as f32
                * (self.len - self.count) as f32) as u64,
        )
    }

    pub(crate) fn get_humanized_remaining_time(&self) -> String {
        rs_humanize::time::format(
            chrono::Utc::now() - chrono::Duration::from_std(self.get_remaining_time()).unwrap(),
        )
    }

    pub(crate) fn get_project_name(&self) -> &str {
        &self.project_name
    }

    pub(crate) fn get_completion_percentage(&self) -> f32 {
        100.0 * self.count as f32 / self.len as f32
    }

    pub(crate) fn tick(&mut self) {
        if self.count == 0 {
            self.reset_start_time();
        }
        self.count += 1;
        self.len = self.len.max(self.count);
    }

    pub(crate) fn is_complete(&self) -> bool {
        self.count == self.len
    }

    pub(crate) fn set_len(&mut self, len: usize) {
        self.len = len;
    }

    pub(crate) fn len(&self) -> usize{
        self.len
    }

    pub(crate) fn get_count(&self) -> usize{
        self.count
    }

    pub(crate) fn reset_start_time(&mut self) {
        self.start_time = SystemTime::now();
    }

    pub(crate) fn is_verbose(&self) -> bool {
        self.verbose
    }

    pub(crate) fn set_verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }
}
