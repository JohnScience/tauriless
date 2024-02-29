#![doc = include_str!("../README.md")]

use dyn_clone::clone_box;
use proc_macro::{token_stream::IntoIter as TokenTreeIter, TokenStream};
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{FnArg, ItemFn, ReturnType};
use tauriless_common::url::TAURILESS_ASYNC_PROTOCOL;

mod impls_asserts;

use impls_asserts::{
    extend_with_serde_deserialize_impls_asserts, extend_with_serde_serialize_impls_assert,
    CloneableIterator,
};

struct Commands(syn::punctuated::Punctuated<syn::Ident, syn::token::Comma>);
impl syn::parse::Parse for Commands {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content =
            syn::punctuated::Punctuated::<syn::Ident, syn::token::Comma>::parse_terminated(input)?;
        Ok(Commands(content))
    }
}

fn extend_with_command(
    ts: &mut proc_macro2::TokenStream,
    fn_item: &ItemFn,
    fn_typed_args: &dyn CloneableIterator<&syn::PatType>,
) {
    let name: &syn::Ident = &fn_item.sig.ident;
    let name_str = name.to_string();
    let url_name = name_str.replace("_", "-");
    let asyncness = &fn_item.sig.asyncness;
    let cmd_name = format!("__command_{name}");
    let cmd_name = syn::Ident::new(&cmd_name, name.span());
    let typed_args_count = clone_box(fn_typed_args).count();
    let types_iter = clone_box(fn_typed_args).map(|pat_type| &pat_type.ty);
    let args_iter =
        (0..typed_args_count).map(|i| syn::Ident::new(&format!("arg{}", i), name.span()));
    let args_iter_clone1 = args_iter.clone();
    let args_iter_clone2 = args_iter.clone();
    let return_type = match &fn_item.sig.output {
        ReturnType::Default => quote! {()},
        ReturnType::Type(_right_arrow, ty) => quote! {#ty},
    };
    let trait_impl = if asyncness.is_none() {
        quote! {
            impl tauriless::Command for #cmd_name {
                type Args = (#(#types_iter),*);
                type RetTy = #return_type;
                const NAME: &'static str = #name_str;
                #[doc = "URLs can't contain underscores, so they are replaced with dashes"]
                const URL_NAME: &'static str = #url_name;
                const IS_ASYNC: bool = false;

                fn sync_command( (#(#args_iter),*): Self::Args ) -> Self::RetTy {
                    #name(#(#args_iter_clone1),*)
                }

                #[allow(unused_variables)]
                fn async_command( (#(#args_iter_clone2),*): Self::Args ) -> impl std::future::Future<Output = Self::RetTy> {
                    async move { unimplemented!() }
                }
            }
        }
    } else {
        quote! {
            impl tauriless::Command for #cmd_name {
                type Args = (#(#types_iter),*);
                #[doc = "The return type of the command, modulo the Future wrapper."]
                type RetTy = #return_type;
                const NAME: &'static str = #name_str;
                #[doc = "URLs can't contain underscores, so they are replaced with dashes"]
                const URL_NAME: &'static str = #url_name;
                const IS_ASYNC: bool = true;

                #[allow(unused_variables)]
                fn sync_command( (#(#args_iter),*): Self::Args ) -> Self::RetTy {
                    todo!()
                }


                fn async_command( (#(#args_iter_clone2),*): Self::Args ) -> impl std::future::Future<Output = Self::RetTy> {
                    async move {
                        #name(#(#args_iter_clone1),*).await
                    }
                }
            }
        }
    };
    let cmd = quote! {
        #[allow(non_camel_case_types)]
        struct #cmd_name;

        #trait_impl
    };

    ts.extend(cmd);
}

/// The `tauriless` equivalent of [`#[tauri::command]`](https://tauri.app/v1/guides/features/command/).
///
/// **Note that running asynchronous commands requires global tokio runtime.
/// See [`tokio::runtime::Runtime::enter`](https://docs.rs/tokio/latest/tokio/runtime/struct.Runtime.html#method.enter).**
///
/// ## Example
///
/// ```rust, no_run
/// use tao::{
///     event::{Event, StartCause, WindowEvent},
///     event_loop::{ControlFlow, EventLoop},
///     window::WindowBuilder,
/// };
/// use tauriless::{command, commands, WebViewBuilderExt};
/// use wry::WebViewBuilder;
///
/// #[command]
/// fn argsless_sync_command() {}
///
/// #[command]
/// async fn async_command_with_args(n: i32) -> i32 {
///    // some async code
///    n * 2
/// }
///
/// fn main() -> wry::Result<()>     {
///    let rt = tokio::runtime::Builder::new_multi_thread()
///        .enable_all()
///        .build()
///        .unwrap();
///    // This allows us to use tokio::spawn inside wry asynchronous custom protocol handlers.
///    // Since wry doesn't allow us to pass a runtime to the WebViewBuilder, we have to use a global runtime.
///    let _rt_guard = rt.enter();
///
///    let event_loop = EventLoop::new();
///    let window = WindowBuilder::new()
///        .with_title("My Tauriless App")
///        .build(&event_loop)
///        .unwrap();
///
///    // ...
///    
///    let _webview = WebViewBuilder::new(&window)
///        // ...
///        .with_tauriless_commands(commands!(argsless_sync_command, async_command_with_args))
///        .build()?;
///
///    event_loop.run(move |event, _, control_flow| {
///        *control_flow = ControlFlow::Wait;
///
///        match event {
///            Event::NewEvents(StartCause::Init) => (),
///            Event::WindowEvent {
///                event: WindowEvent::CloseRequested,
///                ..
///            } => *control_flow = ControlFlow::Exit,
///            _ => (),
///        }
///    });
/// }
/// ```
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
    let fn_typed_args: Box<dyn CloneableIterator<&syn::PatType>> =
        extend_with_serde_deserialize_impls_asserts(&mut ts, inputs);
    let fn_typed_args: &dyn CloneableIterator<&syn::PatType> = &*fn_typed_args;
    extend_with_serde_serialize_impls_assert(&mut ts, return_type);
    extend_with_command(&mut ts, &fn_item, fn_typed_args);
    ts.extend(quote!(#fn_item));
    ts.into()
}

/// Creates an object that can be accepted by `tauriless::WebViewBuilderExt::with_tauriless_commands`.
///
/// **Note that running asynchronous commands requires global tokio runtime.
/// See [`tokio::runtime::Runtime::enter`](https://docs.rs/tokio/latest/tokio/runtime/struct.Runtime.html#method.enter).**
///
/// ## Example
///
/// ```rust, no_run
/// use tao::{
///     event::{Event, StartCause, WindowEvent},
///     event_loop::{ControlFlow, EventLoop},
///     window::WindowBuilder,
/// };
/// use tauriless::{command, commands, WebViewBuilderExt};
/// use wry::WebViewBuilder;
///
/// #[command]
/// fn argsless_sync_command() {}
///
/// #[command]
/// async fn async_command_with_args(n: i32) -> i32 {
///    // some async code
///    n * 2
/// }
///
/// fn main() -> wry::Result<()>     {
///    let rt = tokio::runtime::Builder::new_multi_thread()
///        .enable_all()
///        .build()
///        .unwrap();
///    // This allows us to use tokio::spawn inside wry asynchronous custom protocol handlers.
///    // Since wry doesn't allow us to pass a runtime to the WebViewBuilder, we have to use a global runtime.
///    let _rt_guard = rt.enter();
///
///    let event_loop = EventLoop::new();
///    let window = WindowBuilder::new()
///        .with_title("My Tauriless App")
///        .build(&event_loop)
///        .unwrap();
///
///    // ...
///    
///    let _webview = WebViewBuilder::new(&window)
///        // ...
///        .with_tauriless_commands(commands!(argsless_sync_command, async_command_with_args))
///        .build()?;
///
///    event_loop.run(move |event, _, control_flow| {
///        *control_flow = ControlFlow::Wait;
///
///        match event {
///            Event::NewEvents(StartCause::Init) => (),
///            Event::WindowEvent {
///                event: WindowEvent::CloseRequested,
///                ..
///            } => *control_flow = ControlFlow::Exit,
///            _ => (),
///        }
///    });
/// }
/// ```
#[proc_macro]
pub fn commands(input: TokenStream) -> TokenStream {
    let comma_separated_commands = syn::parse_macro_input!(input as Commands);
    let command_struct_idents = comma_separated_commands.0.clone().into_iter().map(|ident| {
        let ident_str = ident.to_string();
        let cmd_name = format!("__command_{ident_str}");
        syn::Ident::new(&cmd_name, ident.span())
    });
    let command_struct_idents_clone = command_struct_idents.clone();

    let mut async_proto_branches = proc_macro2::TokenStream::new();

    for cmd in command_struct_idents_clone {
        async_proto_branches.extend(quote! {
            <#cmd as tauriless::Command>::URL_NAME => {
                let args: <#cmd as tauriless::Command>::Args = match tauriless::slice_to_deserialize(body.as_slice()) {
                    Ok(args) => args,
                    Err(e) => return responder.respond(tauriless::handle_deserialization_error(<#cmd as tauriless::Command>::NAME, e)),
                };
                if <#cmd as tauriless::Command>::IS_ASYNC {
                    let handle = tokio::runtime::Handle::try_current().expect("Using async protocol handler requires entering the tokio runtime context prior to that. Use `let _rt_guard = rt.enter()` to enter the runtime context. See <https://docs.rs/tokio/latest/tokio/runtime/struct.Runtime.html#method.enter>.");
                    handle.spawn(async move {
                        let ret: <#cmd as tauriless::Command>::RetTy = <#cmd as tauriless::Command>::async_command(args).await;
                        match tauriless::serialize_to_vec_u8(&ret) {
                            Ok(ret) => {
                                responder.respond(wry::http::response::Response::builder()
                                    .status(wry::http::StatusCode::OK)
                                    .header(
                                        wry::http::header::ACCESS_CONTROL_ALLOW_ORIGIN,
                                        wry::http::HeaderValue::from_static("*"),
                                    )
                                    .body(std::borrow::Cow::Owned(ret))
                                    .unwrap()
                                );
                            },
                            Err(e) => {
                                responder.respond(tauriless::handle_serialization_error(e));
                            }
                        }
                    });
                } else {
                    let ret: <#cmd as tauriless::Command>::RetTy = <#cmd as tauriless::Command>::sync_command(args);
                    match tauriless::serialize_to_vec_u8(&ret) {
                        Ok(ret) => {
                            responder.respond(wry::http::response::Response::builder()
                                .status(wry::http::StatusCode::OK)
                                .header(
                                    wry::http::header::ACCESS_CONTROL_ALLOW_ORIGIN,
                                    wry::http::HeaderValue::from_static("*"),
                                )
                                .body(std::borrow::Cow::Owned(ret))
                                .unwrap()
                            );
                        },
                        Err(e) => {
                            responder.respond(tauriless::handle_serialization_error(e));
                        }
                    }
                }
            },
        });
    }

    async_proto_branches.extend(quote! {
        _ => responder.respond(tauriless::handle_unknown_command(path)),
    });

    let ts = quote! {
        {
            // Using closures caused an error.
            fn commands<'a>(builder: wry::WebViewBuilder<'a>) -> wry::WebViewBuilder<'a> {
                builder.with_asynchronous_custom_protocol( #TAURILESS_ASYNC_PROTOCOL.to_string(), | req: wry::http::request::Request<Vec<u8>>, responder: wry::RequestAsyncResponder | {
                    let (parts, body): (wry::http::request::Parts, Vec<u8>) = req.into_parts();
                    let uri: wry::http::uri::Uri = parts.uri;
                    let path: &str = uri.path();
                    let path: &str = path.trim_start_matches('/');
                    match path {
                        #async_proto_branches
                    };
                })
            }
            commands
        }
    };

    ts.into()
}
