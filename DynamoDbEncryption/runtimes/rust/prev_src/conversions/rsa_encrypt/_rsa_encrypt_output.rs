// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::rsa_encrypt::RsaEncryptOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::RSAEncryptOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::RSAEncryptOutput::RSAEncryptOutput {
        cipherText: crate::standard_library_conversions::oblob_to_dafny(&value.cipher_text).Extract(),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::RSAEncryptOutput,
    >,
) -> crate::operation::rsa_encrypt::RsaEncryptOutput {
    crate::operation::rsa_encrypt::RsaEncryptOutput::builder()
        .set_cipher_text(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.cipherText().clone())))
        .build()
        .unwrap()
}
