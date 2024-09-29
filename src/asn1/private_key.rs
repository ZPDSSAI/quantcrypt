use der::{Decode, Encode};
use pem::EncodeConfig;
use pkcs8::{spki::AlgorithmIdentifier, PrivateKeyInfo};

use crate::asn1::asn_util::{is_composite_oid, is_valid_oid};
use crate::dsa::common::dsa_trait::Dsa;
use crate::dsa::dsa_manager::DsaManager;
use crate::QuantCryptError;
use crate::{asn1::composite_private_key::CompositePrivateKey, errors};

use crate::asn1::asn_util::is_dsa_oid;

type Result<T> = std::result::Result<T, QuantCryptError>;

// Implement clone
#[derive(Clone)]
/// A raw public key for use with the certificate builder
pub struct PrivateKey {
    /// The OID for the DSA / KEM
    oid: String,
    /// The key material
    key: Vec<u8>,
    /// Is it a composite key
    is_composite: bool,
}

impl PrivateKey {
    /// Create a new private key
    ///
    /// # Arguments
    ///
    /// * `oid` - The OID for the DSA / KEM
    /// * `key` - The key material
    ///
    /// # Returns
    ///
    /// A new private key
    ///
    /// # Errors
    ///
    /// `KeyError::InvalidPrivateKey` will be returned if the OID is invalid
    pub fn new(oid: &str, key: &[u8]) -> Result<Self> {
        if !is_valid_oid(&oid.to_string()) {
            return Err(errors::QuantCryptError::InvalidPrivateKey);
        }
        let is_composite = is_composite_oid(oid);
        Ok(Self {
            oid: oid.to_string(),
            key: key.to_vec(),
            is_composite,
        })
    }

    /// Create a new private key from a composite private key
    ///
    /// # Arguments
    ///
    /// * `composite_sk` - The composite private key
    ///
    /// # Returns
    ///
    /// A new private key
    ///
    /// # Errors
    ///
    /// `KeyError::InvalidPrivateKey` will be returned if the private key is invalid
    pub fn from_composite(composite_sk: &CompositePrivateKey) -> Result<Self> {
        Ok(Self {
            oid: composite_sk.get_oid().to_string(),
            key: composite_sk
                .to_der()
                .map_err(|_| errors::QuantCryptError::InvalidPrivateKey)?,
            is_composite: true,
        })
    }

    /// Get the OID for the DSA / KEM
    ///
    /// # Returns
    ///
    /// The OID for the DSA / KEM
    pub fn get_oid(&self) -> &str {
        &self.oid
    }

    /// Get the key material
    ///
    /// # Returns
    ///
    /// The key material
    pub fn get_key(&self) -> &[u8] {
        &self.key
    }

    /// Check if the key is a composite key
    ///
    /// # Returns
    ///
    /// True if the key is a composite key, false otherwise
    pub fn is_composite(&self) -> bool {
        self.is_composite
    }

    /// Get the key material as a DER-encoded byte array
    ///
    /// # Returns
    ///
    /// The DER-encoded byte array
    ///
    /// # Errors
    ///
    /// `KeyError::InvalidPrivateKey` will be returned if the private key is invalid
    pub fn to_der(&self) -> Result<Vec<u8>> {
        let priv_key_info = PrivateKeyInfo {
            algorithm: AlgorithmIdentifier {
                oid: self.oid.parse().unwrap(),
                parameters: None,
            },
            private_key: &self.key,
            public_key: None,
        };
        Ok(priv_key_info
            .to_der()
            .map_err(|_| errors::QuantCryptError::InvalidPrivateKey))?
    }

    /// Get the key material as a PEM-encoded string
    ///
    /// # Returns
    ///
    /// The PEM-encoded string
    ///
    /// # Errors
    ///
    /// `KeyError::InvalidPrivateKey` will be returned if the private key is invalid
    pub fn to_pem(&self) -> Result<String> {
        let der = self
            .to_der()
            .map_err(|_| errors::QuantCryptError::InvalidPrivateKey)?;
        let pem_obj = pem::Pem::new("PRIVATE KEY", der);
        let encode_conf = EncodeConfig::default().set_line_ending(pem::LineEnding::LF);
        Ok(pem::encode_config(&pem_obj, encode_conf))
    }

    /// Create a new private key from a PEM-encoded string
    ///
    /// # Arguments
    ///
    /// * `pem` - The PEM-encoded string
    ///
    /// # Returns
    ///
    /// A new private key
    ///
    /// # Errors
    ///
    /// `KeyError::InvalidPrivateKey` will be returned if the private key is invalid
    pub fn from_pem(pem: &str) -> Result<Self> {
        let pem = pem::parse(pem).map_err(|_| errors::QuantCryptError::InvalidPrivateKey)?;
        // Header should be "PRIVATE KEY"
        if pem.tag() != "PRIVATE KEY" {
            return Err(errors::QuantCryptError::InvalidPrivateKey);
        }

        let der = pem.contents();
        Self::from_der(der)
    }

