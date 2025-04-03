use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemFn, ReturnType};

/// Converts a function that returns a `Result<T,E>` into an a function that returns a `ActorResult<Result<T, E>>`
///
/// Example:
///
/// ```rust
/// pub struct App {}
///
/// impl App {
///     #[act_zero_ext::into_actor_result]
///     async fn hello(&self, name: String) -> Result<String, Box<dyn std::error::Error>> {
///         Ok(format!("Hello, {}!", name))
///     }
/// }
/// ```
///
/// Will be converted to:
///
/// ```rust
/// pub struct App {}
///
/// impl App {
///     pub async fn hello(&self, name: String) -> ActorResult<Result<String, Box<dyn std::error::Error>>> {
///         let result = self.do_hello(name).await;
///         Produces::Ok(result)
///     }
///
///     async fn do_hello(&self, name: String) -> Result<String, Box<dyn std::error::Error>> {
///         Ok(format!("Hello, {}!", name))
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn into_actor_result(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the function
    let input_fn = parse_macro_input!(item as ItemFn);

    // Clone for the do_ version
    let mut do_fn = input_fn.clone();
    let fn_name = &input_fn.sig.ident;
    let do_fn_name = format_ident!("do_{}", fn_name);

    do_fn.sig.ident = do_fn_name.clone();
    do_fn.vis = syn::Visibility::Inherited;

    // Extract information for the wrapper function
    let vis = &input_fn.vis;
    let asyncness = &input_fn.sig.asyncness;
    let generics = &input_fn.sig.generics;
    let inputs = &input_fn.sig.inputs;

    // Extract return type for ActorResult wrapper
    let return_type = match &input_fn.sig.output {
        ReturnType::Default => quote! { () },
        ReturnType::Type(_, ty) => quote! { #ty },
    };

    // Get argument names for passing to do_ function
    let arg_names = inputs
        .iter()
        .filter_map(|arg| {
            if let syn::FnArg::Typed(pat_type) = arg {
                if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                    if pat_ident.ident != "self" {
                        return Some(&pat_ident.ident);
                    }
                }
            }
            None
        })
        .collect::<Vec<_>>();

    // Create the wrapper function
    let wrapper_fn = if asyncness.is_some() {
        quote! {
            #vis #asyncness fn #fn_name #generics (#inputs) -> act_zero::ActorResult<#return_type> {
                let result = self.#do_fn_name(#(#arg_names),*).await;
                act_zero::Produces::ok(result)
            }
        }
    } else {
        quote! {
            #vis fn #fn_name #generics (#inputs) -> act_zero::ActorResult<#return_type> {
                let result = self.#do_fn_name(#(#arg_names),*);
                act_zero::Produces::ok(result)
            }
        }
    };

    // Generate the final code
    let result = quote! {
        #wrapper_fn

        #do_fn
    };

    result.into()
}
