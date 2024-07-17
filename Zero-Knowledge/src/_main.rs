/// The code demonstrates serialization and deserialization of a custom struct containing public and
/// secret keys using the serde library in Rust.
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use secp256k1::rand::rngs::OsRng;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct MyStruct {
    #[serde(with = "PublicKeyDef")]
    pub_key: PublicKey,
    #[serde(with = "SecretKeyDef")]
    sec_key: SecretKey,
}

mod PublicKeyDef {
    use super::*;
    use serde::{self, Serializer, Deserializer};

    pub fn serialize<S>(key: &PublicKey, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&key.serialize())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<PublicKey, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes: Vec<u8> = serde::Deserialize::deserialize(deserializer)?;
        PublicKey::from_slice(&bytes).map_err(serde::de::Error::custom)
    }
}

mod SecretKeyDef {
    use super::*;
    use serde::{self, Serializer, Deserializer};

    pub fn serialize<S>(key: &SecretKey, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&key[..])
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<SecretKey, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes: Vec<u8> = serde::Deserialize::deserialize(deserializer)?;
        SecretKey::from_slice(&bytes).map_err(serde::de::Error::custom)
    }
}

fn main() {
    let secp = Secp256k1::new();
    let mut rng = OsRng;

    let (secret_key, public_key) = secp.generate_keypair(&mut rng);

    let my_struct = MyStruct {
        pub_key: public_key,
        sec_key: secret_key,
    };

    let serialized = serde_json::to_string(&my_struct).unwrap();
    println!("Serialized: {}", serialized);

    let deserialized: MyStruct = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialization_deserialization() {
        let secp = Secp256k1::new();
        let mut rng = OsRng;

        let (secret_key, public_key) = secp.generate_keypair(&mut rng);

        let my_struct = MyStruct {
            pub_key: public_key,
            sec_key: secret_key,
        };

        // Serialize the structure
        let serialized = serde_json::to_string(&my_struct).unwrap();
        println!("Serialized: {}", serialized);

        // Deserialize the structure
        let deserialized: MyStruct = serde_json::from_str(&serialized).unwrap();

        // Ensure the original and deserialized structures are the same
        assert_eq!(my_struct, deserialized);
    }
}
