use std::collections::HashMap;

use itertools::Itertools;
use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::FnArg;

mod equityped_arg_pats;
mod impls_exprs;

use impls_exprs::{
    extend_with_serde_deserialize_impls_expr, extend_with_serde_serialize_impls_expr,
};
use equityped_arg_pats::EquitypedArgPats;
use dyn_clone::DynClone;

// Raison d'être: https://github.com/rust-lang/rust/issues/32220
pub(super) trait CloneableIterator<T>: Iterator<Item = T> + DynClone {}
impl<T, It> CloneableIterator<T> for It
where
    It: Iterator<Item = T> + DynClone,
{}

// Extends the token stream with a const assertion that checks if the type
// implements `serde::Deserialize`. If it doesn't, the compilation fails with
// a custom error message.
fn extend_with_serde_deserialize_impls_assert(
    ts: &mut proc_macro2::TokenStream,
    ty_str: &str,
    typed_args_for_type: &EquitypedArgPats,
) {
    let mut group_contents = proc_macro2::TokenStream::new();
    extend_with_serde_deserialize_impls_expr(&mut group_contents, &typed_args_for_type.ty);
    let group_contents = proc_macro2::TokenStream::from(group_contents);
    let should_use_plural = typed_args_for_type.arg_pats.len() > 1;
    // stringify!() is a non-const macro, so we can't use it in a const assertion
    let arg_pats: String = typed_args_for_type
        .arg_pats
        .iter()
        .map(|arg_pat| arg_pat.to_token_stream().to_string())
        .map(|s| format!("`{s}`"))
        .join(", ");
    let panic_msg = format!("The type `{ty_str}` of argument{maybe_s} {arg_pats} for the command must implement [`serde::Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html)", maybe_s = if should_use_plural { "s" } else { ""});
    let doc_str = format!("Checks that `{}` implements serde::Deserialize", ty_str);
    ts.extend(quote! {
        #[doc = #doc_str]
        const _: () = if !(#group_contents) {
            panic!(#panic_msg);
        };
    });
}

pub(crate) fn extend_with_serde_serialize_impls_assert(
    ts: &mut proc_macro2::TokenStream,
    return_type: &syn::ReturnType,
) {
    let mut group_contents = proc_macro2::TokenStream::new();
    extend_with_serde_serialize_impls_expr(&mut group_contents, return_type);

    let ty = match return_type {
        syn::ReturnType::Type(_right_arrow, ty) => ty,
        syn::ReturnType::Default => {
            return;
        }
    };

    let ty_str = ty.to_token_stream().to_string();

    let panic_msg = format!("The return type `{ty_str}` of the command must implement [`serde::Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html)");

    ts.extend(quote! {
        #[doc = "Checks that the return type implements serde::Serialize"]
        const _: () = if !(#group_contents) {
            panic!(#panic_msg);
        };
    });
}

pub(crate) fn extend_with_serde_deserialize_impls_asserts<'a,'b>(
    ts: &'a mut proc_macro2::TokenStream,
    args: &'b Punctuated<FnArg, Comma>,
)
// IDEA(lang design): return either
// 1. anonymous enum that would implement Iterator<Item=&'b syn::PatType> + Clone
// 2. Box<dyn CloneableIterator<&'b syn::PatType> + 'b>
// depending on the size of the biggest variant of the enum (or other factors)
-> Box<dyn CloneableIterator<&'b syn::PatType> + 'b> {
    let mut args_iter = args.clone().into_iter();
    let Some(maybe_receiver) = args_iter.next() else {
        return Box::new(std::iter::empty()) as Box<dyn CloneableIterator<&'b syn::PatType>>;
    };

    let FnArg::Typed(first_typed_arg) = maybe_receiver else {
        panic!("The first argument of the command can't be a receiver like `self`, `&self`, or `&mut self`")
    };

    let args_iter = args_iter.map(|arg| {
        let FnArg::Typed(typed_arg) = arg else {
            panic!("The receiver like `self`, `&self`, or `&mut self` can't be a non-first argument of a function");
        };
        typed_arg
    });

    // HashMap from type names to "argument patterns".
    // We collect those to avoid duplicate checks for the same type.
    // Unfortunately, the types are compared syntactically rather than structurally or nominally.
    let hm = {
        let mut hm: HashMap<String, EquitypedArgPats> = HashMap::new();

        for typed_arg in std::iter::once(first_typed_arg)
            .chain(args_iter)
            .into_iter()
        {
            let ty: Box<syn::Type> = typed_arg.ty;
            let pat: Box<syn::Pat> = typed_arg.pat;
            let ty_str = ty.to_token_stream().to_string();
            hm.entry(ty_str)
                .or_insert(EquitypedArgPats::new(ty))
                .push_arg_pat(pat);
        }

        hm
    };

    // TODO: Filter out types that are known to implement `serde::Deserialize`, at least from the standard library.

    for (ty_str, typed_args_for_type) in hm.iter() {
        extend_with_serde_deserialize_impls_assert(ts, ty_str, typed_args_for_type);
    }

    let iter = args.iter().map(|arg| {
        let typed_arg = match arg {
            FnArg::Receiver(_) => std::unreachable!(),
            FnArg::Typed(typed_arg) => typed_arg,
        };
        typed_arg
    });

    Box::new(iter) as Box<dyn CloneableIterator<&'b syn::PatType>>
}
