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

    