use serde::{Deserialize, Serialize};

pub const IR_VERSION: &str = "0.2.0";

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StabilityLevel {
    Experimental,
    Beta,
    Stable,
    Internal,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IrPackage {
    pub ir_version: String,
    pub stability: StabilityLevel,
    pub name: String,
    pub modules: Vec<IrModule>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IrModule {
    pub id: String,
    pub path: Vec<String>,
    pub docs: Docs,
    pub classes: Vec<IrClass>,
    pub enums: Vec<IrEnum>,
    pub traits: Vec<IrTrait>,
    pub impls: Vec<IrImpl>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Docs {
    pub summary: String,
    pub details: String,
    pub attributes: Vec<String>,
}

impl Docs {
    pub fn empty() -> Self {
        Self {
            summary: String::new(),
            details: String::new(),
            attributes: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IrClass {
    pub id: String,
    pub module_id: String,
    pub name: String,
    pub visibility: Visibility,
    pub docs: Docs,
    pub fields: Vec<IrField>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IrField {
    pub name: String,
    pub ty: TypeRef,
    pub visibility: Visibility,
    pub ownership: Ownership,
    pub docs: Docs,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IrEnum {
    pub id: String,
    pub module_id: String,
    pub name: String,
    pub docs: Docs,
    pub variants: Vec<IrEnumVariant>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IrEnumVariant {
    pub name: String,
    pub docs: Docs,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IrTrait {
    pub id: String,
    pub module_id: String,
    pub name: String,
    pub docs: Docs,
    pub methods: Vec<IrMethod>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IrImpl {
    pub id: String,
    pub module_id: String,
    pub target: TypeRef,
    pub trait_name: Option<String>,
    pub methods: Vec<IrMethod>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IrMethod {
    pub name: String,
    pub receiver: ReceiverKind,
    pub docs: Docs,
    pub is_async: bool,
    pub async_runtime: Option<AsyncRuntimeHint>,
    pub params: Vec<IrParam>,
    pub output: TypeRef,
    pub error: Option<TypeRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IrParam {
    pub name: String,
    pub ty: TypeRef,
    pub ownership: Ownership,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TypeRef {
    pub rust: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReceiverKind {
    Value,
    Ref,
    MutRef,
    Static,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Visibility {
    Public,
    Restricted(String),
    Private,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Ownership {
    Owned,
    Borrowed { mutable: bool, lifetime: Option<String> },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AsyncRuntimeHint {
    Tokio,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum IrItem {
    Class(IrClass),
    Impl(IrImpl),
    Trait(IrTrait),
    Enum(IrEnum),
}

pub fn validate_ir_compatibility(package: &IrPackage) -> Result<(), String> {
    if package.ir_version != IR_VERSION {
        return Err(format!(
            "IR version mismatch: expected {IR_VERSION}, found {}",
            package.ir_version
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn package_json_round_trip() {
        let package = IrPackage {
            ir_version: IR_VERSION.into(),
            stability: StabilityLevel::Beta,
            name: "ferryx_fixture".into(),
            modules: vec![IrModule {
                id: "m::tensor".into(),
                path: vec!["tensor".into()],
                docs: Docs {
                    summary: "Tensor module".into(),
                    details: "Detailed docs".into(),
                    attributes: vec!["doc(hidden)".into()],
                },
                classes: vec![IrClass {
                    id: "m::tensor::Tensor".into(),
                    module_id: "m::tensor".into(),
                    name: "Tensor".into(),
                    visibility: Visibility::Public,
                    docs: Docs::empty(),
                    fields: vec![IrField {
                        name: "data".into(),
                        ty: TypeRef { rust: "Vec<f32>".into() },
                        visibility: Visibility::Public,
                        ownership: Ownership::Owned,
                        docs: Docs::empty(),
                    }],
                }],
                enums: Vec::new(),
                traits: Vec::new(),
                impls: Vec::new(),
            }],
        };

        let json = serde_json::to_string_pretty(&package).expect("serialize package");
        let de: IrPackage = serde_json::from_str(&json).expect("deserialize package");
        assert_eq!(de, package);
    }

    #[test]
    fn detects_version_mismatch() {
        let package = IrPackage {
            ir_version: "0.0.1".into(),
            stability: StabilityLevel::Experimental,
            name: "ferryx_fixture".into(),
            modules: Vec::new(),
        };
        assert!(validate_ir_compatibility(&package).is_err());
    }
}

