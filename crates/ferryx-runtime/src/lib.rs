use std::sync::Arc;

use ferryx_ir::IrItem;
use once_cell::sync::Lazy;

pub use inventory;

#[derive(Debug)]
pub struct ReflectionRecord {
    pub module_path: &'static str,
    pub item_name: &'static str,
    pub item_json: &'static str,
}

inventory::collect!(ReflectionRecord);

#[derive(Debug, Clone)]
pub struct RegisteredItem {
    pub module_path: String,
    pub item_name: String,
    pub item: IrItem,
}

#[derive(Debug, Default)]
pub struct Registry {
    entries: Vec<RegisteredItem>,
}

impl Registry {
    pub fn from_inventory() -> Self {
        let mut entries = Vec::new();
        for record in inventory::iter::<ReflectionRecord> {
            let item: IrItem = match serde_json::from_str(record.item_json) {
                Ok(parsed) => parsed,
                Err(_) => continue,
            };

            entries.push(RegisteredItem {
                module_path: record.module_path.to_owned(),
                item_name: record.item_name.to_owned(),
                item,
            });
        }

        entries.sort_by(|a, b| a.module_path.cmp(&b.module_path).then(a.item_name.cmp(&b.item_name)));

        Self { entries }
    }

    pub fn all_items(&self) -> &[RegisteredItem] {
        &self.entries
    }

    pub fn find_item(&self, module_path: &str, item_name: &str) -> Option<&RegisteredItem> {
        self.entries
            .iter()
            .find(|e| e.module_path == module_path && e.item_name == item_name)
    }

    pub fn methods_for_type(&self, type_name: &str) -> Vec<Arc<String>> {
        let mut names = Vec::new();
        for entry in &self.entries {
            if let IrItem::Impl(imp) = &entry.item {
                if imp.target.rust.ends_with(type_name) || imp.target.rust == type_name {
                    for method in &imp.methods {
                        names.push(Arc::new(method.name.clone()));
                    }
                }
            }
        }
        names
    }
}

pub static GLOBAL_REGISTRY: Lazy<Registry> = Lazy::new(Registry::from_inventory);

pub fn all_items() -> &'static [RegisteredItem] {
    GLOBAL_REGISTRY.all_items()
}

pub fn find_item(module_path: &str, item_name: &str) -> Option<&'static RegisteredItem> {
    GLOBAL_REGISTRY.find_item(module_path, item_name)
}

#[derive(Debug, Clone, PartialEq)]
pub struct NumpyBufferView {
    pub ptr: usize,
    pub len: usize,
    pub item_size: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrowArrayView {
    pub len: usize,
    pub null_count: usize,
}

pub fn project_vec_f32_to_numpy_view(data: &[f32]) -> NumpyBufferView {
    NumpyBufferView {
        ptr: data.as_ptr() as usize,
        len: data.len(),
        item_size: std::mem::size_of::<f32>(),
    }
}

pub fn project_i64_to_arrow_view(data: &[i64]) -> ArrowArrayView {
    ArrowArrayView {
        len: data.len(),
        null_count: 0,
    }
}

