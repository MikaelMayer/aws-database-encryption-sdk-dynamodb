// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::ecdsa_sign::EcdsaSignInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ECDSASignInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ECDSASignInput::ECDSASignInput {
        signatureAlgorithm: primitives::conversions::ecdsa_signature_algorithm::to_dafny(value.signature_algorithm.clone().unwrap()),
 signingKey: crate::standard_library_conversions::oblob_to_dafny(&value.signing_key).Extract(),
 message: crate::standard_library_conversions::oblob_to_dafny(&value.message).Extract(),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ECDSASignInput,
    >,
) -> crate::operation::ecdsa_sign::EcdsaSignInput {
    crate::operation::ecdsa_sign::EcdsaSignInput::builder()
        .set_signature_algorithm(Some( primitives::conversions::ecdsa_signature_algorithm::from_dafny(dafny_value.signatureAlgorithm()) ))
 .set_signing_key(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.signingKey().clone())))
 .set_message(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.message().clone())))
        .build()
        .unwrap()
}
