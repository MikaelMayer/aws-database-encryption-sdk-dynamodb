// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![allow(warnings)]
#![allow(nonstandard_style)]

// Extern methods with a foreign module name
pub mod RSAEncryption {
    pub mod RSA {
        use crate::software::amazon::cryptography::primitives::internaldafny::types::RSAPaddingMode;
        use crate::*;
        use ::std::rc::Rc;
        use aws_lc_rs::encoding::AsDer;
        use aws_lc_rs::error::Unspecified;
        use aws_lc_rs::rsa;
        use aws_lc_rs::rsa::KeySize;
        use aws_lc_rs::signature::KeyPair;
        use aws_lc_rs::signature::RsaKeyPair;
        use der::{
            asn1::AnyRef, Decode, DecodeValue, Encode, Header, Reader, Sequence, SliceReader,
        };
        use pem;
        use software::amazon::cryptography::primitives::internaldafny::types::Error as DafnyError;

        fn key_size_from_length(length: i32) -> KeySize {
            match length {
                2048 => KeySize::Rsa2048,
                3072 => KeySize::Rsa3072,
                4096 => KeySize::Rsa4096,
                8192 => KeySize::Rsa8192,
                _ => panic!("Bad length for GenerateKeyPair"),
            }
        }

        fn error(s: &str) -> Rc<DafnyError> {
            Rc::new(DafnyError::AwsCryptographicPrimitivesError {
                message:
                    dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(s),
            })
        }

        #[allow(non_snake_case)]
        pub fn GenerateKeyPairExtern(
            length_bits: i32,
        ) -> (::dafny_runtime::Sequence<u8>, ::dafny_runtime::Sequence<u8>) {
            let pair = RsaKeyPair::generate(key_size_from_length(length_bits)).unwrap();
            let private = pair.as_der();
            let private = Vec::new();
            (
                pair.public_key().as_ref().iter().cloned().collect(),
                private.iter().cloned().collect(),
            )
        }

        fn get_alg_for_padding(mode: &RSAPaddingMode) -> &'static rsa::OaepAlgorithm {
            match mode {
                RSAPaddingMode::PKCS1 {} => panic!("No support for PKCS1 in Rust"),
                RSAPaddingMode::OAEP_SHA1 {} => &rsa::OAEP_SHA1_MGF1SHA1,
                RSAPaddingMode::OAEP_SHA256 {} => &rsa::OAEP_SHA256_MGF1SHA256,
                RSAPaddingMode::OAEP_SHA384 {} => &rsa::OAEP_SHA384_MGF1SHA384,
                RSAPaddingMode::OAEP_SHA512 {} => &rsa::OAEP_SHA512_MGF1SHA512,
            }
        }

        #[allow(non_snake_case)]
        pub fn GetRSAKeyModulusLengthExtern(
            publicKey: &::dafny_runtime::Sequence<u8>,
        ) -> Rc<Wrappers::Result<u32, Rc<DafnyError>>> {
            let publicKey: Vec<u8> = publicKey.iter().collect();
            let pair = RsaKeyPair::from_pkcs8(&publicKey).unwrap();
            Rc::new(Wrappers::Result::Success {
                value: pair.public_modulus_len() as u32,
            })
            // let publicKey = std::str::from_utf8(&publicKey).unwrap();
            // let my_pem = pem::parse(publicKey).unwrap();
            // let my_key = my_pem.contents();
        }

        fn decrypt_extern(
            mode: &RSAPaddingMode,
            private_key: &[u8],
            cipher_text: &[u8],
        ) -> Result<Vec<u8>, Unspecified> {
            let mode = get_alg_for_padding(mode);
            let private_key = rsa::PrivateDecryptingKey::from_pkcs8(&private_key)?;
            let private_key = rsa::OaepPrivateDecryptingKey::new(private_key)?;
            let mut message: Vec<u8> = Vec::new();
            message.resize(message.len(), 0);
            let message = private_key.decrypt(mode, &cipher_text, &mut message, None)?;
            Ok(message.iter().cloned().collect())
        }

        #[allow(non_snake_case)]
        pub fn DecryptExtern(
            mode: &RSAPaddingMode,
            private_key: &::dafny_runtime::Sequence<u8>,
            cipher_text: &::dafny_runtime::Sequence<u8>,
        ) -> Rc<Wrappers::Result<::dafny_runtime::Sequence<u8>, Rc<DafnyError>>> {
            let private_key: Vec<u8> = private_key.iter().collect();
            let cipher_text: Vec<u8> = cipher_text.iter().collect();
            match decrypt_extern(mode, &private_key, &cipher_text) {
                Ok(x) => Rc::new(Wrappers::Result::Success {
                    value: x.iter().cloned().collect(),
                }),
                Err(e) => {
                    let msg = format!("{}", e);
                    Rc::new(Wrappers::Result::Failure { error: error(&msg) })
                }
            }
        }

        fn encrypt_extern(
            mode: &RSAPaddingMode,
            public_key: &[u8],
            message: &[u8],
        ) -> Result<Vec<u8>, Unspecified> {
            let mode = get_alg_for_padding(mode);
            let public_key = rsa::PublicEncryptingKey::from_der(&public_key)?; // wrong
            let public_key = rsa::OaepPublicEncryptingKey::new(public_key)?;
            let mut ciphertext: Vec<u8> = Vec::new();
            ciphertext.resize(message.len() + public_key.key_size_bytes(), 0);
            let cipher_text = public_key.encrypt(mode, &message, &mut ciphertext, None)?;
            Ok(cipher_text.iter().cloned().collect())
        }

        #[allow(non_snake_case)]
        pub fn EncryptExtern(
            mode: &RSAPaddingMode,
            public_key: &::dafny_runtime::Sequence<u8>,
            message: &::dafny_runtime::Sequence<u8>,
        ) -> Rc<Wrappers::Result<::dafny_runtime::Sequence<u8>, Rc<DafnyError>>> {
            let public_key: Vec<u8> = public_key.iter().collect();
            let message: Vec<u8> = message.iter().collect();
            match encrypt_extern(mode, &public_key, &message) {
                Ok(x) => Rc::new(Wrappers::Result::Success {
                    value: x.iter().cloned().collect(),
                }),
                Err(e) => {
                    let msg = format!("{}", e);
                    Rc::new(Wrappers::Result::Failure { error: error(&msg) })
                }
            }
        }
    }
}
