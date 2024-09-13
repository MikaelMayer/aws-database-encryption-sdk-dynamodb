// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::primitives::operation::hkdf_extract::HkdfExtractInput,
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::HkdfExtractInput,
>{
    ::std::rc::Rc::new(crate::r#software::amazon::cryptography::primitives::internaldafny::types::HkdfExtractInput::HkdfExtractInput {
        digestAlgorithm: crate::primitives::conversions::digest_algorithm::to_dafny(value.digest_algorithm.clone().unwrap()),
 salt: crate::ddb::standard_library_conversions::oblob_to_dafny(&value.salt),
 ikm: crate::ddb::standard_library_conversions::oblob_to_dafny(&value.ikm).Extract(),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::HkdfExtractInput,
    >,
) -> crate::primitives::operation::hkdf_extract::HkdfExtractInput {
    crate::primitives::operation::hkdf_extract::HkdfExtractInput::builder()
        .set_digest_algorithm(Some( crate::primitives::conversions::digest_algorithm::from_dafny(dafny_value.digestAlgorithm()) ))
 .set_salt(crate::ddb::standard_library_conversions::oblob_from_dafny(dafny_value.salt().clone()))
 .set_ikm(Some(crate::ddb::standard_library_conversions::blob_from_dafny(dafny_value.ikm().clone())))
        .build()
        .unwrap()
}
