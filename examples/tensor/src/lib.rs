use ferryx_macros::ferryx;

#[ferryx]
pub struct Tensor {
    pub data: Vec<f32>,
}

#[ferryx]
impl Tensor {
    pub fn add(&self, other: Tensor) -> Tensor {
        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a + b)
            .collect();
        Tensor { data }
    }
}

