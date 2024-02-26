use proc_macro::{token_stream::IntoIter as TokenTreeIter, TokenStream};
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{FnArg, ReturnType, ItemFn};
use dyn_clone::clone_box;
use tauriless_common::url::TAURILESS_SYNC_PROTOCOL;

mod impls_asserts;

use impls_asserts::{
    extend_with_serde_deserialize_impls_asserts,
    extend_with_serde_serialize_impls_assert,
    CloneableIterator,
};


struct Commands(syn::punctuated::Punctuated<syn::Ident, syn::token::Comma>);
impl syn::parse::Parse for Commands {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content = syn::punctuated::Punctuated::<syn::Ident, syn::token::Comma>::parse_terminated(input)?;
        Ok(Commands(content))
    }
}

fn extend_with_command(ts: &mut proc_macro2::TokenStream, fn_item: &ItemFn, fn_typed_args: &dyn CloneableIterator<&syn::PatType>) {
    let name: &syn::Ident = &fn_item.sig.ident;
    let name_str = name.to_string();
    let url_name = name_str.replace("_", "-");
    let asyncness = &fn_item.sig.asyncness;
    let cmd_name = format!("__command_{name}");
    let cmd_name = syn::Ident::new(&cmd_name, name.span());
    let typed_args_count = clone_box(fn_typed_args).count();
    let types_iter = clone_box(fn_typed_args).map(|pat_type| &pat_type.ty);
    let args_iter = (0..typed_args_count).map(|i| syn::Ident::new(&format!("arg{}", i), name.span()));
    let args_iter_clone = args_iter.clone();
    let return_type = match &fn_item.sig.output {
        ReturnType::Default => quote! {()},
        ReturnType::Type(_right_arrow, ty) => quote! {#ty},
    };
    let trait_impl = if asyncness.is_none() {
        quote! {
            impl tauriless::SyncCommand for #cmd_name {
                type Args = (#(#types_iter),*);
                type RetTy = #return_type;
                const NAME: &'static str = #name_str;
                #[doc = "URLs can't contain underscores, so they are replaced with dashes"]
                const URL_NAME: &'static str = #url_name;

                fn command( (#(#args_iter),*): Self::Args ) -> Self::RetTy {
                    #name(#(#args_iter_clone),*)
                }
            }
        }
    } else {
        todo!()
    };
    let cmd = quote! {
        #[allow(non_camel_case_types)]
        struct #cmd_name;

        #trait_impl
    };

    ts.extend(cmd);
}

#[proc_macro_attribute]
pub fn command(attr: TokenStream, item: TokenStream) -> TokenStream {
    {
        let mut tt_iter: TokenTreeIter = attr.into_iter();
        assert!(
            tt_iter.next().is_none(),
            "The {fn_name} attribute does not take any arguments",
            fn_name = stringify!(command)
        );
    }

    let fn_item = syn::parse_macro_input!(item as ItemFn);
    let fn_sig: &syn::Signature = &fn_item.sig;

    let inputs: &Punctuated<FnArg, Comma> = &fn_sig.inputs;
    let return_type: &ReturnType = &fn_sig.output;

    let mut ts = proc_macro2::TokenStream::new();
    let fn_typed_args: Box<dyn CloneableIterator<&syn::PatType>> = extend_with_serde_deserialize_impls_asserts(&mut ts, inputs);
    let fn_typed_args: &dyn CloneableIterator<&syn::PatType> = &*fn_typed_args;
    extend_with_serde_serialize_impls_assert(&mut ts, return_type);
    extend_with_command(&mut ts, &fn_item, fn_typed_args);
    ts.extend(quote!(#fn_item));
    ts.into()
}

#[proc_macro]
pub fn commands(input: TokenStream) -> TokenStream {
    let comma_separated_commands = syn::parse_macro_input!(input as Commands);
    let comma_separated_commands = comma_separated_commands.0.clone().into_iter().map(|ident| {
        let ident_str = ident.to_string();
        let cmd_name = format!("__command_{ident_str}");
        syn::Ident::new(&cmd_name, ident.span())
    });

    let mut branches = proc_macro2::TokenStream::new();

    for cmd in comma_separated_commands {
        branches.extend(quote! {
            <#cmd as tauriless::Command>::URL_NAME if !<#cmd as tauriless::Command>::IS_ASYNC => {
                let args: <#cmd as tauriless::Command>::Args = match tauriless::pot::from_slice(body.as_slice()) {
                    Ok(args) => args,
                    Err(e) => return tauriless::handle_deserialization_error(<#cmd as tauriless::Command>::NAME, e),
                };
                let ret: <#cmd as tauriless::Command>::RetTy = <#cmd as tauriless::Command>::sync_command(args);
                tauriless::pot::to_vec(&ret)
            },
        });
    }
    branches.extend(quote! {
        _ => return tauriless::handle_unknown_command(path),
    });

    let body_tail = quote! {
        let resp_body: std::result::Result::<Vec<u8>, tauriless::pot::Error> = match path {
            #branches
        };
        let resp_body: Vec<u8> = match resp_body {
            Ok(body) => body,
            Err(e) => return tauriless::handle_serialization_error(e),
        };
        #[cfg(debug_assertions)]
        println!("Sending a response: {:#?}", resp_body);
        wry::http::response::Response::builder()
            .status(wry::http::StatusCode::OK)
            .header(
                wry::http::header::ACCESS_CONTROL_ALLOW_ORIGIN,
                wry::http::HeaderValue::from_static("*"),
            )
            .body(std::borrow::Cow::Owned(resp_body))
            .unwrap()
    };

    let ts = quote! {
        {
            // Using closures caused an error.
            fn commands<'a>(builder: wry::WebViewBuilder<'a>) -> wry::WebViewBuilder<'a> {
                builder.with_custom_protocol( #TAURILESS_SYNC_PROTOCOL.to_string(), | req: wry::http::request::Request<Vec<u8>> | {
                    let (parts, body): (wry::http::request::Parts, Vec<u8>) = req.into_parts();
                    let uri: wry::http::uri::Uri = parts.uri;
                    let path: &str = uri.path();
                    let path: &str = path.trim_start_matches('/');
        
                    #body_tail
                })
            }
            commands
        }
    };
    
    ts.into()
}
