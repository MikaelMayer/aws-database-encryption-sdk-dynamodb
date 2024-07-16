// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![allow(warnings, unconditional_panic)]
#![allow(nonstandard_style)]

pub mod Signature {
    pub mod ECDSA {
        use crate::*;
        pub use crate::software::amazon::cryptography::materialproviders::internaldafny::types::ECDSA::*;
        use crate::software::amazon::cryptography::primitives::internaldafny::types::ECDSASignatureAlgorithm;
        use crate::software::amazon::cryptography::primitives::internaldafny::types::Error as DafnyError;
        use ::std::rc::Rc;
        use aws_lc_rs::signature::EcdsaKeyPair;
        use aws_lc_rs::signature::EcdsaSigningAlgorithm;
        use aws_lc_rs::signature::EcdsaVerificationAlgorithm;
        use aws_lc_rs::encoding::AsBigEndian;
        use aws_lc_rs::signature::KeyPair;
        use aws_lc_rs::encoding::AsDer;
        use aws_lc_rs::rand::SystemRandom;
        use aws_lc_rs::signature::UnparsedPublicKey;
        use ptr::LcPtr;

        fn error(s: &str) -> Rc<DafnyError> {
            Rc::new(DafnyError::AwsCryptographicPrimitivesError {
                message:
                    dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(s),
            })
        }

        fn get_alg(x: &ECDSASignatureAlgorithm) -> &'static EcdsaSigningAlgorithm {
            match x {
                ECDSASignatureAlgorithm::ECDSA_P256 {} => {
                    &aws_lc_rs::signature::ECDSA_P256_SHA256_FIXED_SIGNING
                }
                ECDSASignatureAlgorithm::ECDSA_P384 {} => {
                    &aws_lc_rs::signature::ECDSA_P384_SHA384_FIXED_SIGNING
                }
            }
        }

