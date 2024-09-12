// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::rsa_decrypt::RsaDecryptInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::RSADecryptInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::RSADecryptInput::RSADecryptInput {
        padding: primitives::conversions::rsa_padding_mode::to_dafny(value.padding.clone().unwrap()),
 privateKey: crate::standard_library_conversions::oblob_to_dafny(&value.private_key).Extract(),
 cipherText: crate::standard_library_conversions::oblob_to_dafny(&value.cipher_text).Extract(),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::RSADecryptInput,
    >,
) -> crate::operation::rsa_decrypt::RsaDecryptInput {
    crate::operation::rsa_decrypt::RsaDecryptInput::builder()
        .set_padding(Some( primitives::conversions::rsa_padding_mode::from_dafny(dafny_value.padding()) ))
 .set_private_key(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.privateKey().clone())))
 .set_cipher_text(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.cipherText().clone())))
        .build()
        .unwrap()
}
