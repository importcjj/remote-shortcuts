use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Visibility, Ident, Type, ExprArray, Token};
use quote::quote;

struct AutoArray {
    visibility: Visibility,
    name: Ident,
    ty: Type,
    elements: ExprArray,
}

impl Parse for AutoArray {
    fn parse(input: ParseStream) -> Result<Self> {
        let visibility: Visibility = input.parse()?;
        input.parse::<Token![const]>()?;
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let ty: Type = input.parse()?;
        input.parse::<Token![=]>()?;
        let elements: ExprArray = input.parse()?;
        input.parse::<Token![;]>()?;

        Ok(AutoArray{
            visibility,
            name,
            ty,
            elements
        })
    }
}

#[proc_macro]
pub fn auto_array(input: TokenStream) -> TokenStream {
    let AutoArray {
        visibility,
        name,
        ty,
        elements
    } = parse_macro_input!(input as AutoArray);

    let length = elements.elems.len();
    let elements = elements.elems.iter().map(|f| quote!{#f});

    let expanded = quote! {
        #visibility const #name: [#ty; #length] = [#(#elements),*];
    };

    TokenStream::from(expanded)
}