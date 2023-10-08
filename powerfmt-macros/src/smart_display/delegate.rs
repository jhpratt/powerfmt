//! The `#[smart_display::delegate]` attribute.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Error, ItemImpl, Result};

/// Implementation of the `#[smart_display::delegate]` attribute.
pub(crate) fn implementation(attr: TokenStream, item: TokenStream) -> Result<TokenStream> {
    if !attr.is_empty() {
        return Err(Error::new_spanned(
            attr,
            "`delegate` does not take any arguments",
        ));
    }

    // This is what we'll be returning. Attribute macros replace, rather than supplement, the
    // original input. This isn't what we want, so start with the original input and extend it
    // later.
    let orig_item = item.clone();
    let input = syn::parse2::<ItemImpl>(item)?;

    // Ensure that the attribute is only being used on implementations of `SmartDisplay`.
    //
    // The compiler doesn't provide access to anything more than a token stream, so matching on the
    // path is the best we can do.
    match &input.trait_ {
        Some((None, trait_path, _))
            if trait_path
                .segments
                .last()
                .expect("path should have at least one segment")
                .ident
                == "SmartDisplay" =>
        {
            let self_ty = &input.self_ty;
            let generics = &input.generics;

            Ok(quote! {
                #orig_item

                impl #generics ::core::fmt::Display for #self_ty {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        #trait_path::fmt(self, f)
                    }
                }
            })
        }
        _ => Err(Error::new_spanned(
            attr,
            "`delegate` can only be used on implementations of `SmartDisplay`",
        )),
    }
}
