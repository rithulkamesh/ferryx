use ferryx_ir::{
    AsyncRuntimeHint, Docs, IrClass, IrField, IrImpl, IrMethod, IrModule, IrPackage, IrParam, Ownership, ReceiverKind,
    StabilityLevel, TypeRef, Visibility, IR_VERSION,
};
use quote::quote;
use syn::{FnArg, Item, ItemImpl, ItemStruct, ReturnType, Type};

pub fn parse_source_to_ir(package_name: &str, source: &str) -> syn::Result<IrPackage> {
    let file = syn::parse_file(source)?;
    let mut module = IrModule {
        id: "crate".into(),
        path: vec!["crate".into()],
        docs: Docs::empty(),
        classes: Vec::new(),
        enums: Vec::new(),
        traits: Vec::new(),
        impls: Vec::new(),
    };

    for item in file.items {
        match item {
            Item::Struct(st) => module.classes.push(parse_struct(st)),
            Item::Impl(imp) => module.impls.push(parse_impl(imp)),
            _ => {}
        }
    }

    Ok(IrPackage {
        ir_version: IR_VERSION.into(),
        stability: StabilityLevel::Beta,
        name: package_name.to_owned(),
        modules: vec![module],
    })
}

fn parse_struct(item: ItemStruct) -> IrClass {
    IrClass {
        id: format!("crate::{}", item.ident),
        module_id: "crate".into(),
        name: item.ident.to_string(),
        visibility: map_visibility(&item.vis),
        docs: docs_from_attrs(&item.attrs),
        fields: item
            .fields
            .iter()
            .filter_map(|field| {
                let ty = &field.ty;
                Some(IrField {
                    name: field.ident.as_ref()?.to_string(),
                    ty: TypeRef {
                        rust: quote!(#ty).to_string(),
                    },
                    visibility: map_visibility(&field.vis),
                    ownership: ownership_from_type(&field.ty),
                    docs: docs_from_attrs(&field.attrs),
                })
            })
            .collect(),
    }
}

fn parse_impl(item: ItemImpl) -> IrImpl {
    let self_ty = &item.self_ty;
    let methods = item
        .items
        .iter()
        .filter_map(|it| match it {
            syn::ImplItem::Fn(f) => {
                let params = f
                    .sig
                    .inputs
                    .iter()
                    .filter_map(|arg| match arg {
                        FnArg::Typed(pat) => {
                            let pat_name = &pat.pat;
                            let pat_ty = &pat.ty;
                            Some(IrParam {
                                name: quote!(#pat_name).to_string(),
                            ty: TypeRef {
                                    rust: quote!(#pat_ty).to_string(),
                            },
                                ownership: ownership_from_type(&pat.ty),
                            })
                        }
                        FnArg::Receiver(_) => None,
                    })
                    .collect::<Vec<_>>();
                let receiver = receiver_kind(&f.sig.inputs);
                let (output, error) = parse_output(&f.sig.output);
                Some(IrMethod {
                    name: f.sig.ident.to_string(),
                    receiver,
                    docs: docs_from_attrs(&f.attrs),
                    is_async: f.sig.asyncness.is_some(),
                    async_runtime: f.sig.asyncness.map(|_| AsyncRuntimeHint::Tokio),
                    params,
                    output,
                    error,
                })
            }
            _ => None,
        })
        .collect::<Vec<_>>();

    IrImpl {
        id: format!("crate::impl::{}", quote!(#self_ty)),
        module_id: "crate".into(),
        target: TypeRef {
            rust: quote!(#self_ty).to_string(),
        },
        trait_name: item.trait_.as_ref().map(|(_, path, _)| quote!(#path).to_string()),
        methods,
    }
}

fn parse_output(output: &ReturnType) -> (TypeRef, Option<TypeRef>) {
    match output {
        ReturnType::Default => (TypeRef { rust: "()".into() }, None),
        ReturnType::Type(_, ty) => match result_types(ty) {
            Some((ok, err)) => (TypeRef { rust: ok }, Some(TypeRef { rust: err })),
            None => (
                TypeRef {
                    rust: quote!(#ty).to_string(),
                },
                None,
            ),
        },
    }
}

fn result_types(ty: &Type) -> Option<(String, String)> {
    if let Type::Path(tp) = ty {
        let seg = tp.path.segments.last()?;
        if seg.ident == "Result" {
            let syn::PathArguments::AngleBracketed(args) = &seg.arguments else {
                return None;
            };
            let mut parts = args.args.iter().filter_map(|arg| match arg {
                syn::GenericArgument::Type(ty) => Some(quote!(#ty).to_string()),
                _ => None,
            });
            let ok = parts.next()?;
            let err = parts.next()?;
            return Some((ok, err));
        }
    }
    None
}

fn receiver_kind(inputs: &syn::punctuated::Punctuated<FnArg, syn::token::Comma>) -> ReceiverKind {
    match inputs.first() {
        Some(FnArg::Receiver(r)) if r.reference.is_some() && r.mutability.is_some() => ReceiverKind::MutRef,
        Some(FnArg::Receiver(r)) if r.reference.is_some() => ReceiverKind::Ref,
        Some(FnArg::Receiver(_)) => ReceiverKind::Value,
        _ => ReceiverKind::Static,
    }
}

fn ownership_from_type(ty: &Type) -> Ownership {
    match ty {
        Type::Reference(r) => Ownership::Borrowed {
            mutable: r.mutability.is_some(),
            lifetime: r.lifetime.as_ref().map(|lt| lt.to_string()),
        },
        _ => Ownership::Owned,
    }
}

fn docs_from_attrs(attrs: &[syn::Attribute]) -> Docs {
    let mut lines = Vec::new();
    for attr in attrs {
        if attr.path().is_ident("doc") {
            let _ = attr.parse_nested_meta(|meta| {
                if let Ok(value) = meta.value() {
                    let lit: syn::LitStr = value.parse()?;
                    lines.push(lit.value().trim().to_owned());
                }
                Ok(())
            });
        }
    }

    Docs {
        summary: lines.first().cloned().unwrap_or_default(),
        details: if lines.len() > 1 { lines[1..].join("\n") } else { String::new() },
        attributes: Vec::new(),
    }
}

fn map_visibility(vis: &syn::Visibility) -> Visibility {
    match vis {
        syn::Visibility::Public(_) => Visibility::Public,
        syn::Visibility::Restricted(r) => Visibility::Restricted(quote!(#r).to_string()),
        syn::Visibility::Inherited => Visibility::Private,
    }
}

#[cfg(test)]
mod tests {
    use super::parse_source_to_ir;

    #[test]
    fn parses_tensor_example() {
        let src = r#"
        /// Tensor represents vector data
        pub struct Tensor {
            /// data values
            pub data: Vec<f32>
        }

        impl Tensor {
            pub fn add(&self, other: Tensor) -> Tensor { other }
        }
        "#;

        let package = parse_source_to_ir("fixture", src).expect("parser should succeed");
        let module = &package.modules[0];
        assert_eq!(module.classes[0].name, "Tensor");
        assert_eq!(module.impls[0].methods[0].name, "add");
        assert_eq!(module.impls[0].methods[0].params[0].ty.rust, "Tensor");
    }
}

