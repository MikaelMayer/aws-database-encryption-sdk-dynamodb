// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![allow(warnings, unconditional_panic)]
#![allow(nonstandard_style)]
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
            Ok(public_key.as_ref().iter().cloned().collect())
        }

        pub fn GetPublicKey(
            curveAlgorithm: &Rc<ECDHCurveSpec>,
            private_key: &Rc<crate::software::amazon::cryptography::primitives::internaldafny::types::ECCPrivateKey>,
        ) -> Rc<
            crate::Wrappers::Result<
                ::dafny_runtime::Sequence<u8>,
                Rc<crate::software::amazon::cryptography::primitives::internaldafny::types::Error>,
            >,
        > {
            let private_key: Vec<u8> = private_key.pem().iter().collect();
            match get_public_key(&**curveAlgorithm, &private_key) {
                Ok(x) => Rc::new(crate::Wrappers::Result::Success {
                    value: x.iter().cloned().collect(),
                }),
                Err(e) => {
                    let msg = format!("{}", e);
                    Rc::new(crate::Wrappers::Result::Failure {
                        error: super::error(&msg),
                    })
                }
            }
        }

        fn valid_public_key(alg: &ECDHCurveSpec, public_key: &[u8]) -> Result<(), String> {
            let public_key = aws_lc_rs::agreement::UnparsedPublicKey::new(get_alg(alg), public_key);
            // then a miracle occurs
            Ok(())
        }

        pub fn ValidatePublicKey(
            curveAlgorithm: &Rc<ECDHCurveSpec>,
            public_key: &::dafny_runtime::Sequence<u8>,
        ) -> Rc<
            crate::Wrappers::Result<
                bool,
                Rc<crate::software::amazon::cryptography::primitives::internaldafny::types::Error>,
            >,
        > {
            let public_key: Vec<u8> = public_key.iter().collect();
            match valid_public_key(&**curveAlgorithm, &public_key) {
                Ok(x) => Rc::new(crate::Wrappers::Result::Success { value: true }),
                Err(e) => {
                    let msg = format!("{}", e);
                    Rc::new(crate::Wrappers::Result::Failure {
                        error: super::error(&msg),
                    })
                }
            }
        }
        pub fn CompressPublicKey(
            public_key: &::dafny_runtime::Sequence<u8>,
            curveAlgorithm: &Rc<ECDHCurveSpec>,
        ) -> Rc<
            crate::Wrappers::Result<
                ::dafny_runtime::Sequence<u8>,
                Rc<crate::software::amazon::cryptography::primitives::internaldafny::types::Error>,
            >,
        > {
            let public_key: Vec<u8> = public_key.iter().collect();
            match sec1_compress(&public_key, &*curveAlgorithm) {
                Ok(v) => Rc::new(crate::Wrappers::Result::Success {
                    value: v.iter().cloned().collect(),
                }),
                Err(e) => {
                    let msg = format!("{}", e);
                    Rc::new(crate::Wrappers::Result::Failure {
                        error: super::error(&msg),
                    })
                }
            }
        }
        pub fn DecompressPublicKey(
            public_key: &::dafny_runtime::Sequence<u8>,
            curveAlgorithm: &Rc<ECDHCurveSpec>,
        ) -> Rc<
            crate::Wrappers::Result<
                ::dafny_runtime::Sequence<u8>,
                Rc<crate::software::amazon::cryptography::primitives::internaldafny::types::Error>,
            >,
        > {
            let public_key: Vec<u8> = public_key.iter().collect();
            match sec1_decompress(&public_key, &*curveAlgorithm) {
                Ok(v) => Rc::new(crate::Wrappers::Result::Success {
                    value: v.iter().cloned().collect(),
                }),
                Err(e) => {
                    let msg = format!("{}", e);
                    Rc::new(crate::Wrappers::Result::Failure {
                        error: super::error(&msg),
                    })
                }
            }
        }
        pub fn ParsePublicKey(
            publicKey: &::dafny_runtime::Sequence<u8>,
        ) -> Rc<
            crate::Wrappers::Result<
                ::dafny_runtime::Sequence<u8>,
                Rc<crate::software::amazon::cryptography::primitives::internaldafny::types::Error>,
            >,
        > {
            let public_key: Vec<u8> = publicKey.iter().collect();
            match aws_lc_rs::rsa::PublicEncryptingKey::from_der(&public_key) {
                Ok(_) => Rc::new(crate::Wrappers::Result::Success {
                    value: publicKey.clone(),
                }),
                Err(e) => {
                    let msg = format!("{}", e);
                    Rc::new(crate::Wrappers::Result::Failure {
                        error: super::error(&msg),
                    })
                }
            }
        }
    }
    pub mod DeriveSharedSecret {
        use crate::software::amazon::cryptography::primitives::internaldafny::types::ECDHCurveSpec;
        use std::rc::Rc;

        pub fn agree(
            curveAlgorithm: &ECDHCurveSpec,
            private_key_pem: &[u8],
            public_key_der: &[u8],
        ) -> Result<Vec<u8>, String> {
            let pem = std::str::from_utf8(private_key_pem).map_err(|e| format!("{:?}", e))?;
            let private_key = pem::parse(pem).map_err(|e| format!("{:?}", e))?;
            let private_key = aws_lc_rs::agreement::PrivateKey::from_private_key_der(
                super::ECCUtils::get_alg(curveAlgorithm),
                private_key.contents(),
            )
            .map_err(|e| format!("{:?}", e))?;
            let public_key = aws_lc_rs::agreement::UnparsedPublicKey::new(
                super::ECCUtils::get_alg(curveAlgorithm),
                public_key_der,
            );
            let shared: Vec<u8> =
                aws_lc_rs::agreement::agree(&private_key, &public_key, "foo", |x| {
                    Ok(x.iter().cloned().collect())
                })
                .map_err(|e| format!("Failure in aws_lc_rs::agreement::agree."))?;
            Ok(shared)
        }
        pub fn CalculateSharedSecret(
            curveAlgorithm: &Rc<ECDHCurveSpec>,
            private_key: &Rc<crate::software::amazon::cryptography::primitives::internaldafny::types::ECCPrivateKey>,
            public_key: &Rc<crate::software::amazon::cryptography::primitives::internaldafny::types::ECCPublicKey>,
        ) -> Rc<
            crate::Wrappers::Result<
                ::dafny_runtime::Sequence<u8>,
                Rc<crate::software::amazon::cryptography::primitives::internaldafny::types::Error>,
            >,
        > {
            let private_key: Vec<u8> = private_key.pem().iter().collect();
            let public_key: Vec<u8> = public_key.der().iter().collect();
            match agree(&*curveAlgorithm, &private_key, &public_key) {
                Ok(v) => Rc::new(crate::Wrappers::Result::Success {
                    value: v.iter().cloned().collect(),
                }),
                Err(e) => {
                    let msg = format!("{}", e);
                    Rc::new(crate::Wrappers::Result::Failure {
                        error: super::error(&msg),
                    })
                }
            }
        }
    }
    pub mod KeyGeneration {
        use crate::software::amazon::cryptography::primitives::internaldafny::types::ECDHCurveSpec;
        use aws_lc_rs::encoding::AsDer;
        use aws_lc_rs::encoding::EcPrivateKeyRfc5915Der;
        use aws_lc_rs::encoding::PublicKeyX509Der;
        use aws_lc_rs::signature::EcdsaKeyPair;
        use aws_lc_rs::signature::KeyPair;
        use std::rc::Rc;

        fn get_alg(x: &ECDHCurveSpec) -> &'static aws_lc_rs::signature::EcdsaSigningAlgorithm {
            match x {
                ECDHCurveSpec::ECC_NIST_P256 {} => {
                    &aws_lc_rs::signature::ECDSA_P256K1_SHA256_FIXED_SIGNING
                }
                ECDHCurveSpec::ECC_NIST_P384 {} => {
                    &aws_lc_rs::signature::ECDSA_P384_SHA384_FIXED_SIGNING
                }
                ECDHCurveSpec::ECC_NIST_P521 {} => {
                    &aws_lc_rs::signature::ECDSA_P521_SHA512_FIXED_SIGNING
                }
                ECDHCurveSpec::SM2 {} => panic!("No SM2 in Rust"),
            }
        }

        fn ecdsa_key_gen(alg: &ECDHCurveSpec) -> Result<(Vec<u8>, Vec<u8>), String> {
            let pair = EcdsaKeyPair::generate(get_alg(alg)).map_err(|e| format!("{:?}", e))?;
            let public_key_der = AsDer::<PublicKeyX509Der>::as_der(pair.public_key())
                .map_err(|e| format!("{:?}", e))?;
            let public_key: Vec<u8> = super::ECCUtils::sec1_compress(public_key_der.as_ref(), alg)?;

            let private_key_der = AsDer::<EcPrivateKeyRfc5915Der>::as_der(&pair.private_key())
                .map_err(|e| format!("{:?}", e))?;
            let private_key = pem::Pem::new("PRIVATE KEY", private_key_der.as_ref());
            let private_key = pem::encode(&private_key);
            let private_key: Vec<u8> = private_key.into_bytes();
            Ok((public_key, private_key))
        }

        pub fn GenerateKeyPair(
            s: &Rc<ECDHCurveSpec>,
        ) -> Rc<
            crate::Wrappers::Result<
                Rc<crate::ECDH::EccKeyPair>,
                Rc<crate::software::amazon::cryptography::primitives::internaldafny::types::Error>,
            >,
        > {
            match ecdsa_key_gen(&**s) {
                Ok(x) => Rc::new(crate::Wrappers::Result::Success {
                    value: Rc::new(crate::ECDH::EccKeyPair::EccKeyPair {
                        publicKey: x.0.iter().cloned().collect(),
                        privateKey: x.1.iter().cloned().collect(),
                    }),
                }),
                Err(e) => {
                    let msg = format!("{}", e);
                    Rc::new(crate::Wrappers::Result::Failure {
                        error: super::error(&msg),
                    })
                }
            }
        }
    }
}
