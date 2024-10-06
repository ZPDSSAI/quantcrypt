use crate::kem::common::kem_trait::Kem;
use cms::builder::{
    ContentEncryptionAlgorithm, KekRecipientInfoBuilder, KeyAgreeRecipientInfoBuilder,
    KeyTransRecipientInfoBuilder, OtherRecipientInfoBuilder, PasswordRecipientInfoBuilder,
};
use cms::content_info::ContentInfo;
use cms::enveloped_data::{OriginatorInfo, UserKeyingMaterial};
use const_oid::db::rfc5911::ID_ENVELOPED_DATA;
use der::{Decode, Encode};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use x509_cert::attr::{Attribute, Attributes};

use crate::{kem::kem_manager, CeaType, Certificate, QuantCryptError};

use crate::cms::asn1::kemri_builder::KemRecipientInfoBuilder;

type Result<T> = std::result::Result<T, QuantCryptError>;

const ALLOWED_CAE_TYPES: [CeaType; 3] = [
    CeaType::Aes128CbcPad,
    CeaType::Aes192CbcPad,
    CeaType::Aes256CbcPad,
];

pub struct EnvelopedDataBuilder<'a> {
    originator_info: Option<OriginatorInfo>,
    plaintext: Vec<u8>,
    cae_type: CeaType,
    unprotected_attributes: Option<Attributes>,
    kemri_builders: Vec<KemRecipientInfoBuilder>,
    kek_builders: Vec<KekRecipientInfoBuilder>,
    ktri_builders: Vec<KeyTransRecipientInfoBuilder<'a, ChaCha20Rng>>,
    kari_builders: Vec<KeyAgreeRecipientInfoBuilder>,
    pwri_builders: Vec<PasswordRecipientInfoBuilder>,
    ori_builders: Vec<OtherRecipientInfoBuilder>,
}

impl<'a> EnvelopedDataBuilder<'a> {
    pub fn new(plaintext: Vec<u8>, cae_type: CeaType) -> Result<Self> {
        if !ALLOWED_CAE_TYPES.contains(&cae_type) {
            return Err(QuantCryptError::UnsupportedOperation);
        }

        Ok(Self {
            originator_info: None,
            plaintext,
            cae_type,
            unprotected_attributes: None,
            kemri_builders: Vec::new(),
            kek_builders: Vec::new(),
            ktri_builders: Vec::new(),
            kari_builders: Vec::new(),
            pwri_builders: Vec::new(),
            ori_builders: Vec::new(),
        })
    }

    pub fn attribute(&mut self, attribute: Attribute) -> Result<()> {
        if let Some(attributes) = &mut self.unprotected_attributes {
            attributes
                .insert(attribute)
                .map_err(|_| QuantCryptError::InvalidAttribute)?;
            Ok(())
        } else {
            self.unprotected_attributes = Some(Attributes::new());
            let attributes = self.unprotected_attributes.as_mut().unwrap();
            attributes
                .insert(attribute)
                .map_err(|_| QuantCryptError::InvalidAttribute)?;
            Ok(())
        }
    }

    pub fn kem_recipient(
        &mut self,
        cert: Certificate,
        kdf_oid: String,
        wrap_oid: String,
        ukm: Option<UserKeyingMaterial>,
    ) -> Result<()> {
        if !cert.is_key_encipherment_enabled() {
            return Err(QuantCryptError::InvalidCertificate);
        }
        let kem_manager = kem_manager::KemManager::new_from_oid(&cert.get_public_key_oid())?;
        let kemri_builder = KemRecipientInfoBuilder::new(cert, kem_manager, kdf_oid, wrap_oid, ukm);
        self.kemri_builders.push(kemri_builder);
        Ok(())
    }

    pub fn kek_recipient(&mut self, builder: KekRecipientInfoBuilder) {
        self.kek_builders.push(builder);
    }

    pub fn ktri_recipient(&mut self, builder: KeyTransRecipientInfoBuilder<'a, ChaCha20Rng>) {
        self.ktri_builders.push(builder);
    }

    pub fn kari_recipient(&mut self, builder: KeyAgreeRecipientInfoBuilder) {
        self.kari_builders.push(builder);
    }

