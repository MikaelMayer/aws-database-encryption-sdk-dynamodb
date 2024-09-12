// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::aes_decrypt::AesDecryptOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::AESDecryptOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::AESDecryptOutput::AESDecryptOutput {
        plaintext: crate::standard_library_conversions::oblob_to_dafny(&value.plaintext).Extract(),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::AESDecryptOutput,
    >,
) -> crate::operation::aes_decrypt::AesDecryptOutput {
    crate::operation::aes_decrypt::AesDecryptOutput::builder()
        .set_plaintext(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.plaintext().clone())))
        .build()
        .unwrap()
}
