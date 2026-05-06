use ferryx_ir::IrPackage;

#[derive(Debug, Clone)]
pub struct EmittedFile {
    pub path: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct WasmEmission {
    pub files: Vec<EmittedFile>,
}

pub fn emit_wasm(package: &IrPackage) -> WasmEmission {
    let mut bindings = String::new();
    bindings.push_str("// generated wasm host bindings model\n");
    bindings.push_str("export interface WasmHostBridge {\n  init(moduleUrl: string): Promise<void>\n}\n\n");
    for module in &package.modules {
        bindings.push_str(&format!("// module {}\n", module.id));
        for class in &module.classes {
            bindings.push_str(&format!("export interface {}Handle {{ __ptr: number }}\n", class.name));
        }
    }

    let memory_model = serde_json::json!({
        "ownership": "host-bridge",
        "transfer": "explicit-copy-or-borrowed-view",
        "async": "promise-bridge",
        "package": package.name
    });

    WasmEmission {
        files: vec![
            EmittedFile {
                path: "bindings.ts".into(),
                content: bindings,
            },
            EmittedFile {
                path: "memory-model.json".into(),
                content: serde_json::to_string_pretty(&memory_model).expect("serialize wasm memory model"),
            },
        ],
    }
}

