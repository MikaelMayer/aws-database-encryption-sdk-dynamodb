// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::transact_write_items_output_transform::TransactWriteItemsOutputTransformInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::TransactWriteItemsOutputTransformInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::TransactWriteItemsOutputTransformInput::TransactWriteItemsOutputTransformInput {
        sdkOutput: dynamodb::conversions::transact_write_items_output::to_dafny(&value.sdk_output.clone().unwrap())
,
 originalInput: dynamodb::conversions::transact_write_items_input::to_dafny(&value.original_input.clone().unwrap())
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::TransactWriteItemsOutputTransformInput,
    >,
) -> crate::operation::transact_write_items_output_transform::TransactWriteItemsOutputTransformInput {
    crate::operation::transact_write_items_output_transform::TransactWriteItemsOutputTransformInput::builder()
        .set_sdk_output(Some( dynamodb::conversions::transact_write_items_output::from_dafny(dafny_value.sdkOutput().clone())
 ))
 .set_original_input(Some( dynamodb::conversions::transact_write_items_input::from_dafny(dafny_value.originalInput().clone())
 ))
        .build()
        .unwrap()
}