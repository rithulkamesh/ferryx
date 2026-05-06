use ferryx_ir::{
    Docs, IrClass, IrField, IrImpl, IrItem, IrMethod, IrParam, Ownership, ReceiverKind, TypeRef, Visibility,
};
use proc_macro::TokenStream;
use quote::quote;
use syn::{FnArg, ImplItem, ItemImpl, ItemStruct, ReturnType, Type, Visibility as SynVisibility};

#[proc_macro_attribute]
pub fn ferryx(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let cloned = item.clone();
    if let Ok(item_struct) = syn::parse::<ItemStruct>(cloned.clone()) {
        return expand_struct(item_struct).into();
    }
    if let Ok(item_impl) = syn::parse::<ItemImpl>(cloned) {
        return expand_impl(item_impl).into();
    }

    let err = syn::Error::new(proc_macro2::Span::call_site(), "#[ferryx] supports structs and impl blocks only");
    err.to_compile_error().into()
}

fn expand_struct(item: ItemStruct) -> proc_macro2::TokenStream {
    let name = item.ident.to_string();
    let vis = map_visibility(&item.vis);
    let fields = item
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
                ownership: Ownership::Owned,
                docs: docs_from_attrs(&field.attrs),
            })
        })
        .collect::<Vec<_>>();

    let descriptor = IrItem::Class(IrClass {
        id: format!("{name}::class"),
        module_id: "crate".into(),
        name: name.clone(),
        visibility: vis,
        docs: docs_from_attrs(&item.attrs),
        fields,
    });

    let json = serde_json::to_string(&descriptor).expect("serialize class descriptor");
    let original = quote!(#item);
    quote! {
        #original
        ::ferryx_runtime::inventory::submit! {
            ::ferryx_runtime::ReflectionRecord {
                module_path: module_path!(),
                item_name: #name,
                item_json: #json,
            }
        }
    }
}

fn expand_impl(item: ItemImpl) -> proc_macro2::TokenStream {
    let self_ty = &item.self_ty;
    let target = quote!(#self_ty).to_string();
    let item_name = target.clone();
    let methods = item
        .items
        .iter()
        .filter_map(|impl_item| match impl_item {
            ImplItem::Fn(func) => {
                let receiver = receiver_kind(&func.sig.inputs);
                let params = func
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
                                ownership: Ownership::Owned,
                            })
                        }
                        FnArg::Receiver(_) => None,
                    })
                    .collect::<Vec<_>>();

                let (output, error) = extract_output_and_error(&func.sig.output);
                Some(IrMethod {
                    name: func.sig.ident.to_string(),
                    receiver,
                    docs: docs_from_attrs(&func.attrs),
                    is_async: func.sig.asyncness.is_some(),
                    async_runtime: None,
                    params,
                    output,
                    error,
                })
            }
            _ => None,
        })
        .collect::<Vec<_>>();

    let descriptor = IrItem::Impl(IrImpl {
        id: format!("{target}::impl"),
        module_id: "crate".into(),
        target: TypeRef { rust: target },
        trait_name: None,
        methods,
    });

    let json = serde_json::to_string(&descriptor).expect("serialize impl descriptor");
    let original = quote!(#item);
    quote! {
        #original
        ::ferryx_runtime::inventory::submit! {
            ::ferryx_runtime::ReflectionRecord {
                module_path: module_path!(),
                item_name: #item_name,
                item_json: #json,
            }
        }
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

    let summary = lines.first().cloned().unwrap_or_default();
    let details = if lines.len() > 1 { lines[1..].join("\n") } else { String::new() };
    Docs {
        summary,
        details,
        attributes: Vec::new(),
    }
}

fn receiver_kind(inputs: &syn::punctuated::Punctuated<FnArg, syn::token::Comma>) -> ReceiverKind {
    match inputs.first() {
        Some(FnArg::Receiver(r)) if r.reference.is_some() && r.mutability.is_some() => ReceiverKind::MutRef,
        Some(FnArg::Receiver(r)) if r.reference.is_some() => ReceiverKind::Ref,
        Some(FnArg::Receiver(_)) => ReceiverKind::Value,
        _ => ReceiverKind::Static,
    }
}

fn extract_output_and_error(output: &ReturnType) -> (TypeRef, Option<TypeRef>) {
    match output {
        ReturnType::Default => (TypeRef { rust: "()".into() }, None),
        ReturnType::Type(_, ty) => match extract_result_types(ty) {
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

fn extract_result_types(ty: &Type) -> Option<(String, String)> {
    if let Type::Path(tp) = ty {
        let segment = tp.path.segments.last()?;
        if segment.ident == "Result" {
            let syn::PathArguments::AngleBracketed(args) = &segment.arguments else {
                return None;
            };
            let mut iter = args.args.iter().filter_map(|arg| match arg {
                syn::GenericArgument::Type(t) => Some(quote!(#t).to_string()),
                _ => None,
            });
            let ok = iter.next()?;
            let err = iter.next()?;
            return Some((ok, err));
        }
    }
    None
}

fn map_visibility(vis: &SynVisibility) -> Visibility {
    match vis {
        SynVisibility::Public(_) => Visibility::Public,
        SynVisibility::Restricted(r) => Visibility::Restricted(quote!(#r).to_string()),
        SynVisibility::Inherited => Visibility::Private,
    }
}

