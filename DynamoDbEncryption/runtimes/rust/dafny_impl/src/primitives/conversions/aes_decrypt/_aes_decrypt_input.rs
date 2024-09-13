// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::primitives::operation::aes_decrypt::AesDecryptInput,
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::AESDecryptInput,
>{
    ::std::rc::Rc::new(crate::r#software::amazon::cryptography::primitives::internaldafny::types::AESDecryptInput::AESDecryptInput {
        encAlg: crate::primitives::conversions::aes_gcm::to_dafny(value.enc_alg.clone().unwrap())
,
 key: crate::ddb::standard_library_conversions::oblob_to_dafny(&value.key).Extract(),
 cipherTxt: crate::ddb::standard_library_conversions::oblob_to_dafny(&value.cipher_txt).Extract(),
 authTag: crate::ddb::standard_library_conversions::oblob_to_dafny(&value.auth_tag).Extract(),
 iv: crate::ddb::standard_library_conversions::oblob_to_dafny(&value.iv).Extract(),
 aad: crate::ddb::standard_library_conversions::oblob_to_dafny(&value.aad).Extract(),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::AESDecryptInput,
    >,
) -> crate::primitives::operation::aes_decrypt::AesDecryptInput {
    crate::primitives::operation::aes_decrypt::AesDecryptInput::builder()
        .set_enc_alg(Some( crate::primitives::conversions::aes_gcm::from_dafny(dafny_value.encAlg().clone())
 ))
 .set_key(Some(crate::ddb::standard_library_conversions::blob_from_dafny(dafny_value.key().clone())))
 .set_cipher_txt(Some(crate::ddb::standard_library_conversions::blob_from_dafny(dafny_value.cipherTxt().clone())))
 .set_auth_tag(Some(crate::ddb::standard_library_conversions::blob_from_dafny(dafny_value.authTag().clone())))
 .set_iv(Some(crate::ddb::standard_library_conversions::blob_from_dafny(dafny_value.iv().clone())))
 .set_aad(Some(crate::ddb::standard_library_conversions::blob_from_dafny(dafny_value.aad().clone())))
        .build()
        .unwrap()
}
