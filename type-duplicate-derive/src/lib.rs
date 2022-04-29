extern crate proc_macro;

use proc_macro2::{TokenStream, Ident, Span};
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Index,};

const SUFFIX: &str = "Bson";

#[proc_macro_derive(Duplicate)]
pub fn duplicate_type_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    let result = impl_duplicate_type_macro(&input);
    proc_macro::TokenStream::from(result)
}

fn fields(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    // Expands to an expression like
                    //
                    //     0 + self.x.heap_size() + self.y.heap_size() + self.z.heap_size()
                    //
                    // but using fully qualified function call syntax.
                    //
                    // We take some care to use the span of each `syn::Field` as
                    // the span of the corresponding `heap_size_of_children`
                    // call. This way if one of the field types does not
                    // implement `HeapSize` then the compiler's error message
                    // underlines which field it is. An example is shown in the
                    // readme of the parent directory.
                    let recurse = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        quote_spanned! {f.span()=>
                            heapsize::HeapSize::heap_size_of_children(&self.#name)
                        }
                    });
                    quote! {
                        0 #(+ #recurse)*
                    }
                }
                Fields::Unnamed(ref fields) => {
                    // Expands to an expression like
                    //
                    //     0 + self.0.heap_size() + self.1.heap_size() + self.2.heap_size()
                    let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                        let index = Index::from(i);
                        quote_spanned! {f.span()=>
                            heapsize::HeapSize::heap_size_of_children(&self.#index)
                        }
                    });
                    quote! {
                        0 #(+ #recurse)*
                    }
                }
                Fields::Unit => {
                    // Unit structs cannot own more than 0 bytes of heap memory.
                    quote!(0)
                }
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

fn impl_duplicate_type_macro(input: &syn::DeriveInput) -> TokenStream {

    let name = &input.ident;

    let dup_ident = Ident::new(&format!("{}{}", name, SUFFIX), Span::call_site());

    let gen = quote! {
        #[derive(Debug, serde::Serialize, serde::Deserialize)]
        pub struct #dup_ident {
            a: u32,
            b: u32,
        }

        impl duplicate::DuplicateMarker for #name {};
    };

    gen.into()
}
