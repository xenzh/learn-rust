// https://cbreeden.github.io/Macros11/

// this feature gate won't be needed starting from 1.15
#![feature(proc_macro, proc_macro_lib)]

// syn crate is essentially:
// * rust lang entities structs (like Path, "std::collections::HashMap")
// * Nom parser definitions to turn text into lang entry structs
// * ToToken trait impls for converting rust lang structs to Tokens
extern crate syn; // rust source code parser

// quote! macro takes piece of valid rust code
// and turns it into tokems collection performing substitutions
// and rolling loops.
// Result can be parsed into proc_macro::TokenStream which
// is, essentially, an ultimate result of procedural macro.
#[macro_use]
extern crate quote; // handy macro to generate rust code

extern crate proc_macro; // builtin
use proc_macro::TokenStream;

// this is a boilerplate func, should be almost the same for
// all proc_macro uses
#[proc_macro_derive(HelloWorld)]
fn hello_world(input: TokenStream) -> TokenStream {
    // convert rust token stream to string (single available API now)
    let s = input.to_string();

    // parse rust code string into abstract syntax tree using syn crate
    let ast = syn::parse_macro_input(&s).unwrap();

    // generate rust code tokens (actual macro works here)
    let gen = impl_hello_world(&ast);

    // convert result to a macro output and return
    gen.parse().unwrap()
}

fn impl_hello_world(ast: &syn::MacroInput) -> quote::Tokens {
    let name = &ast.ident;
    // panic!("idiomatic error reporting way");
    quote! {
        impl #name {
            fn hello_world() {
                println!("Hello, I'm {}", stringify!(#name));
            }
        }
    }
}