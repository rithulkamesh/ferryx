use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

use ferryx_ir::IrPackage;
use once_cell::sync::Lazy;

pub trait EmitterPlugin: Send + Sync {
    fn id(&self) -> &'static str;
    fn emit(&self, package: &IrPackage) -> BTreeMap<String, String>;
}

pub trait TypeMapperPlugin: Send + Sync {
    fn id(&self) -> &'static str;
    fn map_type(&self, rust_ty: &str, target: &str) -> Option<String>;
}

pub trait SerializerPlugin: Send + Sync {
    fn id(&self) -> &'static str;
    fn serialize_package(&self, package: &IrPackage) -> Result<Vec<u8>, String>;
}

pub trait NotebookRendererPlugin: Send + Sync {
    fn id(&self) -> &'static str;
    fn mime_bundle(&self, type_name: &str, payload: &str) -> BTreeMap<String, String>;
}

#[derive(Default)]
pub struct PluginRegistry {
    emitters: BTreeMap<String, Arc<dyn EmitterPlugin>>,
    type_mappers: BTreeMap<String, Arc<dyn TypeMapperPlugin>>,
    serializers: BTreeMap<String, Arc<dyn SerializerPlugin>>,
    notebook_renderers: BTreeMap<String, Arc<dyn NotebookRendererPlugin>>,
}

impl PluginRegistry {
    pub fn register_emitter(&mut self, plugin: Arc<dyn EmitterPlugin>) {
        self.emitters.insert(plugin.id().into(), plugin);
    }
    pub fn register_type_mapper(&mut self, plugin: Arc<dyn TypeMapperPlugin>) {
        self.type_mappers.insert(plugin.id().into(), plugin);
    }
    pub fn register_serializer(&mut self, plugin: Arc<dyn SerializerPlugin>) {
        self.serializers.insert(plugin.id().into(), plugin);
    }
    pub fn register_notebook_renderer(&mut self, plugin: Arc<dyn NotebookRendererPlugin>) {
        self.notebook_renderers.insert(plugin.id().into(), plugin);
    }
    pub fn emitter_ids(&self) -> Vec<String> {
        self.emitters.keys().cloned().collect()
    }
}

pub static GLOBAL_PLUGIN_REGISTRY: Lazy<RwLock<PluginRegistry>> =
    Lazy::new(|| RwLock::new(PluginRegistry::default()));

