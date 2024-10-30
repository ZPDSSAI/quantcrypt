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
            DsaType::Rsa2048Pkcs15SHA256 => None,
            DsaType::Rsa2048PssSHA256 => None,
            DsaType::Rsa3072Pkcs15SHA512 => None,
            DsaType::Rsa3072PssSHA512 => None,

            DsaType::EcdsaP256SHA256 => Some(32),
            DsaType::EcdsaP256SHA512 => Some(32),
            DsaType::EcdsaP384SHA512 => Some(48),
            DsaType::EcdsaBrainpoolP256r1SHA256 => Some(32),
            DsaType::EcdsaBrainpoolP256r1SHA512 => Some(32),
            DsaType::EcdsaBrainpoolP384r1SHA512 => Some(48),
            DsaType::Ed25519SHA512 => Some(32),
            DsaType::Ed448SHA512 => Some(57),

            DsaType::SlhDsaSha2_128s => Some(32 * 2),
            DsaType::SlhDsaSha2_128f => Some(32 * 2),
            DsaType::SlhDsaSha2_192s => Some(48 * 2),
            DsaType::SlhDsaSha2_192f => Some(48 * 2),
            DsaType::SlhDsaSha2_256s => Some(64 * 2),
            DsaType::SlhDsaSha2_256f => Some(64 * 2),
            DsaType::SlhDsaShake128s => Some(32 * 2),
            DsaType::SlhDsaShake128f => Some(32 * 2),
            DsaType::SlhDsaShake192s => Some(48 * 2),
            DsaType::SlhDsaShake192f => Some(48 * 2),
            DsaType::SlhDsaShake256s => Some(64 * 2),
            DsaType::SlhDsaShake256f => Some(64 * 2),
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

            // pq_sk + trad_sk + pq_overhead + trad_overhead + sequence_overhead (to wrap 2 OAKs)
            PrehashDsaType::MlDsa44Rsa2048Pss => None,
            PrehashDsaType::MlDsa44Rsa2048Pkcs15 => None,
            PrehashDsaType::MlDsa44Ed25519 => Some(2560 + 32 + 24 + 14 + 4),
            PrehashDsaType::MlDsa44EcdsaP256 => Some(2560 + 32 + 24 + 19 + 4),
            PrehashDsaType::MlDsa65Rsa3072Pss => None,
            PrehashDsaType::MlDsa65Rsa3072Pkcs15 => None,
            PrehashDsaType::MlDsa65EcdsaP384 => None, //TODO: newly added, check manually 
            PrehashDsaType::MlDsa65EcdsaBrainpoolP256r1 => Some(4032 + 32 + 24 + 19 + 4),
            PrehashDsaType::MlDsa65Ed25519 => Some(4032 + 32 + 24 + 14 + 4),
            PrehashDsaType::MlDsa87EcdsaP384 => Some(4896 + 48 + 24 + 19 + 4),
            PrehashDsaType::MlDsa87EcdsaBrainpoolP384r1 => Some(4896 + 48 + 24 + 19 + 4),
            PrehashDsaType::MlDsa87Ed448 => Some(4896 + 57 + 24 + 14 + 4),
            PrehashDsaType::MlDsa65Rsa4096Pss=> None, //TODO: newly added, check manually 
            PrehashDsaType::MlDsa65Rsa4096Pkcs15 => None, //TODO: newly added, check manually 
        }
    }
}