    pub fn pwri_recipient(&mut self, builder: PasswordRecipientInfoBuilder) {
        self.pwri_builders.push(builder);
    }

    pub fn ori_recipient(&mut self, builder: OtherRecipientInfoBuilder) {
        self.ori_builders.push(builder);
    }

    pub fn originator_info(&mut self, originator_info: OriginatorInfo) {
        self.originator_info = Some(originator_info);
    }

    pub fn build(self) -> Result<Vec<u8>> {
        let cea = match self.cae_type {
            CeaType::Aes128CbcPad => ContentEncryptionAlgorithm::Aes128Cbc,
            CeaType::Aes192CbcPad => ContentEncryptionAlgorithm::Aes192Cbc,
            CeaType::Aes256CbcPad => ContentEncryptionAlgorithm::Aes256Cbc,
            _ => return Err(QuantCryptError::UnsupportedOperation),
        };

        let mut builder = cms::builder::EnvelopedDataBuilder::new(
            self.originator_info,
            &self.plaintext,
            cea,
            self.unprotected_attributes,
        )
        .map_err(|_| QuantCryptError::Unknown)?;

        for kemri_builder in self.kemri_builders {
            let kemri = kemri_builder;
            builder
                .add_recipient_info(kemri)
                .map_err(|_| QuantCryptError::Unknown)?;
        }

        for kek_builder in self.kek_builders {
            builder
                .add_recipient_info(kek_builder)
                .map_err(|_| QuantCryptError::Unknown)?;
        }

        for ktri_builder in self.ktri_builders {
            builder
                .add_recipient_info(ktri_builder)
                .map_err(|_| QuantCryptError::Unknown)?;
        }

        for kari_builder in self.kari_builders {
            builder
                .add_recipient_info(kari_builder)
                .map_err(|_| QuantCryptError::Unknown)?;
        }

        for pwri_builder in self.pwri_builders {
            builder
                .add_recipient_info(pwri_builder)
                .map_err(|_| QuantCryptError::Unknown)?;
        }

        for ori_builder in self.ori_builders {
            builder
                .add_recipient_info(ori_builder)
                .map_err(|_| QuantCryptError::Unknown)?;
        }

        let mut rng = ChaCha20Rng::from_entropy();

        let enveloped_data = builder
            .build_with_rng(&mut rng)
            .map_err(|_| QuantCryptError::Unknown)?;

        let cms_content_info = ContentInfo {
            content_type: ID_ENVELOPED_DATA,
            content: der::Any::from_der(
                &enveloped_data
                    .to_der()
                    .map_err(|_| QuantCryptError::Unknown)?,
            )
            .map_err(|_| QuantCryptError::Unknown)?,
        };

        let ci_der = cms_content_info
            .to_der()
            .map_err(|_| QuantCryptError::Unknown)?;

        Ok(ci_der)
    }
}

#[cfg(test)]
mod tests {
    use crate::dsa::common::config::oids::Oid;
    use crate::dsa::common::dsa_trait::Dsa;
    use crate::dsa::common::dsa_type::DsaType;
    use crate::dsa::dsa_manager::DsaManager;
    use crate::kem::common::config::oids::Oid as _;
    use crate::kem::common::kem_type::KemType;
    use crate::{CertValidity, CertificateBuilder, PrivateKey, PublicKey};
    use x509_cert::builder::Profile;

    use crate::{
        kdf::common::config::oids::Oid as _, wrap::common::config::oids::Oid as _, KdfType,
        WrapType,
    };

    use super::*;

