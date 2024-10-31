use crate::dsa::common::dsa_type::DsaType;
use crate::dsa::common::prehash_dsa_type::PrehashDsaType;

/// A trait to get the OID of a DSA
pub trait Oid {
    /// Get the OID for the DSA
    ///
    /// # Returns
    ///
    /// The OID for the DSA as a string
    fn get_oid(&self) -> String;
}

impl Oid for DsaType {
    /// Get the OID for the DSA
    ///
    /// # Returns
    ///
    /// The OID for the DSA
    fn get_oid(&self) -> String {
        match self {
            // RSA PSS with SHA512
            DsaType::Rsa2048PssSHA256 | DsaType::Rsa3072PssSHA512 | DsaType::Rsa4096PssSha512 => {
                "1.2.840.113549.1.1.10"
            }
            // RSA PKCS#1 v1.5 with SHA256
            DsaType::Rsa2048Pkcs15SHA256 => "1.2.840.113549.1.1.11",
            // RSA PKCS#1 v1.5 with SHA512
            DsaType::Rsa3072Pkcs15SHA512 | DsaType::Rsa4096Pkcs15Sha512 => "1.2.840.113549.1.1.13",
            // ECDSA with SHA256
            DsaType::EcdsaBrainpoolP256r1SHA256 | DsaType::EcdsaP256SHA256 => "1.2.840.10045.4.3.2",
            // ECDSA with SHA384
            DsaType::EcdsaP384SHA384 | DsaType::EcdsaBrainpoolP384r1SHA384 => "1.2.840.10045.4.3.3",
            DsaType::Ed25519 => "1.3.101.112",
            DsaType::Ed448 => "1.3.101.113",

            DsaType::SlhDsaSha2_128s => "2.16.840.1.101.3.4.3.20",
            DsaType::SlhDsaSha2_128f => "2.16.840.1.101.3.4.3.21",
            DsaType::SlhDsaSha2_192s => "2.16.840.1.101.3.4.3.22",
            DsaType::SlhDsaSha2_192f => "2.16.840.1.101.3.4.3.23",
            DsaType::SlhDsaSha2_256s => "2.16.840.1.101.3.4.3.24",
            DsaType::SlhDsaSha2_256f => "2.16.840.1.101.3.4.3.25",
            DsaType::SlhDsaShake128s => "2.16.840.1.101.3.4.3.26",
            DsaType::SlhDsaShake128f => "2.16.840.1.101.3.4.3.27",
            DsaType::SlhDsaShake192s => "2.16.840.1.101.3.4.3.28",
            DsaType::SlhDsaShake192f => "2.16.840.1.101.3.4.3.29",
            DsaType::SlhDsaShake256s => "2.16.840.1.101.3.4.3.30",
            DsaType::SlhDsaShake256f => "2.16.840.1.101.3.4.3.31",
        }
        .to_string()
    }
}

impl Oid for PrehashDsaType {
    /// Get the OID for the prehash DSA
    ///
    /// # Returns
    ///
    /// The OID for the prehash DSA
    fn get_oid(&self) -> String {
        match self {
            // TODO: https://github.com/IETF-Hackathon/pqc-certificates/blob/master/docs/oids.json
            PrehashDsaType::MlDsa44 => "2.16.840.1.101.3.4.3.17",
            PrehashDsaType::MlDsa65 => "2.16.840.1.101.3.4.3.18",
            PrehashDsaType::MlDsa87 => "2.16.840.1.101.3.4.3.19",

            // Pure ML-DSA Composite Signature Algorithms
            PrehashDsaType::MlDsa44Rsa2048Pss => "2.16.840.1.114027.80.8.1.21",
            PrehashDsaType::MlDsa44Rsa2048Pkcs15 => "2.16.840.1.114027.80.8.1.22",
            PrehashDsaType::MlDsa44Ed25519 => "2.16.840.1.114027.80.8.1.23",
            PrehashDsaType::MlDsa44EcdsaP256 => "2.16.840.1.114027.80.8.1.24",
            PrehashDsaType::MlDsa65Rsa3072Pss => "2.16.840.1.114027.80.8.1.26",
            PrehashDsaType::MlDsa65Rsa3072Pkcs15 => "2.16.840.1.114027.80.8.1.27",
            PrehashDsaType::MlDsa65Rsa4096Pss => "2.16.840.1.114027.80.8.1.34",
            PrehashDsaType::MlDsa65Rsa4096Pkcs15 => "2.16.840.1.114027.80.8.1.35",
            PrehashDsaType::MlDsa65EcdsaP384 => "2.16.840.1.114027.80.8.1.28",
            PrehashDsaType::MlDsa65EcdsaBrainpoolP256r1 => "2.16.840.1.114027.80.8.1.29",
            PrehashDsaType::MlDsa65Ed25519 => "2.16.840.1.114027.80.8.1.30",
            PrehashDsaType::MlDsa87EcdsaP384 => "2.16.840.1.114027.80.8.1.31",
            PrehashDsaType::MlDsa87EcdsaBrainpoolP384r1 => "2.16.840.1.114027.80.8.1.32",
            PrehashDsaType::MlDsa87Ed448 => "2.16.840.1.114027.80.8.1.33",

            // Hash ML-DSA Composite Signature Algorithms
            PrehashDsaType::MlDsa44Rsa2048PssSha256 => "2.16.840.1.114027.80.8.1.40",
            PrehashDsaType::MlDsa44Rsa2048Pkcs15Sha256 => "2.16.840.1.114027.80.8.1.41",
            PrehashDsaType::MlDsa44Ed25519Sha512 => "2.16.840.1.114027.80.8.1.42",
            PrehashDsaType::MlDsa44EcdsaP256Sha256 => "2.16.840.1.114027.80.8.1.43",
            PrehashDsaType::MlDsa65Rsa3072PssSha512 => "2.16.840.1.114027.80.8.1.44",
            PrehashDsaType::MlDsa65Rsa3072Pkcs15Sha512 => "2.16.840.1.114027.80.8.1.45",
            PrehashDsaType::MlDsa65Rsa4096PssSha512 => "2.16.840.1.114027.80.8.1.46",
            PrehashDsaType::MlDsa65Rsa4096Pkcs15Sha512 => "2.16.840.1.114027.80.8.1.47",
            PrehashDsaType::MlDsa65EcdsaP384Sha512 => "2.16.840.1.114027.80.8.1.48",
            PrehashDsaType::MlDsa65EcdsaBrainpoolP256r1Sha512 => "2.16.840.1.114027.80.8.1.49",
            PrehashDsaType::MlDsa65Ed25519Sha512 => "2.16.840.1.114027.80.8.1.50",
            PrehashDsaType::MlDsa87EcdsaP384Sha512 => "2.16.840.1.114027.80.8.1.51",
            PrehashDsaType::MlDsa87EcdsaBrainpoolP384r1Sha512 => "2.16.840.1.114027.80.8.1.52",
            PrehashDsaType::MlDsa87Ed448Sha512 => "2.16.840.1.114027.80.8.1.53",
        }
        .to_string()
    }
}
