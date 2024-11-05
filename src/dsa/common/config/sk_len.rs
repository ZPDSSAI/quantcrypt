use crate::dsa::common::dsa_type::DsaType;
use crate::dsa::common::prehash_dsa_type::PrehashDsaType;
/// A trait to get the length of the public key

pub trait SKLen {
    fn get_sk_len(&self) -> Option<usize>;
}

impl SKLen for DsaType {
    /// Get the length of the private key
    ///
    /// # Returns
    ///
    /// The length of the private key in bytes or `None` if the length is not fixed
    fn get_sk_len(&self) -> Option<usize> {
        match self {
            // RSAs do not have a fixed sk length
            DsaType::Rsa2048Pkcs15Sha256 => None,
            DsaType::Rsa2048PssSha256 => None,
            DsaType::Rsa3072Pkcs15Sha256 => None,
            DsaType::Rsa3072PssSha256 => None,
            DsaType::Rsa4096Pkcs15Sha384 => None,
            DsaType::Rsa4096PssSha384 => None,

            DsaType::EcdsaP256SHA256 => Some(32),
            DsaType::EcdsaBrainpoolP256r1SHA256 => Some(32),
            DsaType::EcdsaP384SHA384 => Some(48),
            DsaType::EcdsaBrainpoolP384r1SHA384 => Some(48),

            DsaType::Ed25519 => Some(32),
            DsaType::Ed448 => Some(57),
        }
    }
}

impl SKLen for PrehashDsaType {
    /// Get the length of the private key
    ///
    /// # Returns
    ///
    /// The length of the private key in bytes or `None` if the length is not fixed
    fn get_sk_len(&self) -> Option<usize> {
        match self {
            PrehashDsaType::MlDsa44 => Some(2560),
            PrehashDsaType::MlDsa65 => Some(4032),
            PrehashDsaType::MlDsa87 => Some(4896),

            PrehashDsaType::HashMlDsa44 => Some(2560),
            PrehashDsaType::HashMlDsa65 => Some(4032),
            PrehashDsaType::HashMlDsa87 => Some(4896),

            // pq_sk + trad_sk + overhead of sequence of two octet strings
            PrehashDsaType::MlDsa44Rsa2048Pss => None, // None
            PrehashDsaType::MlDsa44Rsa2048Pkcs15 => None, // None
            PrehashDsaType::MlDsa44Ed25519 => Some(2560 + 32 + 10), // 2602
            PrehashDsaType::MlDsa44EcdsaP256 => Some(2560 + 32 + 10), // 2602
            PrehashDsaType::MlDsa65Rsa3072Pss => None, // None
            PrehashDsaType::MlDsa65Rsa3072Pkcs15 => None, // None
            PrehashDsaType::MlDsa65Rsa4096Pss => None, // None
            PrehashDsaType::MlDsa65Rsa4096Pkcs15 => None, // None
            PrehashDsaType::MlDsa65EcdsaP384 => Some(4032 + 48 + 10), // 4090
            PrehashDsaType::MlDsa65EcdsaBrainpoolP256r1 => Some(4032 + 32 + 10), // 4074
            PrehashDsaType::MlDsa65Ed25519 => Some(4032 + 32 + 10), // 4074
            PrehashDsaType::MlDsa87EcdsaP384 => Some(4896 + 48 + 10), // 4954
            PrehashDsaType::MlDsa87EcdsaBrainpoolP384r1 => Some(4896 + 48 + 10), // 4954
            PrehashDsaType::MlDsa87Ed448 => Some(4896 + 57 + 10), // 4963

            PrehashDsaType::HashMlDsa44Rsa2048PssSha256 => None, // None
            PrehashDsaType::HashMlDsa44Rsa2048Pkcs15Sha256 => None, // None
            PrehashDsaType::HashMlDsa44Ed25519Sha512 => Some(2560 + 32 + 10), // 2602
            PrehashDsaType::HashMlDsa44EcdsaP256Sha256 => Some(2560 + 32 + 10), // 2602
            PrehashDsaType::HashMlDsa65Rsa3072PssSha512 => None, // None
            PrehashDsaType::HashMlDsa65Rsa3072Pkcs15Sha512 => None, // None
            PrehashDsaType::HashMlDsa65Rsa4096PssSha512 => None, // None
            PrehashDsaType::HashMlDsa65Rsa4096Pkcs15Sha512 => None, // None
            PrehashDsaType::HashMlDsa65EcdsaP384Sha512 => Some(4032 + 48 + 10), // 4090
            PrehashDsaType::HashMlDsa65EcdsaBrainpoolP256r1Sha512 => Some(4032 + 32 + 10), // 4074
            PrehashDsaType::HashMlDsa65Ed25519Sha512 => Some(4032 + 32 + 10), // 4074
            PrehashDsaType::HashMlDsa87EcdsaP384Sha512 => Some(4896 + 48 + 10), // 4954
            PrehashDsaType::HashMlDsa87EcdsaBrainpoolP384r1Sha512 => Some(4896 + 48 + 10), // 4954
            PrehashDsaType::HashMlDsa87Ed448Sha512 => Some(4896 + 57 + 10), // 4963

            // Pure SLH-DSA, aligned with fips205 implementation
            PrehashDsaType::SlhDsaSha2_128s => Some(32 * 2),
            PrehashDsaType::SlhDsaSha2_128f => Some(32 * 2),
            PrehashDsaType::SlhDsaSha2_192s => Some(48 * 2),
            PrehashDsaType::SlhDsaSha2_192f => Some(48 * 2),
            PrehashDsaType::SlhDsaSha2_256s => Some(64 * 2),
            PrehashDsaType::SlhDsaSha2_256f => Some(64 * 2),
            PrehashDsaType::SlhDsaShake128s => Some(32 * 2),
            PrehashDsaType::SlhDsaShake128f => Some(32 * 2),
            PrehashDsaType::SlhDsaShake192s => Some(48 * 2),
            PrehashDsaType::SlhDsaShake192f => Some(48 * 2),
            PrehashDsaType::SlhDsaShake256s => Some(64 * 2),
            PrehashDsaType::SlhDsaShake256f => Some(64 * 2),

            // Prehash SLH-DSA
            PrehashDsaType::HashSlhDsaSha2_128s => Some(32 * 2),
            PrehashDsaType::HashSlhDsaSha2_128f => Some(32 * 2),
            PrehashDsaType::HashSlhDsaSha2_192s => Some(48 * 2),
            PrehashDsaType::HashSlhDsaSha2_192f => Some(48 * 2),
            PrehashDsaType::HashSlhDsaSha2_256s => Some(64 * 2),
            PrehashDsaType::HashSlhDsaSha2_256f => Some(64 * 2),
            PrehashDsaType::HashSlhDsaShake128s => Some(32 * 2),
            PrehashDsaType::HashSlhDsaShake128f => Some(32 * 2),
            PrehashDsaType::HashSlhDsaShake192s => Some(48 * 2),
            PrehashDsaType::HashSlhDsaShake192f => Some(48 * 2),
            PrehashDsaType::HashSlhDsaShake256s => Some(64 * 2),
            PrehashDsaType::HashSlhDsaShake256f => Some(64 * 2),
        }
    }
}