    #[test]
    fn test_enveloped_data_kemri() {
        let plaintext = b"Hello, World!".to_vec();
        let cae_type = CeaType::Aes256CbcPad;
        let mut builder = EnvelopedDataBuilder::new(plaintext.clone(), cae_type)
            .expect("Failed to create EnvelopedDataBuilder");

        let cert_ta_1 = Certificate::from_der(include_bytes!("../../test/data/cms_cw/ta.der"))
            .expect("Failed to create Certificate");

        let kdf_oid = KdfType::HkdfWithSha256.get_oid();
        let wrap_oid = WrapType::Aes256.get_oid();
        let ukm = None;

        let result =
            builder.kem_recipient(cert_ta_1, kdf_oid.clone(), wrap_oid.clone(), ukm.clone());
        assert!(result.is_err());
        assert!(matches!(result, Err(QuantCryptError::InvalidCertificate)));

        let cert_ee_1: Certificate = Certificate::from_der(include_bytes!(
            "../../test/data/cms_cw/1.3.6.1.4.1.22554.5.6.1_ML-KEM-512-ipd_ee.der"
        ))
        .expect("Failed to create Certificate");

        let sk_bytes = include_bytes!(
            "../../test/data/cms_cw/1.3.6.1.4.1.22554.5.6.1_ML-KEM-512-ipd_priv.der"
        );
        let sk_ee_1 = PrivateKey::from_der(sk_bytes).expect("Failed to create PrivateKey");

        builder
            .kem_recipient(
                cert_ee_1.clone(),
                kdf_oid.clone(),
                wrap_oid.clone(),
                ukm.clone(),
            )
            .unwrap();

        // Add a new recipient (of a completely different type)
        let (ta_pk_2, ta_sk_2) = DsaManager::new(DsaType::MlDsa44Rsa2048PssSha256)
            .unwrap()
            .key_gen()
            .unwrap();
        let ta_pk_2 =
            PublicKey::new(&DsaType::MlDsa44Rsa2048PssSha256.get_oid(), &ta_pk_2).unwrap();
        let ta_sk_2 = PrivateKey::new(
            &DsaType::MlDsa44Rsa2048PssSha256.get_oid(),
            &ta_sk_2,
            Some(ta_pk_2.clone()),
        )
        .unwrap();
        let ta_cert_2 = CertificateBuilder::new(
            Profile::Root,
            None,
            CertValidity::new(None, "2025-01-01T00:00:00Z").unwrap(),
            "CN=test.com".to_string(),
            ta_pk_2,
            &ta_sk_2,
        )
        .unwrap()
        .build()
        .unwrap();

        let (ee_pk2, ee_sk2) = kem_manager::KemManager::new(KemType::MlKem768BrainpoolP256r1)
            .unwrap()
            .key_gen()
            .unwrap();

        let ee_pk2 = PublicKey::new(&KemType::MlKem768BrainpoolP256r1.get_oid(), &ee_pk2).unwrap();
        let ee_sk2 = PrivateKey::new(
            &KemType::MlKem768BrainpoolP256r1.get_oid(),
            &ee_sk2.clone(),
            Some(ee_pk2.clone()),
        )
        .unwrap();
        //let spki = SubjectPublicKeyInfo::from_key(ee_pk2).unwrap();
        let validity = CertValidity::new(None, "2025-01-01T00:00:00Z").unwrap(); // Not before is now
        let serial_no = None; // This will generate a random serial number
        let signer = ta_sk_2;
        let subject = "CN=sub.test.com".to_string();

        let ee_cert = CertificateBuilder::new(
            Profile::Leaf {
                issuer: ta_cert_2.get_subject(),
                enable_key_agreement: false,
                enable_key_encipherment: true,
            },
            serial_no,
            validity,
            subject,
            ee_pk2,
            &signer,
        )
        .unwrap()
        .build()
        .unwrap();

        builder
            .kem_recipient(ee_cert.clone(), kdf_oid, wrap_oid, ukm)
            .unwrap();

        let result = builder.build().expect("Failed to build enveloped data");

        // Test if we can decrypt the enveloped data
        let pt = crate::cms::cms_manager::CmsManager::decrypt_kemri(&result, &sk_ee_1, &cert_ee_1)
            .expect("Failed to decrypt enveloped data");

        assert_eq!(pt, plaintext);

        // Test if we can decrypt the enveloped data with the second recipient
        let pt = crate::cms::cms_manager::CmsManager::decrypt_kemri(&result, &ee_sk2, &ee_cert)
            .expect("Failed to decrypt enveloped data");

        // MAGIC: We can decrypt the same CMS message with two different private keys!
        // Any sufficiently advanced technology is indistinguishable from magic.
        // - Arthur C. Clarke

        assert_eq!(pt, plaintext);
    }
}
