use openssl::hash::MessageDigest;
use openssl::rsa::Padding;
use openssl::sign::RsaPssSaltlen;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use rsa::pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey};
use rsa::RsaPrivateKey;

use crate::dsa::common::dsa_info::DsaInfo;
use crate::dsa::common::dsa_trait::Dsa;
use crate::dsa::common::dsa_type::DsaType;
use crate::QuantCryptError;

type Result<T> = std::result::Result<T, QuantCryptError>;

#[derive(Clone)]
pub struct RsaDsaManager {
    pub dsa_info: DsaInfo,
}

impl RsaDsaManager {
    fn get_hash_and_padding(&self) -> (MessageDigest, Padding) {
        match self.dsa_info.dsa_type {
            DsaType::Rsa2048Pkcs15Sha256 => (
                openssl::hash::MessageDigest::sha256(),
                openssl::rsa::Padding::PKCS1,
            ),
            DsaType::Rsa2048PssSha256 => (
                openssl::hash::MessageDigest::sha256(),
                openssl::rsa::Padding::PKCS1_PSS,
            ),
            DsaType::Rsa3072Pkcs15Sha256 => (
                openssl::hash::MessageDigest::sha256(),
                openssl::rsa::Padding::PKCS1,
            ),
            DsaType::Rsa3072PssSha256 => (
                openssl::hash::MessageDigest::sha256(),
                openssl::rsa::Padding::PKCS1_PSS,
            ),
            DsaType::Rsa4096Pkcs15Sha384 => (
                openssl::hash::MessageDigest::sha384(),
                openssl::rsa::Padding::PKCS1,
            ),
            DsaType::Rsa4096PssSha384 => (
                openssl::hash::MessageDigest::sha384(),
                openssl::rsa::Padding::PKCS1_PSS,
            ),
            _ => {
                panic!("Not implemented");
            }
        }
    }
}

impl Dsa for RsaDsaManager {
    /// Create a new DSA instance
    ///
    /// # Arguments
    ///
    /// * `dsa_type` - The type of DSA to create
    ///
    /// # Returns
    ///
    /// A new DSA instance
    fn new(dsa_type: DsaType) -> Result<Self>
    where
        Self: Sized,
    {
        let dsa_info = DsaInfo::new(dsa_type);
        Ok(RsaDsaManager { dsa_info })
    }

    /// Generate a keypair using the default RNG (ChaCha20)
    ///
    /// # Returns
    ///
    /// A tuple containing the public and secret keys (pk, sk)
    fn key_gen(&mut self) -> Result<(Vec<u8>, Vec<u8>)> {
        let mut rng = ChaCha20Rng::from_entropy();
        self.key_gen_with_rng(&mut rng)
    }

    /// Generate a keypair using the specified RNG
    ///
    /// # Arguments
    ///
    /// * `rng` - A random number generator
    ///
    /// # Returns
    ///
    /// A tuple containing the public and secret keys (pk, sk)
    fn key_gen_with_rng(
        &mut self,
        rng: &mut impl rand_core::CryptoRngCore,
    ) -> Result<(Vec<u8>, Vec<u8>)> {
        let bits = match self.dsa_info.dsa_type {
            DsaType::Rsa2048Pkcs15Sha256 => 2048,
            DsaType::Rsa2048PssSha256 => 2048,
            DsaType::Rsa3072Pkcs15Sha256 => 3072,
            DsaType::Rsa3072PssSha256 => 3072,
            DsaType::Rsa4096Pkcs15Sha384 => 4096,
            DsaType::Rsa4096PssSha384 => 4096,
            _ => {
                return Err(QuantCryptError::NotImplemented);
            }
        };

        // Use the RSA crate as we can specify the rng
        let rpk =
            RsaPrivateKey::new(rng, bits).map_err(|_| QuantCryptError::KeyPairGenerationFailed)?;

        let sd = rpk
            .to_pkcs1_der()
            .map_err(|_| QuantCryptError::KeyPairGenerationFailed)?;
        let sk = sd.to_bytes().to_vec();

        // PKCS1 DER format
        let pd = rpk
            .to_public_key()
            .to_pkcs1_der()
            .map_err(|_| QuantCryptError::KeyPairGenerationFailed)?;
        let pk = pd.to_vec();

        Ok((pk, sk))
    }

