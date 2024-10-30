use crate::dsa::common::prehash_dsa_info::PrehashDsaInfo;
use crate::dsa::common::prehash_dsa_trait::PrehashDsa;
use crate::dsa::common::prehash_dsa_type::PrehashDsaType;
use crate::QuantCryptError;

use rand_core::SeedableRng;

// When IPD feature is not enabled
use fips204::ml_dsa_44;
use fips204::ml_dsa_65;
use fips204::ml_dsa_87;
use fips204::traits::{SerDes, Signer, Verifier};

type Result<T> = std::result::Result<T, QuantCryptError>;

macro_rules! sign_ml {
    ($ml_type:ident, $sk:expr, $msg:expr) => {{
        if $sk.len() != $ml_type::SK_LEN {
            return Err(QuantCryptError::InvalidPrivateKey);
        }

        // Convert sk to a fixed-size array [u8; SK_LEN]
        let mut sk_buf = [0u8; $ml_type::SK_LEN];
        sk_buf.copy_from_slice($sk);

        // Try to create a private key from the byte array
        let sk = $ml_type::PrivateKey::try_from_bytes(sk_buf)
            .map_err(|_| QuantCryptError::SignatureFailed)?;

        // Try signing the message
        let sig = sk
            .try_sign($msg, &[]) // Empty context
            .map_err(|_| QuantCryptError::SignatureFailed)?;

        // Convert the signature to a Vec<u8> and return it
        let sig: Vec<u8> = sig.to_vec();
        Ok(sig)
    }};
}

macro_rules! verify_ml {
    ($ml_type:ident, $pk: expr, $msg: expr, $signature: expr) => {{
        if $pk.len() != $ml_type::PK_LEN {
            return Err(QuantCryptError::InvalidPublicKey);
        }

        if $signature.len() != $ml_type::SIG_LEN {
            return Err(QuantCryptError::InvalidSignature);
        }

        // Convert pk to [u8; 1312]
        let mut pk_buf = [0u8; $ml_type::PK_LEN];
        pk_buf.copy_from_slice($pk);

        let mut sig_buf = [0u8; $ml_type::SIG_LEN];
        sig_buf.copy_from_slice($signature);

        let pk = $ml_type::PublicKey::try_from_bytes(pk_buf)
            .map_err(|_| QuantCryptError::InvalidPublicKey)?;

        let result = Ok(pk.verify($msg, &sig_buf, &[]));

        result
    }};
}

macro_rules! get_public_key {
    ($sig_type:ident, $sk:expr) => {{
        if $sk.len() != $sig_type::SK_LEN {
            return Err(QuantCryptError::InvalidPrivateKey);
        }
        let mut sk_buf = [0u8; $sig_type::SK_LEN];
        sk_buf.copy_from_slice($sk);
        let pk = $sig_type::PrivateKey::try_from_bytes(sk_buf)
            .map_err(|_| QuantCryptError::InvalidPrivateKey)?;
        Ok(pk.get_public_key().into_bytes().to_vec())
    }};
}

#[derive(Clone)]
pub struct MlDsaManager {
    pub dsa_info: PrehashDsaInfo,
}

impl PrehashDsa for MlDsaManager {
    /// Create a new DSA instance
    ///
    /// # Arguments
    ///
    /// * `dsa_type` - The type of DSA to create
    fn new(dsa_type: PrehashDsaType) -> Result<Self> {
        let dsa_info = PrehashDsaInfo::new(dsa_type);
        Ok(Self { dsa_info })
    }

    /// Generate a keypair using the specified RNG
    ///
    /// # Arguments
    ///
    /// * `rng` - A random number generator
    ///
    /// # Returns
    ///
    /// A tuple containing the public and secret keys (pk, sk).
    fn key_gen_with_rng(
        &mut self,
        rng: &mut impl rand_core::CryptoRngCore,
    ) -> Result<(Vec<u8>, Vec<u8>)> {
        match self.dsa_info.dsa_type {
            PrehashDsaType::MlDsa44 => {
                let (pk, sk) = ml_dsa_44::try_keygen_with_rng(rng)
                    .map_err(|_| QuantCryptError::KeyPairGenerationFailed)?;
                let pk = pk.into_bytes().to_vec();
                let sk = sk.into_bytes().to_vec();
                Ok((pk, sk))
            }
            PrehashDsaType::MlDsa65 => {
                let (pk, sk) = ml_dsa_65::try_keygen_with_rng(rng)
                    .map_err(|_| QuantCryptError::KeyPairGenerationFailed)?;
                let pk = pk.into_bytes().to_vec();
                let sk = sk.into_bytes().to_vec();
                Ok((pk, sk))
            }
            PrehashDsaType::MlDsa87 => {
                let (pk, sk) = ml_dsa_87::try_keygen_with_rng(rng)
                    .map_err(|_| QuantCryptError::KeyPairGenerationFailed)?;
                let pk = pk.into_bytes().to_vec();
                let sk = sk.into_bytes().to_vec();
                Ok((pk, sk))
            }
            _ => Err(QuantCryptError::NotImplemented),
        }
    }

