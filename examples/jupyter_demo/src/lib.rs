use ferryx_macros::ferryx;

#[ferryx]
pub struct NotebookKernel {
    pub session_id: String,
}

#[ferryx]
impl NotebookKernel {
    pub fn execute_cell(&self, code: String) -> String {
        format!("session={} bytes={}", self.session_id, code.len())
    }
}

