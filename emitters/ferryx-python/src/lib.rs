use std::collections::BTreeSet;

use ferryx_ir::{IrClass, IrImpl, IrMethod, IrPackage, IrTrait, TypeRef};
use ferryx_rewrite::{default_python_rewrite_pipeline, RewriteContext};

#[derive(Debug, Clone)]
pub struct EmittedFile {
    pub path: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct Emission {
    pub files: Vec<EmittedFile>,
}

pub fn emit_python(package: &IrPackage) -> Emission {
    let mut package = package.clone();
    default_python_rewrite_pipeline().run(&mut package, &RewriteContext::default());

    let mut py = String::new();
    let mut pyi = String::new();
    let mut exceptions = BTreeSet::new();

    py.push_str("from __future__ import annotations\n\n");
    pyi.push_str("from __future__ import annotations\n");
    pyi.push_str("from typing import Protocol\n\n");

    for module in &package.modules {
        for tr in &module.traits {
            write_protocol(&mut py, tr);
            write_protocol(&mut pyi, tr);
        }
        for class in &module.classes {
            let methods = methods_for_class(class, &module.impls);
            write_class(&mut py, class, methods.as_slice(), &mut exceptions);
            write_class_stub(&mut pyi, class, methods.as_slice(), &mut exceptions);
        }
    }

    if !exceptions.is_empty() {
        py.push('\n');
        pyi.push('\n');
        for ex in exceptions {
            py.push_str(&format!("class {ex}(Exception):\n    pass\n\n"));
            pyi.push_str(&format!("class {ex}(Exception): ...\n"));
        }
    }

    Emission {
        files: vec![
            EmittedFile {
                path: format!("{}/__init__.py", package.name),
                content: py,
            },
            EmittedFile {
                path: format!("{}/__init__.pyi", package.name),
                content: pyi,
            },
        ],
    }
}

fn write_class(out: &mut String, class: &IrClass, methods: &[IrMethod], exceptions: &mut BTreeSet<String>) {
    out.push_str(&format!("class {}:\n", class.name));
    if !class.docs.summary.is_empty() {
        out.push_str(&format!("    \"\"\"{}\"\"\"\n", class.docs.summary));
    }
    for field in &class.fields {
        out.push_str(&format!("    {}: {}\n", field.name, map_type(&field.ty)));
    }
    if class.fields.is_empty() {
        out.push_str("    pass\n");
    }
    out.push_str(&format!(
        "    def __repr__(self) -> str:\n        return \"{}(...)\"\n",
        class.name
    ));
    out.push_str("    def _repr_markdown_(self) -> str:\n        return self.__repr__()\n");
    for method in methods {
        if let Some(error) = &method.error {
            exceptions.insert(error_name(error));
        }
        out.push_str(&format!(
            "    def {}(self{}) -> {}:\n        raise NotImplementedError(\"Bound at runtime\")\n",
            method.name,
            render_params(method),
            map_type(&method.output)
        ));
    }
    out.push('\n');
}

fn write_class_stub(out: &mut String, class: &IrClass, methods: &[IrMethod], exceptions: &mut BTreeSet<String>) {
    out.push_str(&format!("class {}:\n", class.name));
    for field in &class.fields {
        out.push_str(&format!("    {}: {}\n", field.name, map_type(&field.ty)));
    }
    if class.fields.is_empty() {
        out.push_str("    ...\n");
    }
    for method in methods {
        if let Some(error) = &method.error {
            exceptions.insert(error_name(error));
        }
        out.push_str(&format!(
            "    def {}(self{}) -> {}: ...\n",
            method.name,
            render_params(method),
            map_type(&method.output)
        ));
    }
    out.push('\n');
}

fn write_protocol(out: &mut String, tr: &IrTrait) {
    out.push_str(&format!("class {}Protocol(Protocol):\n", tr.name));
    if tr.methods.is_empty() {
        out.push_str("    ...\n\n");
        return;
    }
    for method in &tr.methods {
        out.push_str(&format!(
            "    def {}(self{}) -> {}: ...\n",
            method.name,
            render_params(method),
            map_type(&method.output)
        ));
    }
    out.push('\n');
}

fn methods_for_class(class: &IrClass, impls: &[IrImpl]) -> Vec<IrMethod> {
    let mut out = Vec::new();
    for imp in impls {
        let target = imp.target.rust.replace(' ', "");
        if target.ends_with(&class.name) || target == class.name {
            out.extend(imp.methods.clone());
        }
    }
    out
}

fn render_params(method: &IrMethod) -> String {
    let mut rendered = String::new();
    for param in &method.params {
        rendered.push_str(&format!(", {}: {}", param.name, map_type(&param.ty)));
    }
    rendered
}

fn map_type(ty: &TypeRef) -> String {
    let rust = ty.rust.replace(' ', "");
    if rust == "String" || rust == "&str" {
        return "str".into();
    }
    if rust == "f32" || rust == "f64" {
        return "float".into();
    }
    if rust == "i32" || rust == "i64" || rust == "u32" || rust == "u64" || rust == "usize" || rust == "isize" {
        return "int".into();
    }
    if rust == "bool" {
        return "bool".into();
    }
    if let Some(inner) = rust.strip_prefix("Vec<").and_then(|t| t.strip_suffix('>')) {
        return format!("list[{}]", map_type(&TypeRef { rust: inner.into() }));
    }
    if let Some(inner) = rust.strip_prefix("Option<").and_then(|t| t.strip_suffix('>')) {
        return format!("{} | None", map_type(&TypeRef { rust: inner.into() }));
    }
    rust
}

fn error_name(error: &TypeRef) -> String {
    let base = error.rust.replace(['<', '>', ':', ',', '&', ' '], "");
    if base.ends_with("Error") {
        base
    } else {
        format!("{base}Error")
    }
}

#[cfg(test)]
mod tests {
    use ferryx_ir::{
        Docs, IrClass, IrField, IrModule, IrPackage, Ownership, StabilityLevel, TypeRef, Visibility, IR_VERSION,
    };

    use crate::emit_python;

    #[test]
    fn emits_tensor_typing() {
        let package = IrPackage {
            ir_version: IR_VERSION.into(),
            stability: StabilityLevel::Beta,
            name: "ferryx_tensor".into(),
            modules: vec![IrModule {
                id: "crate".into(),
                path: vec!["crate".into()],
                docs: Docs::empty(),
                classes: vec![IrClass {
                    id: "crate::Tensor".into(),
                    module_id: "crate".into(),
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

        let emission = emit_python(&package);
        let pyi = emission
            .files
            .iter()
            .find(|f| f.path.ends_with("__init__.pyi"))
            .expect("pyi file exists");
        assert!(pyi.content.contains("class Tensor"));
        assert!(pyi.content.contains("data: list[float]"));
    }
}

