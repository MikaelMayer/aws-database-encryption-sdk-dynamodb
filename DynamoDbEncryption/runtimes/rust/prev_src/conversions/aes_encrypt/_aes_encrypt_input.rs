// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::aes_encrypt::AesEncryptInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::AESEncryptInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::AESEncryptInput::AESEncryptInput {
        encAlg: primitives::conversions::aes_gcm::to_dafny(&value.enc_alg.clone().unwrap())
,
 iv: crate::standard_library_conversions::oblob_to_dafny(&value.iv).Extract(),
 key: crate::standard_library_conversions::oblob_to_dafny(&value.key).Extract(),
 msg: crate::standard_library_conversions::oblob_to_dafny(&value.msg).Extract(),
 aad: crate::standard_library_conversions::oblob_to_dafny(&value.aad).Extract(),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::AESEncryptInput,
    >,
) -> crate::operation::aes_encrypt::AesEncryptInput {
    crate::operation::aes_encrypt::AesEncryptInput::builder()
        .set_enc_alg(Some( primitives::conversions::aes_gcm::from_dafny(dafny_value.encAlg().clone())
 ))
 .set_iv(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.iv().clone())))
 .set_key(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.key().clone())))
 .set_msg(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.msg().clone())))
 .set_aad(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.aad().clone())))
        .build()
        .unwrap()
}
