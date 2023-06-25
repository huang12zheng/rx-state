mod rx_state;

use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::rx_state::ReactiveStateDeriveInput;

/// Processes the given `struct` to create a reactive version by wrapping each
/// field in a `Signal`. This will generate a new `struct` with the given name
/// and implement a `.make_rx()` method on the original that allows turning an
/// instance of the unreactive `struct` into an instance of the reactive one.
///
/// If one of your fields is itself a `struct`, by default it will just be
/// wrapped in a `Signal`, but you can also enable nested fine-grained
/// reactivity by adding the `#[rx(nested)]` helper macro to the field.
/// Fields that have nested reactivity should also use this derive macro.
#[proc_macro_derive(ReactiveState, attributes(rx))]
pub fn reactive_state(input: TokenStream) -> TokenStream {
    let input = match ReactiveStateDeriveInput::from_derive_input(&syn::parse_macro_input!(
        input as DeriveInput
    )) {
        Ok(input) => input,
        Err(err) => return err.write_errors().into(),
    };

    rx_state::make_rx_impl(input).into()
}

#[proc_macro_derive(UnreactiveState)]
pub fn unreactive_state(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    // This is a marker trait, so we barely have to do anything here
    quote! {
        impl ::perseus::state::UnreactiveState for #name {}
    }
    .into()
}

#[test]
fn expand_derive() {
    let input = FromDeriveInput::from_derive_input(&syn::parse_quote! {
        #[derive(ReactiveState)]
        pub struct MyStruct {
            pub a: i32,
            pub b: String,
            pub c: Vec<i32>,
        }
    })
    .unwrap();

    let input = rx_state::make_rx_impl(input).to_string();
    let syntax_tree = syn::parse_file(input.as_str()).unwrap();
    insta::assert_display_snapshot!(prettyplease::unparse(&syntax_tree));
}

#[test]
fn expand_nest() {
    let input = FromDeriveInput::from_derive_input(&syn::parse_quote! {
        #[derive(ReactiveState)]
        pub struct MyStruct {
            #[rx(nested)]
            pub c: Vec<i32>,
            #[rx(nested)]
            pub c: RxVec<i32>
        }
    })
    .unwrap();

    let input = rx_state::make_rx_impl(input).to_string();
    let syntax_tree = syn::parse_file(input.as_str()).unwrap();
    insta::assert_display_snapshot!(prettyplease::unparse(&syntax_tree));
}
