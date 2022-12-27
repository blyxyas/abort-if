use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::quote;

use syn::{parse_macro_input, parse_quote, parse_str, token::Brace, AttributeArgs, Block, ItemFn};

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
