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
        use mpl_standard_library::_Wrappers_Compile as Wrappers;
        use aws_lc_rs::agreement::PrivateKey;
        use aws_lc_sys;
        // use aws_lc_rs::encoding::AsDer;
        // use aws_lc_rs::encoding::PublicKeyX509Der;
        use std::rc::Rc;

        fn get_nid(x: &ECDHCurveSpec) -> i32 {
            match x {
                ECDHCurveSpec::ECC_NIST_P256 {} => aws_lc_sys::NID_X9_62_prime256v1,
                ECDHCurveSpec::ECC_NIST_P384 {} => aws_lc_sys::NID_secp384r1,
                ECDHCurveSpec::ECC_NIST_P521 {} => aws_lc_sys::NID_secp521r1,
                ECDHCurveSpec::SM2 {} => panic!("No SM2 in Rust"),
            }
        }
        // NID_secp224r1 (NIST P-224),
        // NID_secp256k1 (SEC/ANSI P-256 K1)
        
        
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
                ECDHCurveSpec::ECC_NIST_P521 {} => &aws_lc_rs::agreement::ECDH_P521,
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
            // let public_key = sec1_decompress(public_key.as_ref(), alg).map_err(|e| format!("{:?}", e))?;
            // Ok(public_key)
            println!("Public Key 1 : {:?}", public_key.as_ref());
            // let public_key_der = AsDer::<PublicKeyX509Der>::as_der(&public_key)
            //     .map_err(|e| format!("{:?}", e))?;
            // println!("Public Key 2 : {:?}", public_key_der);

            Ok(public_key.as_ref().to_vec())
        }

        fn get_out_of_bounds(curve : &ECDHCurveSpec) -> Vec<u8>
        {
            match curve {
                ECDHCurveSpec::ECC_NIST_P256{} => vec![48, 89, 48, 19, 6, 7, 42, 134, 72, 206, 61, 2, 1, 6, 8, 42, 134, 72, 206, 61, 3, 1, 7, 3, 66, 0, 4, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255],
                ECDHCurveSpec::ECC_NIST_P384{} => vec![48, 118, 48, 16, 6, 7, 42, 134, 72, 206, 61, 2, 1, 6, 5, 43, 129, 4, 0, 34, 3, 98, 0, 4, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255],
                ECDHCurveSpec::ECC_NIST_P521{} => vec![48, 129, 155, 48, 16, 6, 7, 42, 134, 72, 206, 61, 2, 1, 6, 5, 43, 129, 4, 0, 35, 3, 129, 134, 0, 4, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255],
                ECDHCurveSpec::SM2{} => vec![],
            }
        }
        pub fn GetOutOfBoundsPublicKey(curve_algorithm: &Rc<ECDHCurveSpec>) ->
            Rc<Wrappers::Result<::dafny_runtime::Sequence<u8>, Rc<DafnyError>>> {
                let result = get_out_of_bounds(curve_algorithm);
                Rc::new(Wrappers::Result::Success {
                    value : result.iter().cloned().collect()
                })
            }

        fn get_infinity(curve : &ECDHCurveSpec) -> Vec<u8>
        {
            match curve {
                ECDHCurveSpec::ECC_NIST_P256{} => vec![48, 25, 48, 19, 6, 7, 42, 134, 72, 206, 61, 2, 1, 6, 8, 42, 134, 72, 206, 61, 3, 1, 7, 3, 2, 0, 0],
                ECDHCurveSpec::ECC_NIST_P384{} => vec![48, 22, 48, 16, 6, 7, 42, 134, 72, 206, 61, 2, 1, 6, 5, 43, 129, 4, 0, 34, 3, 2, 0, 0],
                ECDHCurveSpec::ECC_NIST_P521{} => vec![48, 22, 48, 16, 6, 7, 42, 134, 72, 206, 61, 2, 1, 6, 5, 43, 129, 4, 0, 35, 3, 2, 0, 0],
                ECDHCurveSpec::SM2{} => vec![],
            }
        }

        pub fn GetInfinityPublicKey(curve_algorithm: &Rc<ECDHCurveSpec>) ->
            Rc<Wrappers::Result<::dafny_runtime::Sequence<u8>, Rc<DafnyError>>> {
                let result = get_infinity(curve_algorithm);
                Rc::new(Wrappers::Result::Success {
                    value : result.iter().cloned().collect()
                })
            }
        pub fn GetPublicKey(
            curve_algorithm: &Rc<ECDHCurveSpec>,
            private_key: &Rc<crate::software::amazon::cryptography::primitives::internaldafny::types::ECCPrivateKey>,
        ) -> Rc<Wrappers::Result<::dafny_runtime::Sequence<u8>, Rc<DafnyError>>> {
            let private_key: Vec<u8> = private_key.pem().iter().collect();
            match get_public_key(curve_algorithm, &private_key) {
                Ok(x) => Rc::new(Wrappers::Result::Success {
                    value: x.iter().cloned().collect(),
                }),
                Err(e) => {
                    let msg = format!("ECDH Get Public Key : {}", e);
                    Rc::new(Wrappers::Result::Failure {
                        error: super::error(&msg),
                    })
                }
            }
        }

        // for the moment, it's valid if we can use it to generate a shared secret
        fn valid_public_key(alg: &ECDHCurveSpec, public_key: &[u8]) -> Result<(), String> {
            let public_key = sec1_decompress(public_key, alg).map_err(|e| format!("{:?}", e))?;
            let private_key = aws_lc_rs::agreement::PrivateKey::generate(get_alg(alg))
                .map_err(|e| format!("{:?}", e))?;
            let public_key = aws_lc_rs::agreement::UnparsedPublicKey::new(get_alg(alg), &public_key);
            match aws_lc_rs::agreement::agree(&private_key, &public_key, "foo", |_x| Ok(false)) {
                Ok(_) => Ok(()),
                Err(_) => Err("Invalid ECDH Public Key".to_string()),
            }
        }

        pub fn ValidatePublicKey(
            curve_algorithm: &Rc<ECDHCurveSpec>,
            public_key: &::dafny_runtime::Sequence<u8>,
        ) -> Rc<Wrappers::Result<bool, Rc<DafnyError>>> {
            let public_key: Vec<u8> = public_key.iter().collect();
            match valid_public_key(curve_algorithm, &public_key) {
                Ok(_) => Rc::new(Wrappers::Result::Success { value: true }),
                Err(e) => {
                    let msg = format!("ECDH Validate Public Key : {}", e);
                    Rc::new(Wrappers::Result::Failure {
                        error: super::error(&msg),
                    })
                }
            }
        }
        fn PrintNid(x : i32)
        {
            println!("{} {} {}", x, x/256, x%256);
        }
        fn PrintNids()
        {
            PrintNid(aws_lc_sys::NID_X9_62_prime256v1);
            PrintNid(aws_lc_sys::NID_secp384r1);
            PrintNid(aws_lc_sys::NID_secp521r1);
        }
        pub fn CompressPublicKey(
            public_key: &::dafny_runtime::Sequence<u8>,
            curve_algorithm: &Rc<ECDHCurveSpec>,
        ) -> Rc<Wrappers::Result<::dafny_runtime::Sequence<u8>, Rc<DafnyError>>> {
            let public_key: Vec<u8> = public_key.iter().collect();
            let tmp = sec1_compress(&public_key, curve_algorithm).unwrap();
            let tmp2 = sec1_decompress(&public_key, curve_algorithm).unwrap();
            println!("Compress {:?} from {} to {} {}", curve_algorithm, public_key.len(), tmp.len(), tmp2.len());
            PrintNids();
            println!("{:?}", public_key);
            match sec1_compress(&public_key, curve_algorithm) {
                Ok(v) => Rc::new(Wrappers::Result::Success {
                    value: v.iter().cloned().collect(),
                }),
                Err(e) => {
                    let msg = format!("ECDH Compress Public Key {}", e);
                    Rc::new(Wrappers::Result::Failure {
                        error: super::error(&msg),
                    })
                }
            }
        }
        pub fn DecompressPublicKey(
            public_key: &::dafny_runtime::Sequence<u8>,
            curve_algorithm: &Rc<ECDHCurveSpec>,
        ) -> Rc<Wrappers::Result<::dafny_runtime::Sequence<u8>, Rc<DafnyError>>> {
            let public_key: Vec<u8> = public_key.iter().collect();
            let tmp2 = sec1_compress(&public_key, curve_algorithm).unwrap();
            let tmp = sec1_decompress(&public_key, curve_algorithm).unwrap();
            println!("Decompress {:?} from {} to {} {}", curve_algorithm, public_key.len(), tmp.len(), tmp2.len());
            match sec1_decompress(&public_key, curve_algorithm) {
                Ok(v) => Rc::new(Wrappers::Result::Success {
                    value: v.iter().cloned().collect(),
                }),
                Err(e) => {
                    let msg = format!("ECDH Decompress Public Key {}", e);
                    Rc::new(Wrappers::Result::Failure {
                        error: super::error(&msg),
                    })
                }
            }
        }
        pub fn ParsePublicKey(
            public_key: &::dafny_runtime::Sequence<u8>,
        ) -> Rc<Wrappers::Result<::dafny_runtime::Sequence<u8>, Rc<DafnyError>>> {
            Rc::new(Wrappers::Result::Success {
                value: public_key.clone(),
            })
            // let public_key: Vec<u8> = publicKey.iter().collect();
            // match aws_lc_rs::rsa::PublicEncryptingKey::from_der(&public_key) {
            //     Ok(_) => Rc::new(Wrappers::Result::Success {
            //         value: publicKey.clone(),
            //     }),
            //     Err(e) => {
            //         let msg = format!("{}", e);
            //         Rc::new(Wrappers::Result::Failure {
            //             error: super::error(&msg),
            //         })
            //     }
            // }
        }
    }
    pub mod DeriveSharedSecret {
        use crate::software::amazon::cryptography::primitives::internaldafny::types::ECDHCurveSpec;
        use crate::software::amazon::cryptography::primitives::internaldafny::types::Error as DafnyError;
        use mpl_standard_library::_Wrappers_Compile as Wrappers;
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
        ) -> Rc<Wrappers::Result<::dafny_runtime::Sequence<u8>, Rc<DafnyError>>> {
            let private_key: Vec<u8> = private_key.pem().iter().collect();
            let public_key: Vec<u8> = public_key.der().iter().collect();
            match agree(curve_algorithm, &private_key, &public_key) {
                Ok(v) => Rc::new(Wrappers::Result::Success {
                    value: v.iter().cloned().collect(),
                }),
                Err(e) => {
                    let msg = format!("ECDH Calculate Shared Secret : {}", e);
                    Rc::new(Wrappers::Result::Failure {
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
        use mpl_standard_library::_Wrappers_Compile as Wrappers;
        use std::rc::Rc;

        fn ecdsa_key_gen(alg: &ECDHCurveSpec) -> Result<(Vec<u8>, Vec<u8>), String> {
            let private_key =
                aws_lc_rs::agreement::PrivateKey::generate(super::ECCUtils::get_alg(alg))
                    .map_err(|e| format!("{:?}", e))?;

            let public_key = private_key
                .compute_public_key()
                .map_err(|e| format!("{:?}", e))?;
            let public_key: Vec<u8> = public_key.as_ref().to_vec();
            let private_key_der = AsDer::<EcPrivateKeyRfc5915Der>::as_der(&private_key)
                .map_err(|e| format!("{:?}", e))?;
            let private_key = pem::Pem::new("PRIVATE KEY", private_key_der.as_ref());
            let private_key = pem::encode(&private_key);
            let private_key: Vec<u8> = private_key.into_bytes();

            Ok((public_key, private_key))
        }

        pub fn GenerateKeyPair(
            s: &Rc<ECDHCurveSpec>,
        ) -> Rc<Wrappers::Result<Rc<crate::ECDH::EccKeyPair>, Rc<DafnyError>>> {
            match ecdsa_key_gen(s) {
                Ok(x) => Rc::new(Wrappers::Result::Success {
                    value: Rc::new(crate::ECDH::EccKeyPair::EccKeyPair {
                        publicKey: x.0.iter().cloned().collect(),
                        privateKey: x.1.iter().cloned().collect(),
                    }),
                }),
                Err(e) => {
                    let msg = format!("ECDH Generate Key Pair : {}", e);
                    Rc::new(Wrappers::Result::Failure {
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
        use mpl_standard_library::_Wrappers_Compile as Wrappers;
        use std::rc::Rc;
        #[test]
        fn test_generate() {
            let alg = Rc::new(ECDHCurveSpec::ECC_NIST_P256 {});

            let pair: crate::ECDH::EccKeyPair = match &*KeyGeneration::GenerateKeyPair(&alg) {
                Wrappers::Result::Success { value } => (**value).clone(),
                Wrappers::Result::Failure { error } => panic!("{:?}", error),
            };

            match &*ECCUtils::ValidatePublicKey(&alg, pair.publicKey()) {
                Wrappers::Result::Success { .. } => {}
                Wrappers::Result::Failure { error } => panic!("{:?}", error),
            };
            println!("\n\nSUCCESS IN VALIDATE\n\n");
        }
    }
}
