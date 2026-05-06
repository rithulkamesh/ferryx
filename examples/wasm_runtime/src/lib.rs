use ferryx_macros::ferryx;

#[ferryx]
pub struct WasmMath {
    pub scale: f32,
}

#[ferryx]
impl WasmMath {
    pub fn mul(&self, v: Vec<f32>) -> Vec<f32> {
        v.into_iter().map(|x| x * self.scale).collect()
    }
}

