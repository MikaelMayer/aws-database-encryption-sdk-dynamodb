// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::primitives::operation::ecdsa_sign::EcdsaSignInput,
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::ECDSASignInput,
>{
    ::std::rc::Rc::new(crate::r#software::amazon::cryptography::primitives::internaldafny::types::ECDSASignInput::ECDSASignInput {
        signatureAlgorithm: crate::primitives::conversions::ecdsa_signature_algorithm::to_dafny(value.signature_algorithm.clone().unwrap()),
 signingKey: crate::ddb::standard_library_conversions::oblob_to_dafny(&value.signing_key).Extract(),
 message: crate::ddb::standard_library_conversions::oblob_to_dafny(&value.message).Extract(),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::ECDSASignInput,
    >,
) -> crate::primitives::operation::ecdsa_sign::EcdsaSignInput {
    crate::primitives::operation::ecdsa_sign::EcdsaSignInput::builder()
        .set_signature_algorithm(Some( crate::primitives::conversions::ecdsa_signature_algorithm::from_dafny(dafny_value.signatureAlgorithm()) ))
 .set_signing_key(Some(crate::ddb::standard_library_conversions::blob_from_dafny(dafny_value.signingKey().clone())))
 .set_message(Some(crate::ddb::standard_library_conversions::blob_from_dafny(dafny_value.message().clone())))
        .build()
        .unwrap()
}
