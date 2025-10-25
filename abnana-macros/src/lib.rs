use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, ItemFn, FnArg, Pat};

/// An attribute-like procedural macro that generates a mirrored function or method
/// with the suffix `_and_trigger`.
///
/// This version correctly handles `async` functions/methods by preserving the
/// `async` keyword and adding `.await` to the internal call when necessary.
#[proc_macro_attribute]
pub fn create_trigger_fn(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // 1. Parse the input method as a function item
    let input = parse_macro_input!(item as ItemFn);

    // 2. Extract essential components
    let vis = &input.vis;
    let original_name = &input.sig.ident;
    let generics = &input.sig.generics;
    let inputs = &input.sig.inputs;
    let output = &input.sig.output;
    let block = &input.block;

    // --- ASYNC SUPPORT ---
    let asyncness = &input.sig.asyncness; // Captures `Option<Token![async]>`

    // 3. Determine if the function has a receiver (i.e., is a method)
    let has_receiver = inputs.iter().any(|arg| matches!(arg, FnArg::Receiver(_)));

    // 4. Create the new function name
    let trigger_name_str = format!("{}_and_trigger", original_name);
    let trigger_name = Ident::new(&trigger_name_str, original_name.span());

    // 5. Collect argument tokens for passing during the call (excluding the receiver)
    let call_args: Vec<_> = inputs.iter().filter_map(|arg| {
        match arg {
            FnArg::Receiver(_) => None,
            FnArg::Typed(pat_type) => {
                if let Pat::Ident(pat_ident) = &*pat_type.pat {
                    Some(quote!{ #pat_ident })
                } else {
                    Some(quote!{ compile_error!("Only simple identifier patterns are supported for arguments.") })
                }
            }
        }
    }).collect();

    // 6. Define the base function call expression (without .await)
    let original_call = if has_receiver {
        // Method call syntax: self.original_name(...)
        quote! { self.#original_name(#(#call_args),*) }
    } else {
        // Function call syntax: original_name(...)
        quote! { #original_name(#(#call_args),*) }
    };

    // 7. Conditionally add `.await` if the function is async
    let final_call_expr = if asyncness.is_some() {
        quote! { #original_call.await }
    } else {
        original_call
    };

    // 8. Determine the display name for the trigger message
    let display_context = if has_receiver {
        quote! { stringify!(Self), stringify!(#original_name) }
    } else {
        quote! { "function", stringify!(#original_name) }
    };

    // 9. Construct the final output token stream
    let expanded = quote! {
        // --- 1. The original definition (kept intact) ---
        #vis #asyncness fn #original_name #generics (#inputs) #output #block

        // --- 2. The new '..._and_trigger' definition (includes #asyncness) ---
        #vis #asyncness fn #trigger_name #generics (#inputs) #output
        {
            // The required print statement (the "trigger")
            println!("\n[TRIGGER] Executing wrapped {}: '{}'", #display_context);

            // Conditional call expression (includes .await if async)
            #final_call_expr
        }
    };

    expanded.into()
}