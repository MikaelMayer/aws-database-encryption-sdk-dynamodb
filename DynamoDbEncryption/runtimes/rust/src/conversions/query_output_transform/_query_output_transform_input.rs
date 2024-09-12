// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::query_output_transform::QueryOutputTransformInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::QueryOutputTransformInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::QueryOutputTransformInput::QueryOutputTransformInput {
        sdkOutput: dynamodb::conversions::query_output::to_dafny(&value.sdk_output.clone().unwrap())
,
 originalInput: dynamodb::conversions::query_input::to_dafny(&value.original_input.clone().unwrap())
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::QueryOutputTransformInput,
    >,
) -> crate::operation::query_output_transform::QueryOutputTransformInput {
    crate::operation::query_output_transform::QueryOutputTransformInput::builder()
        .set_sdk_output(Some( dynamodb::conversions::query_output::from_dafny(dafny_value.sdkOutput().clone())
 ))
 .set_original_input(Some( dynamodb::conversions::query_input::from_dafny(dafny_value.originalInput().clone())
 ))
        .build()
        .unwrap()
}
