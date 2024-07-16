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
        use aws_lc_rs::rsa::KeySize;
        use aws_lc_rs::rsa::OaepAlgorithm;
        use aws_lc_rs::rsa::OaepPrivateDecryptingKey;
        use aws_lc_rs::rsa::OaepPublicEncryptingKey;
        use aws_lc_rs::rsa::PrivateDecryptingKey;
        use aws_lc_rs::rsa::PublicEncryptingKey;
        use aws_lc_rs::signature::KeyPair;
        use aws_lc_rs::signature::RsaKeyPair;
        use pem;
        use software::amazon::cryptography::primitives::internaldafny::types::Error as DafnyError;
        use std::error;

        pub fn key_size_from_length(length: i32) -> KeySize {
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
            let public_key = pem::Pem::new("RSA PUBLIC KEY", pair.public_key().as_ref());
            let public_key = pem::encode(&public_key);

            let private_key = pem::Pem::new("RSA PRIVATE KEY", pair.as_der().unwrap().as_ref());
            let private_key = pem::encode(&private_key);
            (
                public_key.as_bytes().iter().cloned().collect(),
                private_key.as_bytes().iter().cloned().collect(),
            )
        }

        fn get_alg_for_padding(mode: &RSAPaddingMode) -> &'static OaepAlgorithm {
            match mode {
                RSAPaddingMode::PKCS1 {} => panic!("No support for PKCS1 in Rust"),
                RSAPaddingMode::OAEP_SHA1 {} => &aws_lc_rs::rsa::OAEP_SHA1_MGF1SHA1,
                RSAPaddingMode::OAEP_SHA256 {} => &aws_lc_rs::rsa::OAEP_SHA256_MGF1SHA256,
                RSAPaddingMode::OAEP_SHA384 {} => &aws_lc_rs::rsa::OAEP_SHA384_MGF1SHA384,
                RSAPaddingMode::OAEP_SHA512 {} => &aws_lc_rs::rsa::OAEP_SHA512_MGF1SHA512,
            }
        }

        fn get_modulus(public_key: &[u8]) -> Result<u32, String> {
            let public_key = std::str::from_utf8(public_key).map_err(|e| format!("{:?}", e))?;
            let public_key = pem::parse(public_key).map_err(|e| format!("{:?}", e))?;
            let public_key = PublicEncryptingKey::from_der(public_key.contents())
                .map_err(|e| format!("{:?}", e))?;
            Ok(public_key.key_size_bits() as u32)
        }

        #[allow(non_snake_case)]
        pub fn GetRSAKeyModulusLengthExtern(
            public_key: &::dafny_runtime::Sequence<u8>,
        ) -> Rc<Wrappers::Result<u32, Rc<DafnyError>>> {
            let public_key: Vec<u8> = public_key.iter().collect();
            match get_modulus(&public_key) {
                Ok(v) => Rc::new(Wrappers::Result::Success { value: v }),
                Err(e) => Rc::new(Wrappers::Result::Failure { error: error(&e) }),
            }
        }

        fn decrypt_extern(
            mode: &RSAPaddingMode,
            private_key: &[u8],
            cipher_text: &[u8],
        ) -> Result<Vec<u8>, String> {
            let mode = get_alg_for_padding(mode);

            // let private_key = std::str::from_utf8(private_key).map_err(|e| format!("{:?}", e))?;
            // let private_key = pem::parse(private_key).map_err(|e| format!("{:?}", e))?;
            let private_key =
                PrivateDecryptingKey::from_pkcs8(private_key).map_err(|e| format!("{:?}", e))?;
            let private_key =
                OaepPrivateDecryptingKey::new(private_key).map_err(|e| format!("{:?}", e))?;
            let mut message: Vec<u8> = Vec::new();
            message.resize(message.len(), 0);
            let message = private_key
                .decrypt(mode, &cipher_text, &mut message, None)
                .map_err(|e| format!("{:?}", e))?;
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
        ) -> Result<Vec<u8>, String> {
            let mode = get_alg_for_padding(mode);

            let public_key = std::str::from_utf8(public_key).map_err(|e| format!("{:?}", e))?;
            let public_key = pem::parse(public_key).map_err(|e| format!("{:?}", e))?;
            let public_key = PublicEncryptingKey::from_der(public_key.contents())
                .map_err(|e| format!("{:?}", e))?;
            let public_key =
                OaepPublicEncryptingKey::new(public_key).map_err(|e| format!("{:?}", e))?;
            let mut ciphertext: Vec<u8> = Vec::new();
            ciphertext.resize(message.len() + public_key.key_size_bytes(), 0);
            let cipher_text = public_key
                .encrypt(mode, &message, &mut ciphertext, None)
                .map_err(|e| format!("{:?}", e))?;
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

        // #[cfg(test)]
        // mod tests {
        //     use super::*;
        //     #[test]
        //     fn test_generate() {
        //         // let foo = generate_key_pair(2048);
        //         // println!("{:?}", foo);

        //         let (public_key, private_key) = GenerateKeyPairExtern(2048);

        //         // let modulus = GetRSAKeyModulusLengthExtern(&public_key);
        //         // println!("{:?}", modulus);
        //         // let modulus = modulus.UnwrapOr(&42);
        //         // assert_eq!(modulus, 2048);

        //         let mode = RSAPaddingMode::OAEP_SHA256 {};
        //         let plain_text: ::dafny_runtime::Sequence<u8> =
        //             [1u8, 2, 3, 4, 5].iter().cloned().collect();
        //         let empty: ::dafny_runtime::Sequence<u8> = [].iter().cloned().collect();
        //         let cipher_text = DecryptExtern(&mode, &private_key, &plain_text); //.UnwrapOr(&empty);
        //         println!("{:?}", cipher_text);
        //     }
        // }
    }
}
