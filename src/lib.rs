use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, FnArg, ItemFn, Pat, ReturnType};

#[proc_macro_attribute]
pub fn derive_actor(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input function
    let input_fn = parse_macro_input!(item as ItemFn);
    let vis = &input_fn.vis;
    let sig = &input_fn.sig;
    let body = &input_fn.block;

    // Extract function details
    let fn_name = &sig.ident;
    let do_fn_name = format_ident!("do_{}", fn_name);
    let inputs = &sig.inputs;
    let asyncness = &sig.asyncness;
    let generics = &sig.generics;

    // Extract argument names for passing to the do_ function
    let arg_names = inputs
        .iter()
        .filter_map(|arg| {
            match arg {
                FnArg::Receiver(_) => None, // Skip self in argument list
                FnArg::Typed(pat_type) => {
                    if let Pat::Ident(pat_ident) = &*pat_type.pat {
                        Some(&pat_ident.ident)
                    } else {
                        None
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    // Generate the public wrapper function
    let output_type = match &sig.output {
        ReturnType::Default => quote! { () },
        ReturnType::Type(_, ty) => quote! { #ty },
    };

    let wrapper_fn = if asyncness.is_some() {
        quote! {
            #vis #asyncness fn #fn_name #generics (#inputs) -> ActorResult<#output_type> {
                let result = self.#do_fn_name(#(#arg_names),*).await;
                Produces::ok(result)
            }
        }
    } else {
        quote! {
            #vis fn #fn_name #generics (#inputs) -> ActorResult<#output_type> {
                let result = self.#do_fn_name(#(#arg_names),*);
                Produces::ok(result)
            }
        }
    };

    // Generate the private implementation function
    let impl_fn = quote! {
        #vis #asyncness fn #do_fn_name #generics (#inputs) -> #sig.output {
            #body
        }
    };

    // Combine both functions
    let expanded = quote! {
        #impl_fn
    };

    expanded.into()
}
