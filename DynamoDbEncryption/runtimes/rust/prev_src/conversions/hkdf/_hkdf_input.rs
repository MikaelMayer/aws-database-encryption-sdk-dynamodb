// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::hkdf::HkdfInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::HkdfInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::HkdfInput::HkdfInput {
        digestAlgorithm: primitives::conversions::digest_algorithm::to_dafny(value.digest_algorithm.clone().unwrap()),
 salt: crate::standard_library_conversions::oblob_to_dafny(&value.salt),
 ikm: crate::standard_library_conversions::oblob_to_dafny(&value.ikm).Extract(),
 info: crate::standard_library_conversions::oblob_to_dafny(&value.info).Extract(),
 expectedLength: value.expected_length.clone(),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::HkdfInput,
    >,
) -> crate::operation::hkdf::HkdfInput {
    crate::operation::hkdf::HkdfInput::builder()
        .set_digest_algorithm(Some( primitives::conversions::digest_algorithm::from_dafny(dafny_value.digestAlgorithm()) ))
 .set_salt(crate::standard_library_conversions::oblob_from_dafny(dafny_value.salt().clone()))
 .set_ikm(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.ikm().clone())))
 .set_info(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.info().clone())))
 .set_expected_length(crate::standard_library_conversions::oint_from_dafny(dafny_value.expectedLength().clone()))
        .build()
        .unwrap()
}
