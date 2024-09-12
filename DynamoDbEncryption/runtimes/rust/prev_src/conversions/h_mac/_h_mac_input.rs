// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::h_mac::HMacInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::HMacInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::HMacInput::HMacInput {
        digestAlgorithm: primitives::conversions::digest_algorithm::to_dafny(value.digest_algorithm.clone().unwrap()),
 key: crate::standard_library_conversions::oblob_to_dafny(&value.key).Extract(),
 message: crate::standard_library_conversions::oblob_to_dafny(&value.message).Extract(),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::HMacInput,
    >,
) -> crate::operation::h_mac::HMacInput {
    crate::operation::h_mac::HMacInput::builder()
        .set_digest_algorithm(Some( primitives::conversions::digest_algorithm::from_dafny(dafny_value.digestAlgorithm()) ))
 .set_key(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.key().clone())))
 .set_message(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.message().clone())))
        .build()
        .unwrap()
}