        fn get_ver_alg(x: &ECDSASignatureAlgorithm) -> &'static EcdsaVerificationAlgorithm {
            match x {
                ECDSASignatureAlgorithm::ECDSA_P256 {} => {
                    &aws_lc_rs::signature::ECDSA_P256_SHA256_FIXED
                }
                ECDSASignatureAlgorithm::ECDSA_P384 {} => {
                    &aws_lc_rs::signature::ECDSA_P384_SHA384_FIXED
                }
            }
        }

        fn get_nid(x: &ECDSASignatureAlgorithm) -> i32 {
            match x {
                ECDSASignatureAlgorithm::ECDSA_P256 {} => aws_lc_sys::NID_X9_62_prime256v1,
                ECDSASignatureAlgorithm::ECDSA_P384 {} => aws_lc_sys::NID_secp384r1,
            }
        }

        const ELEM_MAX_BITS: usize = 521;
        const ELEM_MAX_BYTES: usize = (ELEM_MAX_BITS + 7) / 8;
        const SCALAR_MAX_BYTES: usize = ELEM_MAX_BYTES;
        const PUBLIC_KEY_MAX_LEN: usize = 1 + (2 * ELEM_MAX_BYTES);

        fn sec1_compress(data: &[u8], alg: &ECDSASignatureAlgorithm) -> Result<Vec<u8>, String> {
            use aws_lc_sys::point_conversion_form_t;
            use aws_lc_sys::EC_GROUP_new_by_curve_name;
            use aws_lc_sys::EC_POINT_new;
            use aws_lc_sys::EC_POINT_oct2point;
            use aws_lc_sys::EC_POINT_point2oct;
            use std::ptr::null_mut;

            let nid = get_nid(alg);
            let ec_group = LcPtr::new(unsafe { EC_GROUP_new_by_curve_name(nid) })
                .map_err(|e| format!("{:?}", e))?;

            let form = point_conversion_form_t::POINT_CONVERSION_COMPRESSED;
            let ec_point =
                LcPtr::new(unsafe { EC_POINT_new(*ec_group) }).map_err(|e| format!("{:?}", e))?;
            let mut out_buf = [0u8; PUBLIC_KEY_MAX_LEN];

            unsafe {
                EC_POINT_oct2point(*ec_group, *ec_point, data.as_ptr(), data.len(), null_mut());
            }
            unsafe {
                EC_POINT_point2oct(
                    *ec_group,
                    *ec_point,
                    form,
                    out_buf.as_mut_ptr(),
                    PUBLIC_KEY_MAX_LEN,
                    null_mut(),
                );
            }
            Ok(data.iter().cloned().collect())
        }

        fn ecdsa_key_gen(alg: &ECDSASignatureAlgorithm) -> Result<(Vec<u8>, Vec<u8>), String> {
            let pair = EcdsaKeyPair::generate(get_alg(alg)).map_err(|e| format!("{:?}", e))?;
            let public_key: Vec<u8> = sec1_compress(pair.public_key().as_ref(), alg)?;
            let private_key: Vec<u8> = pair
                .private_key()
                .as_der()
                .unwrap()
                .as_ref()
                .iter()
                .cloned()
                .collect();
            Ok((public_key, private_key))
        }

        pub fn ExternKeyGen(
            alg: &Rc<ECDSASignatureAlgorithm>,
        ) -> Rc<Wrappers::Result<Rc<Signature::SignatureKeyPair>, Rc<DafnyError>>> {
            match ecdsa_key_gen(&**alg) {
                Ok(x) => Rc::new(Wrappers::Result::Success {
                    value: Rc::new(Signature::SignatureKeyPair::SignatureKeyPair {
                        verificationKey: x.0.iter().cloned().collect(),
                        signingKey: x.1.iter().cloned().collect(),
                    }),
                }),
                Err(e) => {
                    let msg = format!("{}", e);
                    Rc::new(Wrappers::Result::Failure { error: error(&msg) })
                }
            }
        }

        fn ecdsa_sign(
            alg: &ECDSASignatureAlgorithm,
            key: &[u8],
            msg: &[u8],
        ) -> Result<Vec<u8>, String> {
            let private_key = EcdsaKeyPair::from_private_key_der(get_alg(alg), key)
                .map_err(|e| format!("{:?}", e))?;
            let rng = SystemRandom::new();
            let sig = private_key
                .sign(&rng, msg)
                .map_err(|e| format!("{:?}", e))?;
            Ok(sig.as_ref().iter().cloned().collect())
        }

        pub fn Sign(
            alg: &Rc<ECDSASignatureAlgorithm>,
            key: &::dafny_runtime::Sequence<u8>,
            msg: &::dafny_runtime::Sequence<u8>,
        ) -> Rc<Wrappers::Result<::dafny_runtime::Sequence<u8>, Rc<DafnyError>>> {
            let key: Vec<u8> = key.iter().collect();
            let msg: Vec<u8> = msg.iter().collect();
            match ecdsa_sign(&**alg, &key, &msg) {
                Ok(x) => Rc::new(Wrappers::Result::Success {
                    value: x.iter().cloned().collect(),
                }),
                Err(e) => {
                    let msg = format!("{}", e);
                    Rc::new(Wrappers::Result::Failure { error: error(&msg) })
                }
            }
        }

        fn ecdsa_verify(
            alg: &ECDSASignatureAlgorithm,
            key: &[u8],
            msg: &[u8],
            sig: &[u8],
        ) -> Result<bool, String> {
            let public_key = UnparsedPublicKey::new(get_ver_alg(alg), key);
            public_key
                .verify(msg, &sig)
                .map_err(|e| format!("{:?}", e))?;
            println!("Verify Succeeded.");
            Ok(true)
        }

        pub fn Verify(
            alg: &Rc<ECDSASignatureAlgorithm>,
            key: &::dafny_runtime::Sequence<u8>,
            msg: &::dafny_runtime::Sequence<u8>,
            sig: &::dafny_runtime::Sequence<u8>,
        ) -> Rc<Wrappers::Result<bool, Rc<DafnyError>>> {
            let key: Vec<u8> = key.iter().collect();
            let msg: Vec<u8> = msg.iter().collect();
            let sig: Vec<u8> = sig.iter().collect();
            match ecdsa_verify(&**alg, &key, &msg, &sig) {
                Ok(x) => Rc::new(Wrappers::Result::Success { value: x }),
                Err(e) => {
                    let msg = format!("{}", e);
                    Rc::new(Wrappers::Result::Failure { error: error(&msg) })
                }
            }
        }
        #[cfg(test)]
        mod tests {
            use super::*;
            use std::rc::Rc;
            #[test]
            fn test_generate() {
                let alg = Rc::new(ECDSASignatureAlgorithm::ECDSA_P384 {});
                let key_pair = match &*ExternKeyGen(&alg) {
                    Wrappers::Result::Success { value } => value.clone(),
                    Wrappers::Result::Failure { error } => {
                        panic!("ExternKeyGen Failed : {:?}", error);
                    }
                };

                let (s_key, v_key) = match &*key_pair {
                    Signature::SignatureKeyPair::SignatureKeyPair {
                        signingKey,
                        verificationKey,
                    } => (signingKey, verificationKey),
                };

                let message: ::dafny_runtime::Sequence<u8> =
                    [1u8, 2, 3, 4, 5].iter().cloned().collect();

                let sig = match &*Sign(&alg, &s_key, &message) {
                    Wrappers::Result::Success { value } => value.clone(),
                    Wrappers::Result::Failure { error } => {
                        panic!("Sign Failed : {:?}", error);
                    }
                };

                let ver: bool = match &*Verify(&alg, &v_key, &message, &sig) {
                    Wrappers::Result::Success { value } => value.clone(),
                    Wrappers::Result::Failure { error } => {
                        panic!("Verify Failed : {:?}", error);
                    }
                };
                assert!(ver);

                let mut sig_vec: Vec<u8> = sig.iter().collect();
                sig_vec[0] = 42;
                let sig2: ::dafny_runtime::Sequence<u8> = sig_vec.iter().cloned().collect();
                let ver2: bool = match &*Verify(&alg, &v_key, &message, &sig2) {
                    Wrappers::Result::Success { value } => {
                        panic!("Verify Should have failed");
                    }
                    Wrappers::Result::Failure { error } => false,
                };
                assert!(!ver2);

                // let (public_key, private_key) = GenerateKeyPairExtern(2048);

                // // let modulus = GetRSAKeyModulusLengthExtern(&public_key);
                // // println!("{:?}", modulus);
                // // let modulus = modulus.UnwrapOr(&42);
                // // assert_eq!(modulus, 2048);

                // let mode = RSAPaddingMode::OAEP_SHA256 {};
                // let plain_text: ::dafny_runtime::Sequence<u8> =
                //     [1u8, 2, 3, 4, 5].iter().cloned().collect();
                // let empty: ::dafny_runtime::Sequence<u8> = [].iter().cloned().collect();
                // let cipher_text = DecryptExtern(&mode, &private_key, &plain_text); //.UnwrapOr(&empty);
                // println!("{:?}", cipher_text);
            }
        }
    }
}
