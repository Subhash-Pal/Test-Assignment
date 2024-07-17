use secp256k1::{Secp256k1, SecretKey, PublicKey};
use secp256k1::rand::rngs::OsRng;
use serde::{Serialize, Deserialize};

/// `MyStruct` is a structure that holds a public and a secret key.
/// The keys are serialized and deserialized using custom modules.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct MyStruct {
    #[serde(with = "PublicKeyDef")]
    pub_key: PublicKey,
    #[serde(with = "SecretKeyDef")]
    sec_key: SecretKey,
}

/// Module for serializing and deserializing `PublicKey`.
mod PublicKeyDef {
    use super::*;
    use serde::{self, Serializer, Deserializer};

    /// Serializes a `PublicKey` into bytes.
    pub fn serialize<S>(key: &PublicKey, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&key.serialize())
    }

    /// Deserializes bytes into a `PublicKey`.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<PublicKey, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes: Vec<u8> = serde::Deserialize::deserialize(deserializer)?;
        PublicKey::from_slice(&bytes).map_err(serde::de::Error::custom)
    }
}

/// Module for serializing and deserializing `SecretKey`.
mod SecretKeyDef {
    use super::*;
    use serde::{self, Serializer, Deserializer};

    /// Serializes a `SecretKey` into bytes.
    pub fn serialize<S>(key: &SecretKey, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&key[..])
    }

    /// Deserializes bytes into a `SecretKey`.
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

    // Generate a keypair (public and secret keys).
    let (secret_key, public_key) = secp.generate_keypair(&mut rng);

    let my_struct = MyStruct {
        pub_key: public_key,
        sec_key: secret_key,
    };

    // Serialize `MyStruct` to a JSON string.
    let serialized = serde_json::to_string(&my_struct).unwrap();
    println!("Serialized: {}", serialized);

    // Deserialize the JSON string back to `MyStruct`.
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

        // Generate a keypair (public and secret keys) for testing.
        let (secret_key, public_key) = secp.generate_keypair(&mut rng);

        let my_struct = MyStruct {
            pub_key: public_key,
            sec_key: secret_key,
        };

        // Serialize the structure to a JSON string.
        let serialized = serde_json::to_string(&my_struct).unwrap();
        println!("Serialized: {}", serialized);

        // Deserialize the JSON string back to the structure.
        let deserialized: MyStruct = serde_json::from_str(&serialized).unwrap();

        // Ensure the original and deserialized structures are the same.
        assert_eq!(my_struct, deserialized);
    }
}
