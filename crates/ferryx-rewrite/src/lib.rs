use ferryx_ir::{IrMethod, IrPackage, IrParam, Ownership, TypeRef};

#[derive(Debug, Clone)]
pub struct RewriteContext {
    pub target_language: String,
}

impl Default for RewriteContext {
    fn default() -> Self {
        Self {
            target_language: "python".into(),
        }
    }
}

pub trait RewritePass: Send + Sync {
    fn name(&self) -> &'static str;
    fn rewrite_method(&self, _method: &mut IrMethod, _ctx: &RewriteContext) {}
    fn rewrite_param(&self, _param: &mut IrParam, _ctx: &RewriteContext) {}
}

#[derive(Default)]
pub struct RewritePipeline {
    passes: Vec<Box<dyn RewritePass>>,
}

impl RewritePipeline {
    pub fn new() -> Self {
        Self { passes: Vec::new() }
    }

    pub fn with_pass(mut self, pass: Box<dyn RewritePass>) -> Self {
        self.passes.push(pass);
        self
    }

    pub fn pass_names(&self) -> Vec<&'static str> {
        self.passes.iter().map(|p| p.name()).collect()
    }

    pub fn run(&self, package: &mut IrPackage, ctx: &RewriteContext) {
        for module in &mut package.modules {
            for imp in &mut module.impls {
                for method in &mut imp.methods {
                    for pass in &self.passes {
                        pass.rewrite_method(method, ctx);
                    }
                    for param in &mut method.params {
                        for pass in &self.passes {
                            pass.rewrite_param(param, ctx);
                        }
                    }
                }
            }
        }
    }
}

pub struct NamingNormalizationPass;
impl RewritePass for NamingNormalizationPass {
    fn name(&self) -> &'static str {
        "naming_normalization"
    }

    fn rewrite_method(&self, method: &mut IrMethod, _ctx: &RewriteContext) {
        if let Some(stripped) = method.name.strip_prefix("get_") {
            if let Some((prefix, _suffix)) = stripped.rsplit_once("_by_id") {
                method.name = format!("get_{prefix}");
            }
        }
    }
}

pub struct OwnershipProjectionPass;
impl RewritePass for OwnershipProjectionPass {
    fn name(&self) -> &'static str {
        "ownership_projection"
    }

    fn rewrite_param(&self, param: &mut IrParam, _ctx: &RewriteContext) {
        if let Ownership::Borrowed { .. } = param.ownership {
            param.ty = TypeRef {
                rust: param.ty.rust.replace('&', ""),
            };
        }
    }
}

pub struct IteratorProjectionPass;
impl RewritePass for IteratorProjectionPass {
    fn name(&self) -> &'static str {
        "iterator_projection"
    }

    fn rewrite_method(&self, method: &mut IrMethod, _ctx: &RewriteContext) {
        if method.output.rust.starts_with("std :: vec :: IntoIter") || method.output.rust.contains("IntoIter") {
            method.output = TypeRef {
                rust: "Vec<IteratorItem>".into(),
            };
        }
    }
}

pub struct ExceptionProjectionPass;
impl RewritePass for ExceptionProjectionPass {
    fn name(&self) -> &'static str {
        "exception_projection"
    }

    fn rewrite_method(&self, method: &mut IrMethod, _ctx: &RewriteContext) {
        if let Some(error) = &method.error {
            method.docs.attributes.push(format!("raises:{}", error.rust));
        }
    }
}

pub struct AsyncProjectionPass;
impl RewritePass for AsyncProjectionPass {
    fn name(&self) -> &'static str {
        "async_projection"
    }

    fn rewrite_method(&self, method: &mut IrMethod, _ctx: &RewriteContext) {
        if method.is_async {
            method.docs.attributes.push("python:awaitable".into());
        }
    }
}

pub fn default_python_rewrite_pipeline() -> RewritePipeline {
    RewritePipeline::new()
        .with_pass(Box::new(NamingNormalizationPass))
        .with_pass(Box::new(OwnershipProjectionPass))
        .with_pass(Box::new(ExceptionProjectionPass))
        .with_pass(Box::new(IteratorProjectionPass))
        .with_pass(Box::new(AsyncProjectionPass))
}

#[cfg(test)]
mod tests {
    use ferryx_ir::{
        Docs, IrImpl, IrMethod, IrModule, IrPackage, IrParam, Ownership, ReceiverKind, StabilityLevel, TypeRef,
        IR_VERSION,
    };

    use crate::{default_python_rewrite_pipeline, RewriteContext};

    #[test]
    fn rewrites_method_name_and_borrowed_param() {
        let mut package = IrPackage {
            ir_version: IR_VERSION.into(),
            stability: StabilityLevel::Experimental,
            name: "p".into(),
            modules: vec![IrModule {
                id: "m".into(),
                path: vec!["m".into()],
                docs: Docs::empty(),
                classes: Vec::new(),
                enums: Vec::new(),
                traits: Vec::new(),
                impls: vec![IrImpl {
                    id: "i".into(),
                    module_id: "m".into(),
                    target: TypeRef { rust: "UserApi".into() },
                    trait_name: None,
                    methods: vec![IrMethod {
                        name: "get_user_by_id".into(),
                        receiver: ReceiverKind::Ref,
                        docs: Docs::empty(),
                        is_async: false,
                        async_runtime: None,
                        params: vec![IrParam {
                            name: "id".into(),
                            ty: TypeRef { rust: "& str".into() },
                            ownership: Ownership::Borrowed {
                                mutable: false,
                                lifetime: None,
                            },
                        }],
                        output: TypeRef { rust: "User".into() },
                        error: None,
                    }],
                }],
            }],
        };

        let pipeline = default_python_rewrite_pipeline();
        pipeline.run(&mut package, &RewriteContext::default());
        let method = &package.modules[0].impls[0].methods[0];
        assert_eq!(method.name, "get_user");
        assert_eq!(method.params[0].ty.rust.trim(), "str");
    }

    #[test]
    fn snapshot_like_json_output_is_deterministic() {
        let mut package = IrPackage {
            ir_version: IR_VERSION.into(),
            stability: StabilityLevel::Experimental,
            name: "p".into(),
            modules: Vec::new(),
        };
        let pipeline = default_python_rewrite_pipeline();
        pipeline.run(&mut package, &RewriteContext::default());
        let rendered = serde_json::to_string(&package).expect("serialize");
        assert_eq!(
            rendered,
            "{\"ir_version\":\"0.2.0\",\"stability\":\"Experimental\",\"name\":\"p\",\"modules\":[]}"
        );
    }
}

