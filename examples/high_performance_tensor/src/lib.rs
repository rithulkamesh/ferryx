use ferryx_macros::ferryx;

#[ferryx]
pub struct FastTensor {
    pub data: Vec<f32>,
}

#[ferryx]
impl FastTensor {
    pub fn dot(&self, other: FastTensor) -> f32 {
        self.data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a * b)
            .sum()
    }
}

