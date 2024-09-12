// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
use aws_smithy_types::error::operation::BuildError;

#[derive(::std::clone::Clone, ::std::fmt::Debug, ::std::cmp::PartialEq)]
pub struct Client {
    pub(crate) dafny_client: ::dafny_runtime::Object<dyn crate::software::amazon::cryptography::primitives::internaldafny::types::IAwsCryptographicPrimitivesClient>
}

impl Client {
    /// Creates a new client from the service [`Config`](crate::Config).
    #[track_caller]
    pub fn from_conf(
        conf: crate::primitives::types::crypto_config::CryptoConfig,
    ) -> Result<Self, BuildError> {
        let inner =
            crate::aws::cryptography::primitives::internaldafny::_default::AtomicPrimitives(
                &crate::primitives::conversions::crypto_config::_crypto_config::to_dafny(conf),
            );
        if matches!(
            inner.as_ref(),
            crate::Wrappers::Result::Failure { .. }
        ) {
            // TODO: convert error - the potential types are not modeled!
            return Err(BuildError::other(
                ::aws_smithy_types::error::metadata::ErrorMetadata::builder()
                    .message("Invalid client config")
                    .build(),
            ));
        }
        Ok(Self {
            dafny_client: ::dafny_runtime::upcast_object()(inner.Extract())
        })
    }
}

mod aes_decrypt;

mod aes_encrypt;

mod aes_kdf_counter_mode;

mod compress_public_key;

mod decompress_public_key;

mod derive_shared_secret;

mod digest;

mod ecdsa_sign;

mod ecdsa_verify;

mod generate_ecc_key_pair;

mod generate_ecdsa_signature_key;

mod generate_random_bytes;

mod generate_rsa_key_pair;

mod get_public_key_from_private_key;

mod get_rsa_key_modulus_length;

mod hkdf;

mod hkdf_expand;

mod hkdf_extract;

mod h_mac;

mod kdf_counter_mode;

mod parse_public_key;

mod rsa_decrypt;

mod rsa_encrypt;

mod validate_public_key;
