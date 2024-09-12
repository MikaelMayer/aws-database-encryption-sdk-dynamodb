// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::ecdsa_verify::EcdsaVerifyInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ECDSAVerifyInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ECDSAVerifyInput::ECDSAVerifyInput {
        signatureAlgorithm: primitives::conversions::ecdsa_signature_algorithm::to_dafny(value.signature_algorithm.clone().unwrap()),
 verificationKey: crate::standard_library_conversions::oblob_to_dafny(&value.verification_key).Extract(),
 message: crate::standard_library_conversions::oblob_to_dafny(&value.message).Extract(),
 signature: crate::standard_library_conversions::oblob_to_dafny(&value.signature).Extract(),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ECDSAVerifyInput,
    >,
) -> crate::operation::ecdsa_verify::EcdsaVerifyInput {
    crate::operation::ecdsa_verify::EcdsaVerifyInput::builder()
        .set_signature_algorithm(Some( primitives::conversions::ecdsa_signature_algorithm::from_dafny(dafny_value.signatureAlgorithm()) ))
 .set_verification_key(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.verificationKey().clone())))
 .set_message(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.message().clone())))
 .set_signature(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.signature().clone())))
        .build()
        .unwrap()
}
