//! Procedural macros for `powerfmt`.
//!
//! This crate is an implementation detail of `powerfmt` and should not be used directly.

#![allow(
    clippy::missing_const_for_fn, // irrelevant for proc macros
    clippy::std_instead_of_core, // irrelevant for proc macros
    clippy::std_instead_of_alloc, // irrelevant for proc macros
    clippy::alloc_instead_of_core, // irrelevant for proc macros
)]

/// Macros for `SmartDisplay`.
mod smart_display {
    pub(crate) mod delegate;
    pub(crate) mod private_metadata;
}

use proc_macro::TokenStream;

/// Declare an attribute macro.
macro_rules! attribute {
    ($($mod:ident)::+ => $public:ident) => {
        #[doc = concat!("The `#[", attribute!(@stringify $($mod)+), "]` attribute.")]
        #[proc_macro_attribute]
        pub fn $public(attr: TokenStream, item: TokenStream) -> TokenStream {
            match $($mod)::+::implementation(attr.into(), item.into()) {
                Ok(ts) => ts.into(),
                Err(err) => err.to_compile_error().into(),
            }
        }
    };
    (@stringify $first:ident $($remaining:ident)*) => {
        concat!(stringify!($first), $("::", stringify!($remaining),)*)
    };
}

attribute!(smart_display::delegate => smart_display_delegate);
attribute!(smart_display::private_metadata => smart_display_private_metadata);
