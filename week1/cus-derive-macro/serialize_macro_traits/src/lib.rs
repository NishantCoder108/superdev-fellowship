use std::fmt::Error;

pub trait Serialize {
    fn serialize(&self) -> Vec<u8>; //0,1....
}

pub trait Deserialize: Sized {
    fn deserialize(base: &[u8]) -> Result<Self, Error>; // back to struct
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
