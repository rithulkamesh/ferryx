use ferryx_macros::ferryx;

#[ferryx]
pub struct HealthService;

#[ferryx]
impl HealthService {
    pub async fn check(&self) -> String {
        "ok".to_string()
    }
}

