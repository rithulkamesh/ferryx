use ferryx_macros::ferryx;

#[ferryx]
pub struct ArrowColumn {
    pub name: String,
    pub values: Vec<i64>,
}

#[ferryx]
impl ArrowColumn {
    pub fn len(&self) -> usize {
        self.values.len()
    }
}

