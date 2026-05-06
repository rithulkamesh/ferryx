use ferryx_ir::{IrImpl, IrPackage, TypeRef};
use ferryx_rewrite::{default_python_rewrite_pipeline, RewriteContext};

pub fn emit_openapi_json(package: &IrPackage) -> String {
    let mut package = package.clone();
    default_python_rewrite_pipeline().run(&mut package, &RewriteContext::default());
    let mut paths = serde_json::Map::new();
    for module in &package.modules {
        for imp in &module.impls {
            append_impl_paths(imp, &mut paths);
        }
    }

    let doc = serde_json::json!({
        "openapi": "3.1.0",
        "info": { "title": package.name, "version": "0.1.0" },
        "paths": paths
    });
    serde_json::to_string_pretty(&doc).expect("serialize openapi")
}

fn append_impl_paths(imp: &IrImpl, paths: &mut serde_json::Map<String, serde_json::Value>) {
    let base = imp.target.rust.replace(' ', "").to_lowercase();
    for method in &imp.methods {
        let path = format!("/{base}/{}", method.name);
        let response_type = map_schema_type(&method.output);
        let operation = serde_json::json!({
            "post": {
                "operationId": format!("{}_{}", base, method.name),
                "responses": {
                    "200": {
                        "description": "Success",
                        "content": {
                            "application/json": { "schema": response_type }
                        }
                    }
                }
            }
        });
        paths.insert(path, operation);
    }
}

fn map_schema_type(ty: &TypeRef) -> serde_json::Value {
    let rust = ty.rust.replace(' ', "");
    if rust == "String" || rust == "&str" {
        return serde_json::json!({"type":"string"});
    }
    if ["f32", "f64", "i32", "i64", "u32", "u64", "usize", "isize"].contains(&rust.as_str()) {
        return serde_json::json!({"type":"number"});
    }
    if rust == "bool" {
        return serde_json::json!({"type":"boolean"});
    }
    if let Some(inner) = rust.strip_prefix("Vec<").and_then(|s| s.strip_suffix('>')) {
        return serde_json::json!({"type":"array","items":map_schema_type(&TypeRef{ rust: inner.into()})});
    }
    serde_json::json!({"type":"object","title":rust})
}

