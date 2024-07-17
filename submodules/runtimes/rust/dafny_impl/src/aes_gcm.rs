// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![allow(warnings, unconditional_panic)]
#![allow(nonstandard_style)]
use crate::software::amazon::cryptography::primitives::internaldafny::types::AESEncryptOutput;
use crate::software::amazon::cryptography::primitives::internaldafny::types::Error as DafnyError;

use crate::software::amazon::cryptography::primitives::internaldafny::types::AES_GCM;
use crate::*;
use ::std::rc::Rc;
use aws_lc_rs::aead::{Aad, LessSafeKey, Nonce, UnboundKey, AES_256_GCM};
use aws_lc_rs::error::Unspecified;

struct DoAESEncryptOutput {
    cipherText: Vec<u8>,
    authTag: Vec<u8>,
}

fn error(s: &str) -> Rc<DafnyError> {
    Rc::new(DafnyError::AwsCryptographicPrimitivesError {
        message:
            dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(s),
    })
}

fn enc_result(s: &str) -> Rc<Wrappers::Result<Rc<AESEncryptOutput>, Rc<DafnyError>>> {
    Rc::new(Wrappers::Result::Failure { error: error(s) })
}

fn dec_result(s: &str) -> Rc<Wrappers::Result<::dafny_runtime::Sequence<u8>, Rc<DafnyError>>> {
    Rc::new(Wrappers::Result::Failure { error: error(s) })
}
pub mod AESEncryption {
    pub use crate::software::amazon::cryptography::primitives::internaldafny::types::*;
}
impl AES_GCM {
    #[allow(non_snake_case)]

    fn get_alg(&self) -> Result<&'static aws_lc_rs::aead::Algorithm, String> {
        if *self.tagLength() != 16i32 {
            Err(format!(
                "Tag length of {} not supported in Rust. Tag length must be 16.",
                self.tagLength()
            ))
        } else if *self.ivLength() != 12i32 {
            Err(format!(
                "IV length of {} not supported in Rust. Tag length must be 16.",
                self.ivLength()
            ))
        } else if *self.keyLength() == 32i32 {
            Ok(&aws_lc_rs::aead::AES_256_GCM)
        } else if *self.keyLength() == 16i32 {
            Ok(&aws_lc_rs::aead::AES_128_GCM)
        } else {
            Err(format!(
                "Key length of {} not supported in Rust. Tag length must be 16 or 32.",
                self.keyLength()
            ))
        }
    }

    fn do_aes_encrypt(
        &self,
        iv: &[u8],
        key: &[u8],
        msg: &[u8],
        aad: &[u8],
    ) -> Result<DoAESEncryptOutput, String> {
        let alg = self.get_alg()?;
        let mut in_out_buffer = Vec::from(msg);
        let key = UnboundKey::new(alg, &key).map_err(|e| format!("new {:?}", e))?;
        let nonce = Nonce::assume_unique_for_key(iv.try_into().unwrap());
        let mut key = LessSafeKey::new(key);
        let aad = Aad::from(aad);
        let tag = key
            .seal_in_place_separate_tag(nonce, aad, &mut in_out_buffer)
            .map_err(|e| format!("Seal {:?}", e))?;
        Ok(DoAESEncryptOutput {
            cipherText: in_out_buffer,
            authTag: Vec::from(tag.as_ref()),
        })
    }

    fn do_aes_decrypt(
        &self,
        key: &[u8],
        cipherTxt: &[u8],
        authTag: &[u8],
        iv: &[u8],
        aad: &[u8],
    ) -> Result<Vec<u8>, String> {
        let alg = self.get_alg()?;
        let mut out_buffer = Vec::from(cipherTxt);
        let key = UnboundKey::new(alg, &key).map_err(|e| format!("new {:?}", e))?;
        let nonce = Nonce::assume_unique_for_key(iv.try_into().unwrap());
        let mut key = LessSafeKey::new(key);
        let aad = Aad::from(aad);
        key.open_separate_gather(nonce, aad, cipherTxt, authTag, &mut out_buffer)
            .map_err(|e| format!("gather {:?}", e))?;
        Ok(out_buffer)
    }

    pub fn AESEncryptExtern(
        &self,
        iv: &::dafny_runtime::Sequence<u8>,
        key: &::dafny_runtime::Sequence<u8>,
        msg: &::dafny_runtime::Sequence<u8>,
        aad: &::dafny_runtime::Sequence<u8>,
    ) -> Rc<Wrappers::Result<Rc<AESEncryptOutput>, Rc<DafnyError>>> {
        let iv: Vec<u8> = iv.iter().collect();
        let key: Vec<u8> = key.iter().collect();
        let msg: Vec<u8> = msg.iter().collect();
        let aad: Vec<u8> = aad.iter().collect();

        if *self.keyLength() as usize != key.len() {
            let msg = format!(
                "AESEncrypt : algorithm key length was {} but actual key length was {}.",
                self.keyLength(),
                key.len()
            );
            return enc_result(&msg);
        }
        if *self.ivLength() as usize != iv.len() {
            let msg = format!(
                "AESEncrypt : algorithm nonce length was {} but actual nonce length was {}.",
                self.ivLength(),
                iv.len()
            );
            return enc_result(&msg);
        }

        match self.do_aes_encrypt(&iv, &key, &msg, &aad) {
            Ok(x) => Rc::new(Wrappers::Result::Success {
                value: Rc::new(AESEncryptOutput::AESEncryptOutput {
                    cipherText: x.cipherText.iter().cloned().collect(),
                    authTag: x.authTag.iter().cloned().collect(),
                }),
            }),
            Err(e) => {
                let msg = format!("{}", e);
                enc_result(&msg)
            }
        }
    }

    #[allow(non_snake_case)]
    pub fn AESDecryptExtern(
        &self,
        key: &::dafny_runtime::Sequence<u8>,
        cipherTxt: &::dafny_runtime::Sequence<u8>,
        authTag: &::dafny_runtime::Sequence<u8>,
        iv: &::dafny_runtime::Sequence<u8>,
        aad: &::dafny_runtime::Sequence<u8>,
    ) -> Rc<Wrappers::Result<::dafny_runtime::Sequence<u8>, Rc<DafnyError>>> {
        let key: Vec<u8> = key.iter().collect();
        let cipherTxt: Vec<u8> = cipherTxt.iter().collect();
        let authTag: Vec<u8> = authTag.iter().collect();
        let iv: Vec<u8> = iv.iter().collect();
        let aad: Vec<u8> = aad.iter().collect();

        if *self.keyLength() as usize != key.len() {
            let msg = format!(
                "AESEncrypt : algorithm key length was {} but actual key length was {}.",
                self.keyLength(),
                key.len()
            );
            return dec_result(&msg);
        }

        if *self.ivLength() as usize != iv.len() {
            let msg = format!(
                "AESEncrypt : algorithm nonce length was {} but actual nonce length was {}.",
                self.ivLength(),
                iv.len()
            );
            return dec_result(&msg);
        }

        if *self.tagLength() as usize != authTag.len() {
            let msg = format!(
                "AESEncrypt : algorithm authTag length was {} but actual authTag length was {}.",
                self.tagLength(),
                authTag.len()
            );
            return dec_result(&msg);
        }

        match self.do_aes_decrypt(&key, &cipherTxt, &authTag, &iv, &aad) {
            Ok(x) => Rc::new(Wrappers::Result::Success {
                value: x.iter().cloned().collect(),
            }),
            Err(e) => {
                let msg = format!("{}", e);
                dec_result(&msg)
            }
        }
    }
}

