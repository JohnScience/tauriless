use quote::quote;

// "Impls expression" is a stipulative definition for an expression that evaluates to true
// if the type implements a trait, and false otherwise.
// Hence, `serde::Deserialize` "impls expression" is an expression that evaluates to true
// if the type implements `serde::Deserialize`, and false otherwise.
pub(super) fn extend_with_serde_deserialize_impls_expr(
    ts: &mut proc_macro2::TokenStream,
    fn_arg_type: &syn::Type,
) {
    let impls_const_name: syn::Ident = syn::parse_str("IMPLS_SERDE_DESERIALIZE").unwrap();
    let ty = fn_arg_type;
    ts.extend(quote! {
        {
            trait DoesNotImplTrait {
                const #impls_const_name: bool = false;
            }

            impl<T: ?Sized> DoesNotImplTrait for T {}

            struct Wrapper<T: ?Sized> (core::marker::PhantomData<T>);

            #[allow(dead_code)]
            impl<'a, T: ?Sized + serde::Deserialize<'a>> Wrapper<T> {
                const #impls_const_name: bool = true;
            }

            <Wrapper<#ty>>::#impls_const_name
        }
    });
}

pub(super) fn extend_with_serde_serialize_impls_expr(
    ts: &mut proc_macro2::TokenStream,
    return_type: &syn::ReturnType,
) {
    let ty = match return_type {
        syn::ReturnType::Type(_right_arrow, ty) => ty,
        syn::ReturnType::Default => {
            return;
        }
    };
    let trait_path: syn::Path = syn::parse_str("serde::Serialize").unwrap();
    let impls_const_name: syn::Ident = syn::parse_str("IMPLS_SERDE_SERIALIZE").unwrap();
    ts.extend(quote! {
        {
            trait DoesNotImplTrait {
                const #impls_const_name: bool = false;
            }

            impl<T: ?Sized> DoesNotImplTrait for T {}

            struct Wrapper<T: ?Sized> (core::marker::PhantomData<T>);

            #[allow(dead_code)]
            impl<T: ?Sized + #trait_path> Wrapper<T> {
                const #impls_const_name: bool = true;
            }

            <Wrapper<#ty>>::#impls_const_name
        }
    });
}
