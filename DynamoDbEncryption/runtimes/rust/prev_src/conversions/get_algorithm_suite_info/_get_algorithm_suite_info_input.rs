// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::get_algorithm_suite_info::GetAlgorithmSuiteInfoInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GetAlgorithmSuiteInfoInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GetAlgorithmSuiteInfoInput::GetAlgorithmSuiteInfoInput {
        binaryId: crate::standard_library_conversions::oblob_to_dafny(&value.binary_id).Extract(),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GetAlgorithmSuiteInfoInput,
    >,
) -> crate::operation::get_algorithm_suite_info::GetAlgorithmSuiteInfoInput {
    crate::operation::get_algorithm_suite_info::GetAlgorithmSuiteInfoInput::builder()
        .set_binary_id(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.binaryId().clone())))
        .build()
        .unwrap()
}
