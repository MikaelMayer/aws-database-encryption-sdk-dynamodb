// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::primitives::operation::aes_encrypt::AesEncryptOutput,
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::AESEncryptOutput,
>{
    ::std::rc::Rc::new(crate::r#software::amazon::cryptography::primitives::internaldafny::types::AESEncryptOutput::AESEncryptOutput {
        cipherText: crate::ddb::standard_library_conversions::oblob_to_dafny(&value.cipher_text).Extract(),
 authTag: crate::ddb::standard_library_conversions::oblob_to_dafny(&value.auth_tag).Extract(),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::AESEncryptOutput,
    >,
) -> crate::primitives::operation::aes_encrypt::AesEncryptOutput {
    crate::primitives::operation::aes_encrypt::AesEncryptOutput::builder()
        .set_cipher_text(Some(crate::ddb::standard_library_conversions::blob_from_dafny(dafny_value.cipherText().clone())))
 .set_auth_tag(Some(crate::ddb::standard_library_conversions::blob_from_dafny(dafny_value.authTag().clone())))
        .build()
        .unwrap()
}