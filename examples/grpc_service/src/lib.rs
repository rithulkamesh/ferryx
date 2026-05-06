use ferryx_macros::ferryx;

#[ferryx]
pub struct EchoService;

#[ferryx]
impl EchoService {
    pub async fn echo(&self, message: String) -> Result<String, String> {
        Ok(message)
    }
}

