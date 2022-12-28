//! # Abort-if crate
//! 
//! The `abort_if` procedural macro guarantees that a specific function panics if a condition is met.
//! 
//! ## Example
//! 
//! You can assure that a function won't be used if feature `x` is enabled
//! 
//! ```
//! use abort_if::abort_if;
//! #[abort_if(feature = x)]
//! fn foo() {
//! 	using_that_feature();
//! }
//! 
//! fn main() {
//! 	foo();
//! }
//! ```
//! 
//! This code will panic if that feature is enabled.
//! 
//! ## Features
//! 
//! The default is panicking using `compiler_error!`. This will output the following information:
//! 
//! ```
//! error: Condition was met.
//!  --> src/main.rs:5:1
//!   |
//! 5 | #[abort_if(feature = "x")]
//!   | ^^^^^^^^^^^^^^^^^^^^^^^^^^
//!   |
//!   = note: this error originates in the attribute macro `abort_if` (in Nightly builds, run with -Z macro-backtrace for more info)
//! ```
//! 
//! You can use the feature `custom_abort` to write a custom abort macro. When using this feature, make sure to have a `custom_abort_error!` macro with an `expr` as the argument.
//! 
//! ---
//! 
//! If you use the `custom_abort` feature, you can also use the `keep_going` one. This feature functions that, if your `custom_abort_error` macro works as a warning instead of a hard error, the code will keep going.

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::quote;

use syn::{parse_macro_input, parse_quote, parse_str, token::Brace, AttributeArgs, Block, ItemFn};

/// The main proc-macro. It takes arguments.
/// 
/// ##### Example:
/// 
/// ```rust, ignore
/// #[abort_if(debug_assertions)]
/// fn x() {
/// 	// ...
/// }
/// ```
/// 
/// This will fail if `debug_assertions` is enabled, so it will abort if it isn't on the release mode.
/// 
/// The arguments can have nested conditionals, such as `not` or `any`, like this:
/// 
/// ```rust, ignore
/// #[abort_if(any(debug_assertions, feature = "debug_mode"))]
/// fn x() {
/// 	// ...
/// }
/// ```
/// 
/// This code will abort if either `debug_assertions` is active, or the `debug_mode` feature is enabled.
#[proc_macro_error]
#[proc_macro_attribute]
pub fn abort_if(raw_args: TokenStream, input: TokenStream) -> TokenStream {
    let raw_args_clone = raw_args.clone();
    let mut input = parse_macro_input!(input as ItemFn);
    let args = parse_macro_input!(raw_args_clone as AttributeArgs);

    let throw_err_str: &str;
    if cfg!(feature = "custom_abort") {
        throw_err_str = "custom_abort_error!(\"Condition was met.\");"
    } else {
        throw_err_str = "compile_error!(\"Condition was met.\");"
    }

    let mut alternative = ItemFn {
        attrs: Vec::new(),
        vis: input.vis.clone(),
        sig: input.sig.clone(),
        block: Block {
            brace_token: Brace {
                span: input.block.brace_token.span,
            },
            // stmts: vec![parse_str(&format!("panic!(\"The condition `{}` was met, so the function `{}` panicked\");", raw_args.to_string(), input.sig.ident.to_string())).unwrap()]
            stmts: vec![parse_str(throw_err_str).unwrap()],
        }
        .into(),
    };

	if cfg!(feature = "keep_going") {
		alternative.block.stmts.append(&mut input.block.stmts);
	}

    for arg in args {
        input.attrs.push(parse_quote! {#[cfg(not(#arg))]});
        alternative.attrs.push(parse_quote! {#[cfg(#arg)]});
    }

    TokenStream::from(quote! {
        #input
        #alternative
    })
}
