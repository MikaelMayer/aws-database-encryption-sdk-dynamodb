// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::primitives::operation::generate_ecdsa_signature_key::GenerateEcdsaSignatureKeyInput,
) -> ::std::rc::Rc<
    crate::software::amazon::cryptography::primitives::internaldafny::types::GenerateECDSASignatureKeyInput,
>{
    ::std::rc::Rc::new(crate::software::amazon::cryptography::primitives::internaldafny::types::GenerateECDSASignatureKeyInput::GenerateECDSASignatureKeyInput {
        signatureAlgorithm: crate::primitives::conversions::ecdsa_signature_algorithm::to_dafny(value.signature_algorithm.clone().unwrap()),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::software::amazon::cryptography::primitives::internaldafny::types::GenerateECDSASignatureKeyInput,
    >,
) -> crate::primitives::operation::generate_ecdsa_signature_key::GenerateEcdsaSignatureKeyInput {
    crate::primitives::operation::generate_ecdsa_signature_key::GenerateEcdsaSignatureKeyInput::builder()
        .set_signature_algorithm(Some( crate::primitives::conversions::ecdsa_signature_algorithm::from_dafny(dafny_value.signatureAlgorithm()) ))
        .build()
        .unwrap()
}
