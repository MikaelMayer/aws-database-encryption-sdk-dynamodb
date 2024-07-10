// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![allow(warnings, unconditional_panic)]
#![allow(nonstandard_style)]
use crate::software::amazon::cryptography::primitives::internaldafny::types::AESEncryptOutput;
use crate::software::amazon::cryptography::primitives::internaldafny::types::Error as DafnyError;
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
impl crate::software::amazon::cryptography::primitives::internaldafny::types::AES_GCM {
    #[allow(non_snake_case)]

    fn do_aes_encrypt(
        iv: &[u8],
        key: &[u8],
        msg: &[u8],
        aad: &[u8],
    ) -> Result<DoAESEncryptOutput, Unspecified> {
        let mut in_out_buffer = Vec::from(msg);
        let key = UnboundKey::new(&aws_lc_rs::aead::AES_256_GCM, &key)?;
        let nonce = Nonce::assume_unique_for_key(iv.try_into().unwrap());
        let mut key = LessSafeKey::new(key);
        let aad = Aad::from(aad);
        let tag = key.seal_in_place_separate_tag(nonce, aad, &mut in_out_buffer)?;
        Ok(DoAESEncryptOutput {
            cipherText: in_out_buffer,
            authTag: Vec::from(tag.as_ref()),
        })
    }

    fn do_aes_decrypt(
        key: &[u8],
        cipherTxt: &[u8],
        authTag: &[u8],
        iv: &[u8],
        aad: &[u8],
    ) -> Result<Vec<u8>, Unspecified> {
        let mut out_buffer = Vec::from(cipherTxt);
        let key = UnboundKey::new(&aws_lc_rs::aead::AES_256_GCM, &key)?;
        let nonce = Nonce::assume_unique_for_key(iv.try_into().unwrap());
        let mut key = LessSafeKey::new(key);
        let aad = Aad::from(aad);
        key.open_separate_gather(nonce, aad, cipherTxt, authTag, &mut out_buffer)?;
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

        match Self::do_aes_encrypt(&iv, &key, &msg, &aad) {
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

        match Self::do_aes_decrypt(&key, &cipherTxt, &authTag, &iv, &aad) {
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
