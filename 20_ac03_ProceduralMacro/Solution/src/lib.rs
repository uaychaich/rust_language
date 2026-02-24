extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote; 
use syn;

#[proc_macro]
pub fn uay_proc_macro1(_item: TokenStream) -> TokenStream {
    let output = "Hello World";
    let expaned = quote! {
        #output
    };
    TokenStream::from(expaned)
}

#[proc_macro]
pub fn uay_proc_macro2(_item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(_item as syn::LitStr);
    let output = input.value().to_uppercase();
    let expaned = quote! {
        #output
    };
    TokenStream::from(expaned)
}

#[proc_macro_derive(uayprocmacro3)]
pub fn uay_proc_macro3(_item: TokenStream) -> TokenStream {
   let input = syn::parse_macro_input!(_item as syn::DeriveInput);
    let name = input.ident;
    if let syn::Data::Struct(_) = input.data {
    } else {
        return TokenStream::new();
    }
    let expaned = quote! {
        impl #name {
            pub fn showme(&self) -> String {
                format!("This is a function from the uay_proc_macro3 derive macro for struct {}", stringify!(#name))
            }
        }
    };
    TokenStream::from(expaned)
}

#[proc_macro_attribute]
pub fn uay_proc_macro4(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let attrinput = syn::parse_macro_input!(_attr as syn::LitStr).value();
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    let name = input.sig.ident;
    let block = input.block;
    let replaced = quote! {
        fn #name() {
            println!("This is a function from the uay_proc_macro4 attribute macro for function {}", stringify!(#name));
            println!("The attribute value is: {}", #attrinput);
            #block
        }
    };
    TokenStream::from(replaced)
}