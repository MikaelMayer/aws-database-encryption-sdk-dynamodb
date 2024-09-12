// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::delete_item_output_transform::DeleteItemOutputTransformInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DeleteItemOutputTransformInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DeleteItemOutputTransformInput::DeleteItemOutputTransformInput {
        sdkOutput: dynamodb::conversions::delete_item_output::to_dafny(&value.sdk_output.clone().unwrap())
,
 originalInput: dynamodb::conversions::delete_item_input::to_dafny(&value.original_input.clone().unwrap())
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DeleteItemOutputTransformInput,
    >,
) -> crate::operation::delete_item_output_transform::DeleteItemOutputTransformInput {
    crate::operation::delete_item_output_transform::DeleteItemOutputTransformInput::builder()
        .set_sdk_output(Some( dynamodb::conversions::delete_item_output::from_dafny(dafny_value.sdkOutput().clone())
 ))
 .set_original_input(Some( dynamodb::conversions::delete_item_input::from_dafny(dafny_value.originalInput().clone())
 ))
        .build()
        .unwrap()
}
