// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::rsa_decrypt::RsaDecryptOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::RSADecryptOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::RSADecryptOutput::RSADecryptOutput {
        plaintext: crate::standard_library_conversions::oblob_to_dafny(&value.plaintext).Extract(),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::RSADecryptOutput,
    >,
) -> crate::operation::rsa_decrypt::RsaDecryptOutput {
    crate::operation::rsa_decrypt::RsaDecryptOutput::builder()
        .set_plaintext(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.plaintext().clone())))
        .build()
        .unwrap()
}
