use ferryx_macros::ferryx;

#[ferryx]
pub struct FloatBuffer {
    pub data: Vec<f32>,
}

#[ferryx]
impl FloatBuffer {
    pub fn as_slice_len(&self) -> usize {
        self.data.len()
    }
}

