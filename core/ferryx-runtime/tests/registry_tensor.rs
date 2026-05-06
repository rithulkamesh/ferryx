use ferryx_macros::ferryx;

#[ferryx]
pub struct Tensor {
    pub data: Vec<f32>,
}

#[ferryx]
impl Tensor {
    pub fn add(&self, other: Tensor) -> Tensor {
        other
    }
}

#[test]
fn tensor_descriptor_is_registered() {
    let entries = ferryx_runtime::all_items();
    assert!(!entries.is_empty());
    let class = ferryx_runtime::find_item(module_path!(), "Tensor");
    assert!(class.is_some(), "Tensor class descriptor should be available");
    let methods = ferryx_runtime::GLOBAL_REGISTRY.methods_for_type("Tensor");
    assert!(methods.iter().any(|m| m.as_str() == "add"));
}

