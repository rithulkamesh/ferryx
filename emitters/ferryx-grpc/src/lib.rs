use ferryx_ir::IrPackage;
use ferryx_rewrite::{default_python_rewrite_pipeline, RewriteContext};

pub fn emit_proto(package: &IrPackage) -> String {
    let mut package = package.clone();
    default_python_rewrite_pipeline().run(&mut package, &RewriteContext::default());
    let mut proto = String::new();
    proto.push_str("syntax = \"proto3\";\n\n");
    proto.push_str(&format!("package {};\n\n", package.name.replace('-', "_")));
    for module in &package.modules {
        for class in &module.classes {
            proto.push_str(&format!("message {} {{\n", class.name));
            for (idx, field) in class.fields.iter().enumerate() {
                let ty = map_field_type(&field.ty.rust);
                proto.push_str(&format!("  {} {} = {};\n", ty, field.name, idx + 1));
            }
            proto.push_str("}\n\n");
        }
        for imp in &module.impls {
            let svc = imp.target.rust.replace(' ', "");
            proto.push_str(&format!("service {}Service {{\n", svc));
            for method in &imp.methods {
                proto.push_str(&format!(
                    "  rpc {} ({}Request) returns ({}Response);\n",
                    method.name, method.name, method.name
                ));
            }
            proto.push_str("}\n\n");
        }
    }
    proto
}

fn map_field_type(rust: &str) -> &'static str {
    match rust.replace(' ', "").as_str() {
        "String" | "&str" => "string",
        "f32" | "f64" => "double",
        "i32" | "i64" | "u32" | "u64" | "usize" | "isize" => "int64",
        "bool" => "bool",
        _ => "string",
    }
}

