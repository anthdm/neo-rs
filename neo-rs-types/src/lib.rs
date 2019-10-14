extern crate hex;

pub enum Error {
    InvalidSliceLength,
    InvalidHexString,
}

macro_rules! make_hash_type {
    ($hash_type:ident, $hash_size:expr) => {
        pub struct $hash_type(pub [u8; $hash_size]);

        impl $hash_type {
            pub fn to_hex(&self) -> String {
                hex::encode(self.0)
            }

            pub fn from_slice(src: &[u8]) -> Result<Self, Error> {
                if src.len() != $hash_size {
                    return Err(Error::InvalidSliceLength);
                }

                let mut bytes = [0u8; $hash_size];
                bytes.copy_from_slice(src);
                Ok($hash_type(bytes))
            }

            pub fn zero() -> Self {
                let bytes = [0u8; $hash_size];

                $hash_type::from_slice(bytes.as_ref()).expect("failed to create from slice")
            }
        }
    };
}

// Holds the 32 byte long hash of arbitrary data.
make_hash_type!(H256, 32);

// Holds the 20 byte long hash of arbitrary data.
make_hash_type!(H160, 20);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn something() {
        assert_eq!(true, true);
    }
}