    /// Sign a message
    ///
    /// # Arguments
    ///
    /// * `sk` - The secret key to sign the message
    /// * `msg` - The message to sign
    ///
    /// # Returns
    ///
    /// The signature of the message
    fn sign(&self, sk: &[u8], msg: &[u8]) -> Result<Vec<u8>> {
        let rsa_sk = openssl::rsa::Rsa::private_key_from_der(sk)
            .map_err(|_| QuantCryptError::SerializationFailed)?;
        let pkey =
            openssl::pkey::PKey::from_rsa(rsa_sk).map_err(|_| QuantCryptError::SignatureFailed)?;

        let (hash, padding) = self.get_hash_and_padding();

        // Createa a signer
        let mut signer = openssl::sign::Signer::new(hash, &pkey)
            .map_err(|_| QuantCryptError::SignatureFailed)?;
        signer
            .set_rsa_padding(padding)
            .map_err(|_| QuantCryptError::SignatureFailed)?;

        if padding == openssl::rsa::Padding::PKCS1_PSS {
            signer
                .set_rsa_mgf1_md(hash)
                .map_err(|_| QuantCryptError::SignatureFailed)?;
            signer
                .set_rsa_pss_saltlen(RsaPssSaltlen::DIGEST_LENGTH)
                .map_err(|_| QuantCryptError::SignatureFailed)?;
        }

        // Sign the message
        signer
            .update(msg)
            .map_err(|_| QuantCryptError::SignatureFailed)?;

        let signature = signer
            .sign_to_vec()
            .map_err(|_| QuantCryptError::SignatureFailed)?;
        Ok(signature)
    }

    /// Verify a signature
    ///
    /// # Arguments
    ///
    /// * `pk` - The public key to verify the signature
    /// * `msg` - The message to verify
    /// * `signature` - The signature to verify
    ///
    /// # Returns
    ///
    /// A boolean indicating if the signature is valid
    fn verify(&self, pk: &[u8], msg: &[u8], signature: &[u8]) -> Result<bool> {
        let rsa_pk = openssl::rsa::Rsa::public_key_from_der_pkcs1(pk)
            .map_err(|_| QuantCryptError::SerializationFailed)?;
        let pkey = openssl::pkey::PKey::from_rsa(rsa_pk)
            .map_err(|_| QuantCryptError::SignatureVerificationFailed)?;

        let (hash, padding) = self.get_hash_and_padding();

        // Create a verifier
        let mut verifier = openssl::sign::Verifier::new(hash, &pkey)
            .map_err(|_| QuantCryptError::SignatureVerificationFailed)?;
        verifier
            .set_rsa_padding(padding)
            .map_err(|_| QuantCryptError::SignatureVerificationFailed)?;

        if padding == openssl::rsa::Padding::PKCS1_PSS {
            verifier
                .set_rsa_mgf1_md(hash)
                .map_err(|_| QuantCryptError::SignatureVerificationFailed)?;
            verifier
                .set_rsa_pss_saltlen(RsaPssSaltlen::DIGEST_LENGTH)
                .map_err(|_| QuantCryptError::SignatureVerificationFailed)?;
        }

        // Verify the signature
        verifier
            .update(msg)
            .map_err(|_| QuantCryptError::SignatureVerificationFailed)?;
        let result = verifier
            .verify(signature)
            .map_err(|_| QuantCryptError::SignatureVerificationFailed)?;
        Ok(result)
    }

    /// Get DSA metadata information such as the key lengths,
    /// size of signature, etc.
    ///
    /// These values are also used to test the correctness of the DSA
    ///
    /// # Returns
    ///
    /// A structure containing metadata about the DSA
    fn get_dsa_info(&self) -> DsaInfo {
        self.dsa_info.clone()
    }

    fn get_public_key(&self, sk: &[u8]) -> Result<Vec<u8>> {
        let rsa_sk = openssl::rsa::Rsa::private_key_from_der(sk)
            .map_err(|_| QuantCryptError::InvalidPrivateKey)?;
        rsa_sk
            .public_key_to_der_pkcs1()
            .map_err(|_| QuantCryptError::SerializationFailed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsa::common::dsa_type::DsaType;
    use crate::dsa::common::macros::test_dsa;

    #[test]
    fn test_rsa_2048_pkcs15_sha256() {
        let rsa_dsa_manager = RsaDsaManager::new(DsaType::Rsa2048Pkcs15Sha256);
        test_dsa!(rsa_dsa_manager);
    }

    #[test]
    fn test_rsa_2048_pss_sha256() {
        let rsa_dsa_manager = RsaDsaManager::new(DsaType::Rsa2048PssSha256);
        test_dsa!(rsa_dsa_manager);
    }

    #[test]
    fn test_rsa_3072_pkcs15_sha256() {
        let rsa_dsa_manager = RsaDsaManager::new(DsaType::Rsa3072Pkcs15Sha256);
        test_dsa!(rsa_dsa_manager);
    }

    #[test]
    fn test_rsa_3072_pss_sha256() {
        let rsa_dsa_manager = RsaDsaManager::new(DsaType::Rsa3072PssSha256);
        test_dsa!(rsa_dsa_manager);
    }

    #[test]
    fn test_rsa_4096_pkcs15_sha384() {
        let rsa_dsa_manager = RsaDsaManager::new(DsaType::Rsa4096Pkcs15Sha384);
        test_dsa!(rsa_dsa_manager);
    }

    #[test]
    fn test_rsa_4096_pss_sha384() {
        let rsa_dsa_manager = RsaDsaManager::new(DsaType::Rsa4096PssSha384);
        test_dsa!(rsa_dsa_manager);
    }
}
