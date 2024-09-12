// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::batch_write_item_input_transform::BatchWriteItemInputTransformInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::BatchWriteItemInputTransformInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::BatchWriteItemInputTransformInput::BatchWriteItemInputTransformInput {
        sdkInput: dynamodb::conversions::batch_write_item_input::to_dafny(&value.sdk_input.clone().unwrap())
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::BatchWriteItemInputTransformInput,
    >,
) -> crate::operation::batch_write_item_input_transform::BatchWriteItemInputTransformInput {
    crate::operation::batch_write_item_input_transform::BatchWriteItemInputTransformInput::builder()
        .set_sdk_input(Some( dynamodb::conversions::batch_write_item_input::from_dafny(dafny_value.sdkInput().clone())
 ))
        .build()
        .unwrap()
}
