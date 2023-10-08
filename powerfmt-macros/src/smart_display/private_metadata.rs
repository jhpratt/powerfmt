//! The `#[smart_display::private_metadata]` attribute.

use std::iter;

use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::{Paren, Pub};
use syn::{
    Error, ItemStruct, Path, PathArguments, PathSegment, Result, Token, VisRestricted, Visibility,
};

/// Implementation of the `#[smart_display::private_metadata]` attribute.
pub(crate) fn implementation(attr: TokenStream, item: TokenStream) -> Result<TokenStream> {
    if !attr.is_empty() {
        return Err(Error::new_spanned(
            attr,
            "`delegate_display` does not take any arguments",
        ));
    }

    let mut input = syn::parse2::<ItemStruct>(item)?;

    // Public items are not supported by the macro.
    if matches!(input.vis, Visibility::Public(_)) {
        return Err(Error::new_spanned(
            input.vis.into_token_stream(),
            "`metadata` can only be used on non-pubic items",
        ));
    }

    // Preserve the original visibility for the re-export.
    let original_vis = input.vis.clone();
    // Change the visibility of the struct to public, avoiding any compiler errors.
    input.vis = Visibility::Public(Token![pub](input.vis.span()));

    // Change the visibility of fields as necessary.
    for field in &mut input.fields {
        vis_prepend_super(&mut field.vis)?;
    }

    let mod_name = format_ident!(
        "__powerfmt_private_{}",
        input.ident,
        span = Span::call_site() // Avoid a warning about the module not being snake case.
    );

    Ok(quote! {
        mod #mod_name {
            use super::*;

            #[non_exhaustive]
            #input
        }

        #original_vis use #mod_name::*;
    })
}

fn vis_prepend_super(vis: &mut Visibility) -> Result<()> {
    match vis {
        Visibility::Public(_) => {
            // If the visibility is public, there is nothing to do.
            Ok(())
        }
        Visibility::Restricted(VisRestricted { path, in_token, .. }) => {
            let segments = &path.segments;
            let ident = &segments
                .first()
                .or_else(|| segments.last())
                .expect("path should have at least one segment")
                .ident;

            match ident.to_string().as_ref() {
                "crate" => {
                    // If the path is simply `crate`, there is nothing to do. Otherwise, we need to
                    // remove the final segment.
                    if path.get_ident().is_none() {
                        path.segments.pop();
                    }

                    Ok(())
                }
                "super" => {
                    *in_token = in_token.or_else(|| Some(Token![in](path.span())));

                    // Any path starting with `super` needs another `super` prepended.
                    path.segments = iter::once(PathSegment {
                        ident: Token![super](path.span()).into(),
                        arguments: PathArguments::default(),
                    })
                    .chain(path.segments.iter().cloned())
                    .collect();

                    Ok(())
                }
                "self" => {
                    // `pub(self)` -> `pub(super)`
                    //
                    // There is absolutely no reason this should ever be written, but it is valid
                    // syntax.
                    let super_span = path.get_ident().map_or_else(Span::call_site, Ident::span);
                    path.segments = Punctuated::from_iter([PathSegment {
                        ident: Token![super](super_span).into(),
                        arguments: PathArguments::None,
                    }]);

                    Ok(())
                }
                _ => Err(Error::new_spanned(path, "unexpected visibility path")),
            }
        }
        Visibility::Inherited => {
            // inherited visibility -> `pub(super)`
            *vis = Visibility::Restricted(VisRestricted {
                pub_token: Pub::default(),
                paren_token: Paren::default(),
                in_token: None,
                path: Box::new(Path {
                    leading_colon: None,
                    segments: Punctuated::from_iter([PathSegment {
                        ident: format_ident!("super"),
                        arguments: PathArguments::None,
                    }]),
                }),
            });

            Ok(())
        }
    }
}
