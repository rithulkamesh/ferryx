use ferryx_macros::ferryx;

#[ferryx]
pub struct TensorRuntime {
    pub shape: Vec<usize>,
    pub data: Vec<f32>,
}

#[ferryx]
impl TensorRuntime {
    pub fn new(shape: Vec<usize>, data: Vec<f32>) -> Result<TensorRuntime, String> {
        let expected: usize = shape.iter().product();
        if expected != data.len() {
            return Err(format!("shape product {expected} != data len {}", data.len()));
        }
        Ok(TensorRuntime { shape, data })
    }

    pub fn matmul2x2(&self, other: TensorRuntime) -> Result<TensorRuntime, String> {
        if self.shape != vec![2, 2] || other.shape != vec![2, 2] {
            return Err("matmul2x2 requires two 2x2 tensors".into());
        }
        let a = &self.data;
        let b = &other.data;
        let out = vec![
            a[0] * b[0] + a[1] * b[2],
            a[0] * b[1] + a[1] * b[3],
            a[2] * b[0] + a[3] * b[2],
            a[2] * b[1] + a[3] * b[3],
        ];
        Ok(TensorRuntime {
            shape: vec![2, 2],
            data: out,
        })
    }

    pub async fn scale_async(&self, factor: f32) -> TensorRuntime {
        TensorRuntime {
            shape: self.shape.clone(),
            data: self.data.iter().map(|x| x * factor).collect(),
        }
    }
}

