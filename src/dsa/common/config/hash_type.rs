use crate::dsa::common::prehash_dsa_type::PrehashDsaType;
use crate::hash::common::hash_type::HashType;

/// A trait to get the HashType for a DSA
pub trait HashTypeConfig {
    /// Get the hash type for the DSA
    ///
    /// # Returns
    ///
    /// The hash type for the DSA or None if no hash is used
    fn get_hash_type(&self) -> Option<HashType>;
}

impl HashTypeConfig for PrehashDsaType {
    /// Get the hash type for the DSA
    ///
    /// # Returns
    ///
    /// The hash thype for the DSA or None if no hash is used
    fn get_hash_type(&self) -> Option<HashType> {
        match self {
            // ML DSA
            PrehashDsaType::MlDsa44 => None,
            PrehashDsaType::MlDsa65 => None,
            PrehashDsaType::MlDsa87 => None,

            PrehashDsaType::HashMlDsa44 => Some(HashType::Sha512),
            PrehashDsaType::HashMlDsa65 => Some(HashType::Sha512),
            PrehashDsaType::HashMlDsa87 => Some(HashType::Sha512),

            // Pure ML-DSA Composite Signature Algorithms
            PrehashDsaType::MlDsa44Rsa2048Pss => None,
            PrehashDsaType::MlDsa44Rsa2048Pkcs15 => None,
            PrehashDsaType::MlDsa44Ed25519 => None,
            PrehashDsaType::MlDsa44EcdsaP256 => None,
            PrehashDsaType::MlDsa65Rsa3072Pss => None,
            PrehashDsaType::MlDsa65Rsa3072Pkcs15 => None,
            PrehashDsaType::MlDsa65Rsa4096Pss => None,
            PrehashDsaType::MlDsa65Rsa4096Pkcs15 => None,
            PrehashDsaType::MlDsa65EcdsaP384 => None,
            PrehashDsaType::MlDsa65EcdsaBrainpoolP256r1 => None,
            PrehashDsaType::MlDsa65Ed25519 => None,
            PrehashDsaType::MlDsa87EcdsaP384 => None,
            PrehashDsaType::MlDsa87EcdsaBrainpoolP384r1 => None,
            PrehashDsaType::MlDsa87Ed448 => None,

            // Hash ML-DSA Composite Signature Algorithms
            PrehashDsaType::HashMlDsa44Rsa2048PssSha256 => Some(HashType::Sha256),
            PrehashDsaType::HashMlDsa44Rsa2048Pkcs15Sha256 => Some(HashType::Sha256),
            PrehashDsaType::HashMlDsa44Ed25519Sha512 => Some(HashType::Sha512),
            PrehashDsaType::HashMlDsa44EcdsaP256Sha256 => Some(HashType::Sha256),
            PrehashDsaType::HashMlDsa65Rsa3072PssSha512 => Some(HashType::Sha512),
            PrehashDsaType::HashMlDsa65Rsa3072Pkcs15Sha512 => Some(HashType::Sha512),
            PrehashDsaType::HashMlDsa65Rsa4096PssSha512 => Some(HashType::Sha512),
            PrehashDsaType::HashMlDsa65Rsa4096Pkcs15Sha512 => Some(HashType::Sha512),
            PrehashDsaType::HashMlDsa65EcdsaP384Sha512 => Some(HashType::Sha512),
            PrehashDsaType::HashMlDsa65EcdsaBrainpoolP256r1Sha512 => Some(HashType::Sha512),
            PrehashDsaType::HashMlDsa65Ed25519Sha512 => Some(HashType::Sha512),
            PrehashDsaType::HashMlDsa87EcdsaP384Sha512 => Some(HashType::Sha512),
            PrehashDsaType::HashMlDsa87EcdsaBrainpoolP384r1Sha512 => Some(HashType::Sha512),
            PrehashDsaType::HashMlDsa87Ed448Sha512 => Some(HashType::Sha512),

            PrehashDsaType::SlhDsaSha2_128s => None,
            PrehashDsaType::SlhDsaSha2_128f => None,
            PrehashDsaType::SlhDsaSha2_192s => None,
            PrehashDsaType::SlhDsaSha2_192f => None,
            PrehashDsaType::SlhDsaSha2_256s => None,
            PrehashDsaType::SlhDsaSha2_256f => None,
            PrehashDsaType::SlhDsaShake128s => None,
            PrehashDsaType::SlhDsaShake128f => None,
            PrehashDsaType::SlhDsaShake192s => None,
            PrehashDsaType::SlhDsaShake192f => None,
            PrehashDsaType::SlhDsaShake256s => None,
            PrehashDsaType::SlhDsaShake256f => None,

            // Prehash SLH-DSA
            PrehashDsaType::HashSlhDsaSha2_128s => Some(HashType::Sha256),
            PrehashDsaType::HashSlhDsaSha2_128f => Some(HashType::Sha256),
            PrehashDsaType::HashSlhDsaSha2_192s => Some(HashType::Sha512),
            PrehashDsaType::HashSlhDsaSha2_192f => Some(HashType::Sha512),
            PrehashDsaType::HashSlhDsaSha2_256s => Some(HashType::Sha512),
            PrehashDsaType::HashSlhDsaSha2_256f => Some(HashType::Sha512),
            PrehashDsaType::HashSlhDsaShake128s => Some(HashType::Shake128),
            PrehashDsaType::HashSlhDsaShake128f => Some(HashType::Shake128),
            PrehashDsaType::HashSlhDsaShake192s => Some(HashType::Shake256),
            PrehashDsaType::HashSlhDsaShake192f => Some(HashType::Shake256),
            PrehashDsaType::HashSlhDsaShake256s => Some(HashType::Shake256),
            PrehashDsaType::HashSlhDsaShake256f => Some(HashType::Shake256),
        }
    }
}
