use ferryx_macros::ferryx;

#[ferryx]
pub struct AsyncModel {
    pub name: String,
}

#[ferryx]
impl AsyncModel {
    pub async fn infer(&self, input: Vec<f32>) -> Vec<f32> {
        input.into_iter().map(|v| v * 2.0).collect()
    }
}

