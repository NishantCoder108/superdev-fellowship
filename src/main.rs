use std::fmt::Error;

trait Serialize {
    fn serialize(&self) -> Vec<u8>;
}

// trait Deserialize: Sized {
// 	fn deserialize(base: &[u8]) -> Result<Self, Error>;
// }

trait Deserialize: Sized {
    fn deserialize(vec: Vec<u8>) -> Result<Swap, Error>;
}

#[derive(Debug)]
struct Swap {
    qty_1: u32,
    qty_2: u32,
}

impl Serialize for Swap {
    fn serialize(&self) -> Vec<u8> {
        let mut v = vec![];
        let v1 = self.qty_1.to_be_bytes();
        let v2 = self.qty_2.to_be_bytes();

        // v.push(v1);
        // v.push(v2);

        v.extend_from_slice(&v1);
        v.extend_from_slice(&v2);

        v
    }
}

impl Deserialize for Swap {
    fn deserialize(data: Vec<u8>) -> Result<Swap, std::fmt::Error> {
        if data.len() < 8 {
            return Err(std::fmt::Error);
        }

        println!("Vec is : {:?}", data);
        let qty_1 = u32::from_be_bytes(data[0..4].try_into().unwrap());
        let qty_2 = u32::from_be_bytes(data[4..8].try_into().unwrap());

        Ok(Swap { qty_1, qty_2 })
    }
}
fn main() {
    println!("Hello, world!");

    let swap = Swap { qty_1: 3, qty_2: 3 };
    let swap_vec = swap.serialize();

    println!("Swap :{:?}", swap_vec);
    let des = Swap::deserialize(swap_vec);
    println!("Deserialize Swap : {:?}", des);
}
