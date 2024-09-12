// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::kdf_counter_mode::KdfCtrInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::KdfCtrInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::KdfCtrInput::KdfCtrInput {
        digestAlgorithm: primitives::conversions::digest_algorithm::to_dafny(value.digest_algorithm.clone().unwrap()),
 ikm: crate::standard_library_conversions::oblob_to_dafny(&value.ikm).Extract(),
 expectedLength: value.expected_length.clone(),
 purpose: crate::standard_library_conversions::oblob_to_dafny(&value.purpose),
 nonce: crate::standard_library_conversions::oblob_to_dafny(&value.nonce),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::KdfCtrInput,
    >,
) -> crate::operation::kdf_counter_mode::KdfCtrInput {
    crate::operation::kdf_counter_mode::KdfCtrInput::builder()
        .set_digest_algorithm(Some( primitives::conversions::digest_algorithm::from_dafny(dafny_value.digestAlgorithm()) ))
 .set_ikm(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.ikm().clone())))
 .set_expected_length(crate::standard_library_conversions::oint_from_dafny(dafny_value.expectedLength().clone()))
 .set_purpose(crate::standard_library_conversions::oblob_from_dafny(dafny_value.purpose().clone()))
 .set_nonce(crate::standard_library_conversions::oblob_from_dafny(dafny_value.nonce().clone()))
        .build()
        .unwrap()
}
