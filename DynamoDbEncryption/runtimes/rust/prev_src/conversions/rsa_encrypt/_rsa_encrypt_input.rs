// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::rsa_encrypt::RsaEncryptInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::RSAEncryptInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::RSAEncryptInput::RSAEncryptInput {
        padding: primitives::conversions::rsa_padding_mode::to_dafny(value.padding.clone().unwrap()),
 publicKey: crate::standard_library_conversions::oblob_to_dafny(&value.public_key).Extract(),
 plaintext: crate::standard_library_conversions::oblob_to_dafny(&value.plaintext).Extract(),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::RSAEncryptInput,
    >,
) -> crate::operation::rsa_encrypt::RsaEncryptInput {
    crate::operation::rsa_encrypt::RsaEncryptInput::builder()
        .set_padding(Some( primitives::conversions::rsa_padding_mode::from_dafny(dafny_value.padding()) ))
 .set_public_key(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.publicKey().clone())))
 .set_plaintext(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.plaintext().clone())))
        .build()
        .unwrap()
}
