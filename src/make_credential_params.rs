/*!
make_credential API parameters
*/

use crate::credential_management_params::CredentialProtectionPolicy;
use crate::public_key::PublicKey;
use crate::public_key_credential_descriptor::PublicKeyCredentialDescriptor;
use crate::str_buf::StrBuf;
use std::fmt;
use strum_macros::AsRefStr;

/// Attestation Object
/// [https://www.w3.org/TR/webauthn/#sctn-attestation](https://www.w3.org/TR/webauthn/#sctn-attestation)
#[derive(Debug, Default)]
pub struct Attestation {
    pub fmt: String,
    pub rpid_hash: Vec<u8>,
    pub flags_user_present_result: bool,
    pub flags_user_verified_result: bool,
    pub flags_attested_credential_data_included: bool,
    pub flags_extension_data_included: bool,
    pub sign_count: u32,
    pub aaguid: Vec<u8>,
    pub credential_descriptor: PublicKeyCredentialDescriptor,
    pub credential_publickey: PublicKey,
    pub extensions: Vec<Extension>,
    pub auth_data: Vec<u8>,

    pub attstmt_alg: i32,
    pub attstmt_sig: Vec<u8>,
    pub attstmt_x5c: Vec<Vec<u8>>,
}

impl fmt::Display for Attestation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut strbuf = StrBuf::new(42);
        strbuf
            .appenh("- rpid_hash", &self.rpid_hash)
            .append(
                "- flags_user_present_result",
                &self.flags_user_present_result,
            )
            .append(
                "- flags_attested_credential_data_included",
                &self.flags_attested_credential_data_included,
            )
            .append(
                "- flags_extension_data_included",
                &self.flags_extension_data_included,
            )
            .append("- sign_count", &self.sign_count)
            .appenh("- aaguid", &self.aaguid)
            .append("- credential_descriptor", &self.credential_descriptor)
            .append("- credential_publickey", &self.credential_publickey)
            .append("- attstmt_alg", &self.attstmt_alg)
            .appenh("- attstmt_sig", &self.attstmt_sig)
            .append("- attstmt_x5c_num", &self.attstmt_x5c.len());

        for ex in &self.extensions {
            strbuf.append("- extension", &format!("{:?}", ex));
        }

        write!(f, "{}", strbuf.build())
    }
}

#[derive(Debug, Clone, strum_macros::Display, AsRefStr)]
pub enum Extension {
    #[strum(serialize = "credProtect")]
    CredProtect(Option<CredentialProtectionPolicy>),
    #[strum(serialize = "credBlob")]
    CredBlob(Option<Vec<u8>>), //  "credBlob": Byte String containing the credBlob value
    #[strum(serialize = "minPinLength")]
    MinPinLength(Option<bool>), // "minPinLength": true
    #[strum(serialize = "hmac-secret")]
    HmacSecret(Option<bool>),
}

#[derive(Debug, Copy, Clone)]
pub enum CredentialSupportedKeyType {
    Ecdsa256 = -7,
    Ed25519 = -8,
}

impl std::default::Default for CredentialSupportedKeyType {
    fn default() -> Self {
        Self::Ecdsa256
    }
}
