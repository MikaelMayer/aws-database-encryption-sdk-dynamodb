// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::primitives::operation::kdf_counter_mode::KdfCtrInput,
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::KdfCtrInput,
>{
    ::std::rc::Rc::new(crate::r#software::amazon::cryptography::primitives::internaldafny::types::KdfCtrInput::KdfCtrInput {
        digestAlgorithm: crate::primitives::conversions::digest_algorithm::to_dafny(value.digest_algorithm.clone().unwrap()),
 ikm: crate::ddb::standard_library_conversions::oblob_to_dafny(&value.ikm).Extract(),
 expectedLength: value.expected_length.clone().unwrap(),
 purpose: crate::ddb::standard_library_conversions::oblob_to_dafny(&value.purpose),
 nonce: crate::ddb::standard_library_conversions::oblob_to_dafny(&value.nonce),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::KdfCtrInput,
    >,
) -> crate::primitives::operation::kdf_counter_mode::KdfCtrInput {
    crate::primitives::operation::kdf_counter_mode::KdfCtrInput::builder()
        .set_digest_algorithm(Some( crate::primitives::conversions::digest_algorithm::from_dafny(dafny_value.digestAlgorithm()) ))
 .set_ikm(Some(crate::ddb::standard_library_conversions::blob_from_dafny(dafny_value.ikm().clone())))
 .set_expected_length(Some( dafny_value.expectedLength() .clone() ))
 .set_purpose(crate::ddb::standard_library_conversions::oblob_from_dafny(dafny_value.purpose().clone()))
 .set_nonce(crate::ddb::standard_library_conversions::oblob_from_dafny(dafny_value.nonce().clone()))
        .build()
        .unwrap()
}
