// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![deny(warnings, unconditional_panic)]
#![deny(nonstandard_style)]
#![deny(clippy::all)]

#[allow(non_snake_case)]
pub mod ECDH {
    use crate::software::amazon::cryptography::primitives::internaldafny::types::Error as DafnyError;
    use std::rc::Rc;

    fn error(s: &str) -> Rc<DafnyError> {
        Rc::new(DafnyError::AwsCryptographicPrimitivesError {
            message:
                dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(s),
        })
    }

    pub mod ECCUtils {
        use crate::software::amazon::cryptography::primitives::internaldafny::types::ECDHCurveSpec;
        use crate::software::amazon::cryptography::primitives::internaldafny::types::Error as DafnyError;
        use aws_lc_rs::agreement::PrivateKey;
        use aws_lc_sys;
        use std::rc::Rc;

        fn get_nid(x: &ECDHCurveSpec) -> i32 {
            match x {
                ECDHCurveSpec::ECC_NIST_P256 {} => aws_lc_sys::NID_secp256k1,
                ECDHCurveSpec::ECC_NIST_P384 {} => aws_lc_sys::NID_secp384r1,
                ECDHCurveSpec::ECC_NIST_P521 {} => aws_lc_sys::NID_secp521r1,
                ECDHCurveSpec::SM2 {} => panic!("No SM2 in Rust"),
            }
        }

        pub(crate) fn sec1_compress(data: &[u8], alg: &ECDHCurveSpec) -> Result<Vec<u8>, String> {
            crate::ecdsa::Signature::ECDSA::sec1_convert(
                data,
                get_nid(alg),
                aws_lc_sys::point_conversion_form_t::POINT_CONVERSION_COMPRESSED,
            )
        }

        pub(crate) fn sec1_decompress(data: &[u8], alg: &ECDHCurveSpec) -> Result<Vec<u8>, String> {
            crate::ecdsa::Signature::ECDSA::sec1_convert(
                data,
                get_nid(alg),
                aws_lc_sys::point_conversion_form_t::POINT_CONVERSION_UNCOMPRESSED,
            )
        }

        pub(crate) fn get_alg(x: &ECDHCurveSpec) -> &'static aws_lc_rs::agreement::Algorithm {
            match x {
                ECDHCurveSpec::ECC_NIST_P256 {} => &aws_lc_rs::agreement::ECDH_P256,
                ECDHCurveSpec::ECC_NIST_P384 {} => &aws_lc_rs::agreement::ECDH_P384,
                ECDHCurveSpec::ECC_NIST_P521 {} => &aws_lc_rs::agreement::ECDH_P384,
                ECDHCurveSpec::SM2 {} => panic!("No SM2 in Rust"),
            }
        }

        fn get_public_key(alg: &ECDHCurveSpec, pem: &[u8]) -> Result<Vec<u8>, String> {
            let pem = std::str::from_utf8(pem).map_err(|e| format!("{:?}", e))?;
            let private_key = pem::parse(pem).map_err(|e| format!("{:?}", e))?;
            let private_key =
                PrivateKey::from_private_key_der(get_alg(alg), private_key.contents())
                    .map_err(|e| format!("{:?}", e))?;
            let public_key = private_key
                .compute_public_key()
                .map_err(|e| format!("{:?}", e))?;
            Ok(public_key.as_ref().to_vec())
        }

        pub fn GetPublicKey(
            curve_algorithm: &Rc<ECDHCurveSpec>,
            private_key: &Rc<crate::software::amazon::cryptography::primitives::internaldafny::types::ECCPrivateKey>,
        ) -> Rc<crate::Wrappers::Result<::dafny_runtime::Sequence<u8>, Rc<DafnyError>>> {
            let private_key: Vec<u8> = private_key.pem().iter().collect();
            match get_public_key(curve_algorithm, &private_key) {
                Ok(x) => Rc::new(crate::Wrappers::Result::Success {
                    value: x.iter().cloned().collect(),
                }),
                Err(e) => {
                    let msg = format!("ECDH Get Public Key : {}", e);
                    Rc::new(crate::Wrappers::Result::Failure {
                        error: super::error(&msg),
                    })
                }
            }
        }

