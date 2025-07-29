## Serialize Deserialize using Derive Macros

### What I Learned
- Derive Macros

### What I Explored

- Create `Cargo.toml` file and put inside workspace and members . Define inside related lib like `serialize_macro_traits` and `deserialize_macro_traits`. So basically this lib will be used for serialization and deserialization.


## Step by Step to create custom derive macro for Serialize and Deserialize

1. Create a new cargo workspace (root Cargo.toml)
   ```toml
     [workspace]
     resolver = "3"  

2. Create a new library crate called serialise_macro_traits
   ```bash
   cargo init serialize_macro_traits --lib
   ```
3. Define the traits
   ```rust

    use std::fmt::Error;

    pub trait Serialize {
    	fn serialize(&self) -> Vec<u8>;
    }
    
    pub trait Deserialize: Sized {
    	fn deserialize(base: &[u8]) -> Result<Self, Error>;
    } 
   ```


4. Create a new library crate called serialize_macro
    ```bash
    cargo init serialize_macro --lib
    ```

5. Update Cargo.toml to export a macro
    ```toml
    [lib]
    proc-macro = true
    ```
6. Add syn and quote as dependencies
    ```toml

     [dependencies]
      syn = "2.0"
      quote = "1.0"    
      ```
7. <details>

   <summary>  Define the macro </summary>


      ```rust
      
      use proc_macro::TokenStream;
      use quote::quote;

      #[proc_macro_derive(SerializeNumberStruct)]
      pub fn serialise_number_struct(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate.
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation.
    
     }

     #[proc_macro_derive(DeserializeNumberStruct)]
     pub fn deserialise_number_struct(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate.
    let ast = syn::parse(input).unwrap();
    }

      ```

   </details>
   
8. <details>

    <summary> Implement the macro</summary>

     ```rust
      
         use proc_macro::TokenStream;
          se quote::quote;
          se syn::{DeriveInput, Data, Fields};
          
          [proc_macro_derive(SerializeNumberStruct)]
          ub fn serialise_number_struct(input: TokenStream) -> TokenStream {
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

   [proc_macro_derive(DeserializeNumberStruct)]
   ub fn deserialise_number_struct(input: TokenStream) -> TokenStream {
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
   </details>   


9. For testing the macro, create a new binary
   ```bash
   cargo init --bin app
   
   ```

10. Add dependencies to `serialize_macro` and `serialize_macro_traits`
    ```rust
    
       use std::fmt::Error;
       use serialize_macro::{SerializeNumberStruct, DeserializeNumberStruct};
       use serialize_macro_traits::{Serialize, Deserialize};       

       #[derive(SerializeNumberStruct, DeserializeNumberStruct)]
       struct Swap {
           qty_1: i32,
           qty_2: i32,
           qty_3: i32
       }       
       

       fn main() {
           println!("Hello, world!");
           let s = Swap {
               qty_1: 1,
               qty_2: 2,
               qty_3: 1000
           };
           let bytes = s.serialize();
           println!("{:?}", bytes);
       }

    ```

11. <details> <summary> Question - What is #(#field_serializations)* </summary>

    ## The `#()*` Pattern

    rust
    
    `#(#field_serializations)*`
    
    This is `quote!`'s way of doing iteration/repetition. It's like a "for each" loop but for code     generation.
    
    **What it means:**
    
    - `#(...)` - "repeat this pattern"
    - - "for each item"
    - `#field_serializations` - "insert each item from this iterator"
    
    ## A Simpler Example
    
    Let's say `field_serializations` contains:

    

    ```rust
    vec![
        quote! { result.extend_from_slice(&self.qty_1.to_be_bytes()); },
        quote! { result.extend_from_slice(&self.qty_2.to_be_bytes()); },
    ]
    ```     

    Then `#(#field_serializations)*` generates:      

          

    ```rust
    result.extend_from_slice(&self.qty_1.to_be_bytes());
    result.extend_from_slice(&self.qty_2.to_be_bytes());
    ```      

    ## Alternative Without the Pattern      

    You could write it more verbosely like this:      

    rust      

    ```rust
    let mut tokens = quote! {};
    for field_serialization in field_serializations {
        tokens = quote! {
            #tokens
            #field_serialization
        };
    }
    tokens
    ```      

    But `#(#field_serializations)*` is the idiomatic shorthand.
    </details>