fn print_alg(x: &aws_lc_rs::aead::Algorithm, y: &str) {
    println!(
        "{} : key : {}, tag : {}, nonce : {}",
        y,
        x.key_len(),
        x.tag_len(),
        x.nonce_len()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;
    #[test]
    fn test_generate() {
        let iv: ::dafny_runtime::Sequence<u8> = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
            .iter()
            .cloned()
            .collect();
        let key: ::dafny_runtime::Sequence<u8> = [
            2u8, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32, 33,
        ]
        .iter()
        .cloned()
        .collect();
        let msg: ::dafny_runtime::Sequence<u8> = [2u8, 4, 6, 8, 10, 12].iter().cloned().collect();
        let aad: ::dafny_runtime::Sequence<u8> =
            [3u8, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17]
                .iter()
                .cloned()
                .collect();

        let alg = AES_GCM::AES_GCM {
            keyLength: 32,
            tagLength: 16,
            ivLength: 12,
        };
        let cipher = match &*alg.AESEncryptExtern(&iv, &key, &msg, &aad) {
            Wrappers::Result::Success { value } => value.clone(),
            Wrappers::Result::Failure { error } => {
                panic!("AESEncryptExtern Failed : {:?}", error);
            }
        };

        let (cipherText, authTag) = match &*cipher {
            AESEncryptOutput::AESEncryptOutput {
                cipherText,
                authTag,
            } => (cipherText, authTag),
        };

        let output = match &*alg.AESDecryptExtern(&key, &cipherText, &authTag, &iv, &aad) {
            Wrappers::Result::Success { value } => value.clone(),
            Wrappers::Result::Failure { error } => {
                panic!("AESEncryptExtern Failed : {:?}", error);
            }
        };

        assert_eq!(output, msg);
    }
}
