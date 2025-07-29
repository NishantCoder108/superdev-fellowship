use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Error, Fields};

fn todo_app_impl(input: DeriveInput) -> Result<proc_macro2::TokenStream, syn::Error> {
    let input_data = input.clone();
    let name = input_data.ident;
    let attrs = input_data.attrs;
    match input_data.data {
        Data::Struct(todo_struct) => match todo_struct.fields {
            Fields::Named(todo_fields) => {
                let todo_field = todo_fields
                    .named
                    .iter()
                    .map(|todo_field_item| {
                        let field_name = todo_field_item.ident.as_ref().unwrap();
                        let field_ty = &todo_field_item.ty;
                        let pascal_converter = pascal_case(&field_name.to_string());
                        let renamed = format!("TodoApp{}", pascal_converter);

                        quote! {
                            #[serde(rename = #renamed)]
                            #field_name : #field_ty
                        }
                    })
                    .collect::<Vec<_>>();

                Ok(quote! {
                    #(#attrs)*
                    struct #name {
                        #(#todo_field),*
                    }
                })
            }
            _ => Err(Error::new_spanned(
                input,
                "Only struct with name fields are supported",
            )),
        },
        _ => Err(Error::new_spanned(input, "Only structs are supported")),
    }
}

#[proc_macro_attribute]
pub fn todo_app(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match todo_app_impl(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn pascal_case(chars: &String) -> String {
    chars
        .split("_")
        .map(|char_item| {
            let mut word = char_item.chars();
            match word.next() {
                Some(c) => c.to_uppercase().chain(word).collect::<String>(),
                None => String::new(),
            }
        })
        .collect()
}

/**
 * *_____________Prev Another Approach:________________________
 * 
 * #![crate_type = "proc-macro"]
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, FieldsNamed, parse_macro_input};

fn todo_app_impl(input: DeriveInput) -> Result<proc_macro2::TokenStream, syn::Error> {
    let struct_name = &input.ident;
    let struct_vis = &input.vis;
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => fields,
            _ => {
                return Err(syn::Error::new_spanned(
                    struct_name,
                    "Expected a struct with named fields",
                ));
            }
        },
        _ => return Err(syn::Error::new_spanned(struct_name, "Expected a struct")),
    };

    let field_names = fields
        .named
        .iter()
        .map(|field| {
            let field_name = &field.ident.as_ref().unwrap();
            let field_ty = &field.ty;
            let pascal = to_pascal_case(&field_name.to_string());
            let renamed = format!("TodoApp{}", pascal);

            quote! {
                #[serde(rename=#renamed)]
                #field_name: #field_ty
            }
        })
        .collect::<Vec<_>>();

    let token = quote! {
        #[derive(serde::Serialize, serde::Deserialize)]
        #struct_vis struct #struct_name {
            #(#field_names),*
        }


    };

    Ok(token)
}

#[proc_macro_attribute]
pub fn todo_app(_args: TokenStream, _input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(_input as DeriveInput);
    match todo_app_impl(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
                None => "".to_string(),
            }
        })
        .collect()
}
 */