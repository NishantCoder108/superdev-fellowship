
#### Assignment Link: https://petal-estimate-4e9.notion.site/Assignment-1-Simple-macro-2327dfd1073580008ce2d5dfe3b06dd5

## Week 1 - Learning

1. **proc_macro** - It is the plugin of rust compiler so we write code that write more code. We can write custom derive macro, attribute macro and function macro.
2. **TokenStream** –  It is input/output for interacting with rust compiler. It converts input Rust code into tokens. Basically rust code will convert into TokenStream.
3. **quote!** – It helps to write code inside code . we use it to generate Rust code that will be expanded during compile time. We can inject variables, repeat stuff, and it gives back a TokenStream
4. **&ast.ident** – It will ast tree structure that key name is. for example , struct MyData {}, so MyData is ident.
5. **#(#field_serializations)*** – It is like a loop at compile time.Basically It will insert each item from iterator
6. **generate.into()** – It will converted into TokenStream so compiler can read.
7. **syn::parse(input).unwrap()** – It will parse the input TokenStream into Rust code.It helps  read and understand the code inside a macro. We can access things like: `struct name` , `Fields`, `Attributes`, `Enums, Traits, etc`

### Code :

```rust

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Data, Fields};

#[proc_macro_derive(SerializeNumberStruct)]
pub fn serialise_number_struct(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let serialize_fields = match &ast.data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(fields) => {
                    let field_serializations = fields.named.iter().map(|field| {
                        let field_name = &field.ident;
                        quote! {
                            result.extend_from_slice(&self.#field_name.to_be_bytes());
                        }
                    });
                    /*
                        field_serializeations = [quote!(result.extend_from_slice(&self.qty_1.to_be_bytes())), quote!(result.extend_from_slice(&self.qty_2.to_be_bytes()))]
                     */
                    quote! {
                        #(#field_serializations)*
                    }
                }
                _ => panic!("Only named fields are supported"),
            }
        }
        _ => panic!("Only structs are supported"),
    };
    /*
        serialize_fields ->
        result.extend_from_slice(&self.qty_1.to_be_bytes())
        result.extend_from_slice(&self.qty_2.to_be_bytes())
        result.extend_from_slice(&self.qty_3.to_be_bytes())
     */

    let generated = quote! {
        impl Serialize for #name {
            fn serialize(&self) -> Vec<u8> {
                let mut result = Vec::new();
                #serialize_fields
                result
            }
        }
    };
    generated.into()
}


//Deserialization

#[proc_macro_derive(DeserializeNumberStruct)]
pub fn deserialise_number_struct(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let (deserialize_fields, field_assignments, total_size) = match &ast.data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(fields) => {
                    let mut offset: usize = 0;
                    let mut field_deserializations = Vec::new();
                    let mut field_assignments = Vec::new();
                    
                    for field in &fields.named {
                        let field_name = &field.ident;
                        let field_size = 4;
                        let start_offset = offset;
                        let end_offset = offset + field_size;
                        
                        field_deserializations.push(quote! {
                            let #field_name = {
                                let bytes: [u8; 4] = base[#start_offset..#end_offset]
                                    .try_into()
                                    .map_err(|_| Error)?;
                                i32::from_be_bytes(bytes)
                            };
                        });
                        
                        field_assignments.push(quote! {
                            #field_name
                        });
                        
                        offset += field_size;
                    }
                    
                    (field_deserializations, field_assignments, offset)
                }
                _ => panic!("Only named fields are supported"),
            }
        }
        _ => panic!("Only structs are supported"),
    };

    let generated = quote! {
        impl Deserialize for #name {
            fn deserialize(base: &[u8]) -> Result<Self, Error> {
                if base.len() < #total_size {
                    return Err(Error);
                }
                
                #(#deserialize_fields)*
                
                Ok(#name {
                    #(#field_assignments,)*
                })
            }
        }
    };
    generated.into()
}

```
