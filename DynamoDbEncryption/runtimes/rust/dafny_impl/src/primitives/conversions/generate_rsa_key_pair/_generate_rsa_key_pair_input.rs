// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::primitives::operation::generate_rsa_key_pair::GenerateRsaKeyPairInput,
) -> ::std::rc::Rc<
    crate::software::amazon::cryptography::primitives::internaldafny::types::GenerateRSAKeyPairInput,
>{
    ::std::rc::Rc::new(crate::software::amazon::cryptography::primitives::internaldafny::types::GenerateRSAKeyPairInput::GenerateRSAKeyPairInput {
        lengthBits: value.length_bits.clone(),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::software::amazon::cryptography::primitives::internaldafny::types::GenerateRSAKeyPairInput,
    >,
) -> crate::primitives::operation::generate_rsa_key_pair::GenerateRsaKeyPairInput {
    crate::primitives::operation::generate_rsa_key_pair::GenerateRsaKeyPairInput::builder()
        .set_length_bits(crate::ddb::standard_library_conversions::oint_from_dafny(dafny_value.lengthBits().clone()))
        .build()
        .unwrap()
}