    /// Generate a keypair using the default RNG ChaCha20Rng
    ///
    /// # Returns
    ///
    /// A tuple containing the public and secret keys (pk, sk)
    fn key_gen(&mut self) -> Result<(Vec<u8>, Vec<u8>)> {
        let mut rng = rand_chacha::ChaCha20Rng::from_entropy();
        self.key_gen_with_rng(&mut rng)
    }

    /// Sign a message
    ///
    /// # Arguments
    ///
    /// * `msg` - The message to sign
    /// * `sk` - The secret key
    /// * `ctx` - The context
    /// 
    /// # Returns
    ///
    /// The signature
    fn sign(&self, sk: &[u8], msg: &[u8], ctx: Option<&[u8]>) -> Result<Vec<u8>> {
        //TODO: Implement this
        match self.dsa_info.dsa_type {
            PrehashDsaType::MlDsa44 => sign_ml!(ml_dsa_44, sk, msg),
            PrehashDsaType::MlDsa65 => sign_ml!(ml_dsa_65, sk, msg),
            PrehashDsaType::MlDsa87 => sign_ml!(ml_dsa_87, sk, msg),
            _ => Err(QuantCryptError::NotImplemented),
        }
    }

    fn sign_prehash(&self, sk: &[u8], msg: &[u8], ctx: Option<&[u8]>, ph: &[u8]) -> Result<Vec<u8>>{
        //TODO: Implement this
        Ok(vec![0])
    }

    /// Verify a signature
    ///
    /// # Arguments
    ///
    /// * `msg` - The message to verify
    /// * `pk` - The public key
    /// * `sig` - The signature
    /// * `ctx` - The context
    ///
    /// # Returns
    ///
    /// A boolean indicating if the signature is valid
    fn verify(&self, pk: &[u8], msg: &[u8], signature: &[u8], ctx: Option<&[u8]>) -> Result<bool> {
        // TODO: Use context
        match self.dsa_info.dsa_type {
            PrehashDsaType::MlDsa44 => {
                verify_ml!(ml_dsa_44, pk, msg, signature)
            }
            PrehashDsaType::MlDsa65 => {
                verify_ml!(ml_dsa_65, pk, msg, signature)
            }
            PrehashDsaType::MlDsa87 => {
                verify_ml!(ml_dsa_87, pk, msg, signature)
            }
            _ => Err(QuantCryptError::NotImplemented),
        }
    }

    fn verify_prehash(&self, pk: &[u8], msg: &[u8], signature: &[u8], ctx: Option<&[u8]>, ph: &[u8]) -> Result<bool>{
        //TODO: Implement this
        Ok(false)
    }

    /// Get DSA metadata information such as the key lengths,
    /// size of signature, etc.
    fn get_dsa_info(&self) -> PrehashDsaInfo {
        self.dsa_info.clone()
    }

    fn get_public_key(&self, sk: &[u8]) -> Result<Vec<u8>> {
        match self.dsa_info.dsa_type {
            PrehashDsaType::MlDsa44 => get_public_key!(ml_dsa_44, sk),
            PrehashDsaType::MlDsa65 => get_public_key!(ml_dsa_65, sk),
            PrehashDsaType::MlDsa87 => get_public_key!(ml_dsa_87, sk),
            _ => Err(QuantCryptError::NotImplemented),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsa::common::prehash_dsa_type::PrehashDsaType;
    use crate::dsa::common::macros::test_prehash_dsa;

    // #[test]
    // fn test_ml_dsa_44() {
    //     let dsa = MlDsaManager::new(PrehashDsaType::MlDsa44);
    //     test_prehash_dsa!(dsa);
    // }

    // #[test]
    // fn test_ml_dsa_65() {
    //     let dsa = MlDsaManager::new(PrehashDsaType::MlDsa65);
    //     test_prehash_dsa!(dsa);
    // }

    // #[test]
    // fn test_ml_dsa_87() {
    //     let dsa = MlDsaManager::new(PrehashDsaType::MlDsa87);
    //     test_prehash_dsa!(dsa);
    // }
}
