use ferryx_macros::ferryx;

#[ferryx]
pub struct LlmRuntime {
    pub model: String,
}

#[ferryx]
impl LlmRuntime {
    pub async fn complete(&self, prompt: String) -> String {
        format!("model={} tokens={}", self.model, prompt.split_whitespace().count())
    }
}

