use proc_macro2::TokenStream;
use quote::quote;
use syn::{Block, ItemFn, token::Async};

pub fn try_catch(func: syn::ItemFn, assert_unwind_safe: bool) -> TokenStream {
    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = func;

    let block = match &sig.asyncness {
        Some(async_tok) => try_catch_async(&block, async_tok, assert_unwind_safe),
        None => try_catch_plain(&block, assert_unwind_safe),
    };

    quote! {
        #(#attrs)*
        #vis #sig {
            #block
        }
    }
}

fn unwind_safe(ts: TokenStream, assert_unwind_safe: bool) -> TokenStream {
    if assert_unwind_safe {
        quote! {
            std::panic::AssertUnwindSafe(#ts)
        }
    } else {
        ts
    }
}

fn try_catch_async(block: &Block, async_tok: &Async, assert_unwind_safe: bool) -> TokenStream {
    let ts = quote! {
        #async_tok move {
            #block
        }
    };
    let fut = unwind_safe(ts, assert_unwind_safe);
    quote! {
        anythrow::try_catch_fut(#fut).await
    }
}

fn try_catch_plain(block: &Block, assert_unwind_safe: bool) -> TokenStream {
    let ts = quote! {
        move || {
            #block
        }
    };
    let block = unwind_safe(ts, assert_unwind_safe);
    quote! {
        anythrow::try_catch(#block)
    }
}
