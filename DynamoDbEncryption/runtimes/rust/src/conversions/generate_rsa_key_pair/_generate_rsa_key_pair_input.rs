// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::generate_rsa_key_pair::GenerateRsaKeyPairInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GenerateRSAKeyPairInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GenerateRSAKeyPairInput::GenerateRSAKeyPairInput {
        lengthBits: value.length_bits.clone(),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GenerateRSAKeyPairInput,
    >,
) -> crate::operation::generate_rsa_key_pair::GenerateRsaKeyPairInput {
    crate::operation::generate_rsa_key_pair::GenerateRsaKeyPairInput::builder()
        .set_length_bits(crate::standard_library_conversions::oint_from_dafny(dafny_value.lengthBits().clone()))
        .build()
        .unwrap()
}
