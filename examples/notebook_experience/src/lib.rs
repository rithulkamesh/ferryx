use ferryx_macros::ferryx;

#[ferryx]
pub struct TensorPreview {
    pub shape: Vec<usize>,
    pub sample: Vec<f32>,
}

#[ferryx]
impl TensorPreview {
    pub fn summary(&self) -> String {
        format!("shape={:?}, sample={:?}", self.shape, self.sample)
    }
}