    /// Create a new private key from a DER-encoded byte array
    ///
    /// # Arguments
    ///
    /// * `der` - The DER-encoded byte array
    ///
    /// # Returns
    ///
    /// A new private key
    ///
    /// # Errors
    ///
    /// `KeyError::InvalidPrivateKey` will be returned if the private key is invalid
    pub fn from_der(der: &[u8]) -> Result<Self> {
        let priv_key_info = PrivateKeyInfo::from_der(der)
            .map_err(|_| errors::QuantCryptError::InvalidPrivateKey)?;

        // Check if the OID is valid
        if !is_valid_oid(&priv_key_info.algorithm.oid.to_string()) {
            return Err(errors::QuantCryptError::InvalidPrivateKey);
        }

        // Check if the OID is a composite key
        let is_composite = is_composite_oid(&priv_key_info.algorithm.oid.to_string());

        Ok(Self {
            oid: priv_key_info.algorithm.oid.to_string(),
            key: priv_key_info.private_key.to_vec(),
            is_composite,
        })
    }

    /// Sign a message
    ///
    /// # Arguments
    ///
    /// * `data` - The data to sign
    ///
    /// # Returns
    ///
    /// The signature
    pub fn sign(&self, data: &[u8]) -> Result<Vec<u8>> {
        // Signing is only possible with DSA keys
        if !is_dsa_oid(&self.oid) {
            return Err(errors::QuantCryptError::UnsupportedOperation);
        }

        let dsa = DsaManager::new_from_oid(&self.oid)?;

        let sig = dsa.sign(&self.key, data)?;

        Ok(sig)
    }
}

#[cfg(test)]
mod test {
    use crate::dsa::common::config::oids::Oid;
    use crate::dsa::common::dsa_type::DsaType;

    use super::*;

    #[test]
    fn test_composite_private_key() {
        let pem_bytes = include_bytes!("../../test/data/mldsa44_ecdsa_p256_sha256_sk.pem");
        let pem = std::str::from_utf8(pem_bytes).unwrap().trim();
        let pk = PrivateKey::from_pem(pem).unwrap();

        assert!(pk.is_composite());
        assert_eq!(pk.get_oid(), DsaType::MlDsa44EcdsaP256SHA256.get_oid());

        let key_bytes = pk.get_key();
        let pk2 = CompositePrivateKey::from_der(&pk.oid, &key_bytes).unwrap();

        assert_eq!(pk.oid, pk2.get_oid());

        let pk2 = PrivateKey::from_composite(&pk2).unwrap();
        let pem2 = pk2.to_pem().unwrap();
        assert_eq!(pem, pem2.trim());

        let oid = DsaType::MlDsa44EcdsaP256SHA256.get_oid();
        assert_eq!(pk.oid, oid);
    }

    #[test]
    fn test_pk_no_headers() {
        let pem_bytes = include_bytes!("../../test/data/bad/no_headers.pem");
        let pem = std::str::from_utf8(pem_bytes).unwrap().trim();
        let pk = PrivateKey::from_pem(pem);

        assert!(pk.is_err());
        assert!(matches!(
            pk.err().unwrap(),
            errors::QuantCryptError::InvalidPrivateKey
        ));
    }

    #[test]
    fn test_pk_bad_base64() {
        let pem_bytes = include_bytes!("../../test/data/bad/bad_base64.pem");
        let pem = std::str::from_utf8(pem_bytes).unwrap().trim();
        let pk = PrivateKey::from_pem(pem);

        assert!(pk.is_err());
        assert!(matches!(
            pk.err().unwrap(),
            errors::QuantCryptError::InvalidPrivateKey
        ));
    }

    #[test]
    fn test_pk_empty() {
        let pem_bytes = include_bytes!("../../test/data/bad/empty.pem");
        let pem = std::str::from_utf8(pem_bytes).unwrap().trim();
        let pk = PrivateKey::from_pem(pem);

        assert!(pk.is_err());
        assert!(matches!(
            pk.err().unwrap(),
            errors::QuantCryptError::InvalidPrivateKey
        ));
    }

    #[test]
    fn test_pk_bad_tag() {
        let pem_bytes = include_bytes!("../../test/data/bad/bad_tag.pem");
        let pem = std::str::from_utf8(pem_bytes).unwrap().trim();
        let pk = PrivateKey::from_pem(pem);

        assert!(pk.is_err());
        assert!(matches!(
            pk.err().unwrap(),
            errors::QuantCryptError::InvalidPrivateKey
        ));
    }

    #[test]
    fn test_pk_bad_algorithm() {
        let pem_bytes = include_bytes!("../../test/data/bad/private_rsa_2048.pem");
        let pem = std::str::from_utf8(pem_bytes).unwrap().trim();
        let pk = PrivateKey::from_pem(pem);

        assert!(pk.is_err());
        assert!(matches!(
            pk.err().unwrap(),
            errors::QuantCryptError::InvalidPrivateKey
        ));
    }

    #[test]
    fn test_sk_serialization_deserialization() {
        let pem_bytes = include_bytes!("../../test/data/mldsa44_ecdsa_p256_sha256_sk.pem");
        let pem = std::str::from_utf8(pem_bytes).unwrap().trim();
        let pk = PrivateKey::from_pem(pem).unwrap();

        let der = pk.to_der().unwrap();
        let pk2 = PrivateKey::from_der(&der).unwrap();
        let pem2 = pk2.to_pem().unwrap();
        assert_eq!(pem.trim(), pem2.trim());

        let der2 = pk2.to_der().unwrap();
        assert_eq!(der, der2);
    }
}
