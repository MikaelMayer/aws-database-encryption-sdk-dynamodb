// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::primitives::operation::h_mac::HMacInput,
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::HMacInput,
>{
    ::std::rc::Rc::new(crate::r#software::amazon::cryptography::primitives::internaldafny::types::HMacInput::HMacInput {
        digestAlgorithm: crate::primitives::conversions::digest_algorithm::to_dafny(value.digest_algorithm.clone().unwrap()),
 key: crate::ddb::standard_library_conversions::oblob_to_dafny(&value.key).Extract(),
 message: crate::ddb::standard_library_conversions::oblob_to_dafny(&value.message).Extract(),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::HMacInput,
    >,
) -> crate::primitives::operation::h_mac::HMacInput {
    crate::primitives::operation::h_mac::HMacInput::builder()
        .set_digest_algorithm(Some( crate::primitives::conversions::digest_algorithm::from_dafny(dafny_value.digestAlgorithm()) ))
 .set_key(Some(crate::ddb::standard_library_conversions::blob_from_dafny(dafny_value.key().clone())))
 .set_message(Some(crate::ddb::standard_library_conversions::blob_from_dafny(dafny_value.message().clone())))
        .build()
        .unwrap()
}
