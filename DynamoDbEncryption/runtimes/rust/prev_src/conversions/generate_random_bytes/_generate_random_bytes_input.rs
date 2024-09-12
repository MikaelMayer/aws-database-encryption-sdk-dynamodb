// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::generate_random_bytes::GenerateRandomBytesInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GenerateRandomBytesInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GenerateRandomBytesInput::GenerateRandomBytesInput {
        length: value.length.clone(),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GenerateRandomBytesInput,
    >,
) -> crate::operation::generate_random_bytes::GenerateRandomBytesInput {
    crate::operation::generate_random_bytes::GenerateRandomBytesInput::builder()
        .set_length(crate::standard_library_conversions::oint_from_dafny(dafny_value.length().clone()))
        .build()
        .unwrap()
}
