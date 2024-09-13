// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::put_item_output_transform::PutItemOutputTransformInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::PutItemOutputTransformInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::PutItemOutputTransformInput::PutItemOutputTransformInput {
        sdkOutput: dynamodb::conversions::put_item_output::to_dafny(&value.sdk_output.clone().unwrap())
,
 originalInput: dynamodb::conversions::put_item_input::to_dafny(&value.original_input.clone().unwrap())
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::PutItemOutputTransformInput,
    >,
) -> crate::operation::put_item_output_transform::PutItemOutputTransformInput {
    crate::operation::put_item_output_transform::PutItemOutputTransformInput::builder()
        .set_sdk_output(Some( dynamodb::conversions::put_item_output::from_dafny(dafny_value.sdkOutput().clone())
 ))
 .set_original_input(Some( dynamodb::conversions::put_item_input::from_dafny(dafny_value.originalInput().clone())
 ))
        .build()
        .unwrap()
}