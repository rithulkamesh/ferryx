use ferryx_ir::IrPackage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TargetLanguage {
    Python,
    TypeScript,
    Wasm,
    OpenApi,
    Grpc,
    Julia,
    R,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetCapabilities {
    pub async_support: bool,
    pub exceptions: bool,
    pub protocols: bool,
    pub notebook_repr: bool,
    pub zero_copy_buffers: bool,
    pub browser_compatible: bool,
    pub schema_generation: bool,
    pub ownership_restrictions: bool,
}

pub fn capabilities(target: TargetLanguage) -> TargetCapabilities {
    match target {
        TargetLanguage::Python => TargetCapabilities {
            async_support: true,
            exceptions: true,
            protocols: true,
            notebook_repr: true,
            zero_copy_buffers: true,
            browser_compatible: false,
            schema_generation: false,
            ownership_restrictions: false,
        },
        TargetLanguage::TypeScript => TargetCapabilities {
            async_support: true,
            exceptions: true,
            protocols: false,
            notebook_repr: false,
            zero_copy_buffers: false,
            browser_compatible: true,
            schema_generation: true,
            ownership_restrictions: false,
        },
        TargetLanguage::Wasm => TargetCapabilities {
            async_support: true,
            exceptions: false,
            protocols: false,
            notebook_repr: false,
            zero_copy_buffers: true,
            browser_compatible: true,
            schema_generation: false,
            ownership_restrictions: true,
        },
        TargetLanguage::OpenApi => TargetCapabilities {
            async_support: true,
            exceptions: true,
            protocols: false,
            notebook_repr: false,
            zero_copy_buffers: false,
            browser_compatible: true,
            schema_generation: true,
            ownership_restrictions: false,
        },
        TargetLanguage::Grpc => TargetCapabilities {
            async_support: true,
            exceptions: true,
            protocols: false,
            notebook_repr: false,
            zero_copy_buffers: false,
            browser_compatible: false,
            schema_generation: true,
            ownership_restrictions: false,
        },
        TargetLanguage::Julia | TargetLanguage::R => TargetCapabilities {
            async_support: false,
            exceptions: true,
            protocols: false,
            notebook_repr: true,
            zero_copy_buffers: true,
            browser_compatible: false,
            schema_generation: false,
            ownership_restrictions: true,
        },
    }
}

pub fn validate_target_compatibility(package: &IrPackage, target: TargetLanguage) -> Vec<String> {
    let caps = capabilities(target);
    let mut warnings = Vec::new();
    let has_async = package
        .modules
        .iter()
        .flat_map(|m| m.impls.iter())
        .flat_map(|i| i.methods.iter())
        .any(|m| m.is_async);
    if has_async && !caps.async_support {
        warnings.push(format!("{target:?} target does not fully support async semantics"));
    }
    let has_errors = package
        .modules
        .iter()
        .flat_map(|m| m.impls.iter())
        .flat_map(|i| i.methods.iter())
        .any(|m| m.error.is_some());
    if has_errors && !caps.exceptions {
        warnings.push(format!("{target:?} target cannot project exceptions directly"));
    }
    warnings
}

