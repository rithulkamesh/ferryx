use ferryx_macros::ferryx;

#[ferryx]
pub struct ColumnBatch {
    pub name: String,
    pub values: Vec<f64>,
}

#[ferryx]
impl ColumnBatch {
    pub fn mean(&self) -> f64 {
        let sum: f64 = self.values.iter().sum();
        sum / self.values.len() as f64
    }
}

