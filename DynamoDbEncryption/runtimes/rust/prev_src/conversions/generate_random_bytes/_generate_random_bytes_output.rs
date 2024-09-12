// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::generate_random_bytes::GenerateRandomBytesOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GenerateRandomBytesOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GenerateRandomBytesOutput::GenerateRandomBytesOutput {
        data: crate::standard_library_conversions::oblob_to_dafny(&value.data).Extract(),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GenerateRandomBytesOutput,
    >,
) -> crate::operation::generate_random_bytes::GenerateRandomBytesOutput {
    crate::operation::generate_random_bytes::GenerateRandomBytesOutput::builder()
        .set_data(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.data().clone())))
        .build()
        .unwrap()
}
