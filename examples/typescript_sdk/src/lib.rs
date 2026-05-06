use ferryx_macros::ferryx;

#[ferryx]
pub struct UserApi {
    pub service_name: String,
}

#[ferryx]
impl UserApi {
    pub async fn get_user_by_id(&self, id: String) -> Result<String, String> {
        Ok(format!("{}:{id}", self.service_name))
    }
}

