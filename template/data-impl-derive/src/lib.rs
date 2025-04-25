use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput, Ident, GenericParam, TypeParam};

#[proc_macro_attribute]
pub fn data_functions(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(item as DeriveInput);
    
    // Get the name of the struct
    let name = &input.ident;
    
    // Get the generics from the struct
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    
    // Create a new where clause that includes T: std::fmt::Debug
    let mut where_clause = where_clause.cloned().unwrap_or_else(|| syn::WhereClause {
        where_token: syn::token::Where::default(),
        predicates: syn::punctuated::Punctuated::new(),
    });
    
    // Add T: std::fmt::Debug for each type parameter
    for param in &generics.params {
        if let GenericParam::Type(TypeParam { ident, .. }) = param {
            let predicate: syn::WherePredicate = syn::parse_quote! {
                #ident: std::fmt::Debug
            };
            where_clause.predicates.push(predicate);
        }
    }
    
    // Generate the implementation
    let expanded = quote! {
        // Keep the original struct
        #input
        
        // Implement the methods
        impl #impl_generics #name #ty_generics #where_clause {
            pub fn print_data(&self) {
                println!("{:?}", self);
            }
            
            pub fn modify_data(&mut self) {
                self.increment_version();
            }
        }
    };
    
    // Return the generated code
    TokenStream::from(expanded)
}
