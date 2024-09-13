// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::primitives::operation::rsa_encrypt::RsaEncryptInput,
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::RSAEncryptInput,
>{
    ::std::rc::Rc::new(crate::r#software::amazon::cryptography::primitives::internaldafny::types::RSAEncryptInput::RSAEncryptInput {
        padding: crate::primitives::conversions::rsa_padding_mode::to_dafny(value.padding.clone().unwrap()),
 publicKey: crate::ddb::standard_library_conversions::oblob_to_dafny(&value.public_key).Extract(),
 plaintext: crate::ddb::standard_library_conversions::oblob_to_dafny(&value.plaintext).Extract(),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::RSAEncryptInput,
    >,
) -> crate::primitives::operation::rsa_encrypt::RsaEncryptInput {
    crate::primitives::operation::rsa_encrypt::RsaEncryptInput::builder()
        .set_padding(Some( crate::primitives::conversions::rsa_padding_mode::from_dafny(dafny_value.padding()) ))
 .set_public_key(Some(crate::ddb::standard_library_conversions::blob_from_dafny(dafny_value.publicKey().clone())))
 .set_plaintext(Some(crate::ddb::standard_library_conversions::blob_from_dafny(dafny_value.plaintext().clone())))
        .build()
        .unwrap()
}