        // for the moment, it's valid if we can use it to generate a shared secret
        fn valid_public_key(alg: &ECDHCurveSpec, public_key: &[u8]) -> Result<(), String> {
            let private_key = aws_lc_rs::agreement::PrivateKey::generate(get_alg(alg))
                .map_err(|e| format!("{:?}", e))?;
            let public_key = aws_lc_rs::agreement::UnparsedPublicKey::new(get_alg(alg), public_key);
            match aws_lc_rs::agreement::agree(&private_key, &public_key, "foo", |_x| Ok(false)) {
                Ok(_) => Ok(()),
                Err(_) => Err("Invalid ECDH Public Key".to_string()),
            }
        }

        pub fn ValidatePublicKey(
            curve_algorithm: &Rc<ECDHCurveSpec>,
            public_key: &::dafny_runtime::Sequence<u8>,
        ) -> Rc<crate::Wrappers::Result<bool, Rc<DafnyError>>> {
            let public_key: Vec<u8> = public_key.iter().collect();
            match valid_public_key(curve_algorithm, &public_key) {
                Ok(_) => Rc::new(crate::Wrappers::Result::Success { value: true }),
                Err(e) => {
                    let msg = format!("ECDH Validate Public Key : {}", e);
                    Rc::new(crate::Wrappers::Result::Failure {
                        error: super::error(&msg),
                    })
                }
            }
        }
        pub fn CompressPublicKey(
            public_key: &::dafny_runtime::Sequence<u8>,
            curve_algorithm: &Rc<ECDHCurveSpec>,
        ) -> Rc<crate::Wrappers::Result<::dafny_runtime::Sequence<u8>, Rc<DafnyError>>> {
            let public_key: Vec<u8> = public_key.iter().collect();
            match sec1_compress(&public_key, curve_algorithm) {
                Ok(v) => Rc::new(crate::Wrappers::Result::Success {
                    value: v.iter().cloned().collect(),
                }),
                Err(e) => {
                    let msg = format!("ECDH Compress Public Key {}", e);
                    Rc::new(crate::Wrappers::Result::Failure {
                        error: super::error(&msg),
                    })
                }
            }
        }
        pub fn DecompressPublicKey(
            public_key: &::dafny_runtime::Sequence<u8>,
            curve_algorithm: &Rc<ECDHCurveSpec>,
        ) -> Rc<crate::Wrappers::Result<::dafny_runtime::Sequence<u8>, Rc<DafnyError>>> {
            let public_key: Vec<u8> = public_key.iter().collect();
            match sec1_decompress(&public_key, curve_algorithm) {
                Ok(v) => Rc::new(crate::Wrappers::Result::Success {
                    value: v.iter().cloned().collect(),
                }),
                Err(e) => {
                    let msg = format!("ECDH Decompress Public Key {}", e);
                    Rc::new(crate::Wrappers::Result::Failure {
                        error: super::error(&msg),
                    })
                }
            }
        }
        pub fn ParsePublicKey(
            public_key: &::dafny_runtime::Sequence<u8>,
        ) -> Rc<crate::Wrappers::Result<::dafny_runtime::Sequence<u8>, Rc<DafnyError>>> {
            Rc::new(crate::Wrappers::Result::Success {
                value: public_key.clone(),
            })
            // let public_key: Vec<u8> = publicKey.iter().collect();
            // match aws_lc_rs::rsa::PublicEncryptingKey::from_der(&public_key) {
            //     Ok(_) => Rc::new(crate::Wrappers::Result::Success {
            //         value: publicKey.clone(),
            //     }),
            //     Err(e) => {
            //         let msg = format!("{}", e);
            //         Rc::new(crate::Wrappers::Result::Failure {
            //             error: super::error(&msg),
            //         })
            //     }
            // }
        }
    }
    pub mod DeriveSharedSecret {
        use crate::software::amazon::cryptography::primitives::internaldafny::types::ECDHCurveSpec;
        use crate::software::amazon::cryptography::primitives::internaldafny::types::Error as DafnyError;
        use std::rc::Rc;

        pub fn agree(
            curve_algorithm: &ECDHCurveSpec,
            private_key_pem: &[u8],
            public_key_der: &[u8],
        ) -> Result<Vec<u8>, String> {
            let pem = std::str::from_utf8(private_key_pem).map_err(|e| format!("{:?}", e))?;
            let private_key = pem::parse(pem).map_err(|e| format!("{:?}", e))?;
            let private_key = aws_lc_rs::agreement::PrivateKey::from_private_key_der(
                super::ECCUtils::get_alg(curve_algorithm),
                private_key.contents(),
            )
            .map_err(|e| format!("{:?}", e))?;
            let public_key = aws_lc_rs::agreement::UnparsedPublicKey::new(
                super::ECCUtils::get_alg(curve_algorithm),
                public_key_der,
            );
            let shared: Vec<u8> =
                aws_lc_rs::agreement::agree(&private_key, &public_key, "foo", |x| Ok(x.to_vec()))
                    .map_err(|_e| "Failure in aws_lc_rs::agreement::agree.".to_string())?;
            Ok(shared)
        }
        pub fn CalculateSharedSecret(
            curve_algorithm: &Rc<ECDHCurveSpec>,
            private_key: &Rc<crate::software::amazon::cryptography::primitives::internaldafny::types::ECCPrivateKey>,
            public_key: &Rc<crate::software::amazon::cryptography::primitives::internaldafny::types::ECCPublicKey>,
        ) -> Rc<crate::Wrappers::Result<::dafny_runtime::Sequence<u8>, Rc<DafnyError>>> {
            let private_key: Vec<u8> = private_key.pem().iter().collect();
            let public_key: Vec<u8> = public_key.der().iter().collect();
            match agree(curve_algorithm, &private_key, &public_key) {
                Ok(v) => Rc::new(crate::Wrappers::Result::Success {
                    value: v.iter().cloned().collect(),
                }),
                Err(e) => {
                    let msg = format!("ECDH Calculate Shared Secret : {}", e);
                    Rc::new(crate::Wrappers::Result::Failure {
                        error: super::error(&msg),
                    })
                }
            }
        }
    }
    pub mod KeyGeneration {
        use crate::software::amazon::cryptography::primitives::internaldafny::types::ECDHCurveSpec;
        use crate::software::amazon::cryptography::primitives::internaldafny::types::Error as DafnyError;
        use aws_lc_rs::encoding::AsDer;
        use aws_lc_rs::encoding::EcPrivateKeyRfc5915Der;
        use std::rc::Rc;

        fn ecdsa_key_gen(alg: &ECDHCurveSpec) -> Result<(Vec<u8>, Vec<u8>), String> {
            let private_key =
                aws_lc_rs::agreement::PrivateKey::generate(super::ECCUtils::get_alg(alg))
                    .map_err(|e| format!("{:?}", e))?;

            let public_key = private_key
                .compute_public_key()
                .map_err(|e| format!("{:?}", e))?;
            let public_key: Vec<u8> = super::ECCUtils::sec1_compress(public_key.as_ref(), alg)?;

            let private_key_der = AsDer::<EcPrivateKeyRfc5915Der>::as_der(&private_key)
                .map_err(|e| format!("{:?}", e))?;
            let private_key = pem::Pem::new("PRIVATE KEY", private_key_der.as_ref());
            let private_key = pem::encode(&private_key);
            let private_key: Vec<u8> = private_key.into_bytes();

            Ok((public_key, private_key))
        }

        pub fn GenerateKeyPair(
            s: &Rc<ECDHCurveSpec>,
        ) -> Rc<crate::Wrappers::Result<Rc<crate::ECDH::EccKeyPair>, Rc<DafnyError>>> {
            match ecdsa_key_gen(s) {
                Ok(x) => Rc::new(crate::Wrappers::Result::Success {
                    value: Rc::new(crate::ECDH::EccKeyPair::EccKeyPair {
                        publicKey: x.0.iter().cloned().collect(),
                        privateKey: x.1.iter().cloned().collect(),
                    }),
                }),
                Err(e) => {
                    let msg = format!("ECDH Generate Key Pair : {}", e);
                    Rc::new(crate::Wrappers::Result::Failure {
                        error: super::error(&msg),
                    })
                }
            }
        }
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::software::amazon::cryptography::primitives::internaldafny::types::ECDHCurveSpec;
        use std::rc::Rc;
        #[test]
        fn test_generate() {
            let alg = Rc::new(ECDHCurveSpec::ECC_NIST_P256 {});

            let pair: crate::ECDH::EccKeyPair = match &*KeyGeneration::GenerateKeyPair(&alg) {
                crate::Wrappers::Result::Success { value } => (**value).clone(),
                crate::Wrappers::Result::Failure { error } => panic!("{:?}", error),
            };

            match &*ECCUtils::ValidatePublicKey(&alg, pair.publicKey()) {
                crate::Wrappers::Result::Success { .. } => {}
                crate::Wrappers::Result::Failure { error } => panic!("{:?}", error),
            };
        }
    }
}
