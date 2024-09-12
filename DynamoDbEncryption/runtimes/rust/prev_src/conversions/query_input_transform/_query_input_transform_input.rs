// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::query_input_transform::QueryInputTransformInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::QueryInputTransformInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::QueryInputTransformInput::QueryInputTransformInput {
        sdkInput: dynamodb::conversions::query_input::to_dafny(&value.sdk_input.clone().unwrap())
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::QueryInputTransformInput,
    >,
) -> crate::operation::query_input_transform::QueryInputTransformInput {
    crate::operation::query_input_transform::QueryInputTransformInput::builder()
        .set_sdk_input(Some( dynamodb::conversions::query_input::from_dafny(dafny_value.sdkInput().clone())
 ))
        .build()
        .unwrap()
}
