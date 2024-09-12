// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::primitives::operation::generate_ecdsa_signature_key::GenerateEcdsaSignatureKeyOutput,
) -> ::std::rc::Rc<
    crate::software::amazon::cryptography::primitives::internaldafny::types::GenerateECDSASignatureKeyOutput,
>{
    ::std::rc::Rc::new(crate::software::amazon::cryptography::primitives::internaldafny::types::GenerateECDSASignatureKeyOutput::GenerateECDSASignatureKeyOutput {
        signatureAlgorithm: crate::primitives::conversions::ecdsa_signature_algorithm::to_dafny(value.signature_algorithm.clone().unwrap()),
 verificationKey: crate::ddb::standard_library_conversions::oblob_to_dafny(&value.verification_key).Extract(),
 signingKey: crate::ddb::standard_library_conversions::oblob_to_dafny(&value.signing_key).Extract(),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::software::amazon::cryptography::primitives::internaldafny::types::GenerateECDSASignatureKeyOutput,
    >,
) -> crate::primitives::operation::generate_ecdsa_signature_key::GenerateEcdsaSignatureKeyOutput {
    crate::primitives::operation::generate_ecdsa_signature_key::GenerateEcdsaSignatureKeyOutput::builder()
        .set_signature_algorithm(Some( crate::primitives::conversions::ecdsa_signature_algorithm::from_dafny(dafny_value.signatureAlgorithm()) ))
 .set_verification_key(Some(crate::ddb::standard_library_conversions::blob_from_dafny(dafny_value.verificationKey().clone())))
 .set_signing_key(Some(crate::ddb::standard_library_conversions::blob_from_dafny(dafny_value.signingKey().clone())))
        .build()
        .unwrap()
}
