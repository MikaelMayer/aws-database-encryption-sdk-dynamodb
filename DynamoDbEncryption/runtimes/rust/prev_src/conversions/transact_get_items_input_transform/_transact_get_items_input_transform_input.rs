// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::transact_get_items_input_transform::TransactGetItemsInputTransformInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::TransactGetItemsInputTransformInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::TransactGetItemsInputTransformInput::TransactGetItemsInputTransformInput {
        sdkInput: dynamodb::conversions::transact_get_items_input::to_dafny(&value.sdk_input.clone().unwrap())
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::TransactGetItemsInputTransformInput,
    >,
) -> crate::operation::transact_get_items_input_transform::TransactGetItemsInputTransformInput {
    crate::operation::transact_get_items_input_transform::TransactGetItemsInputTransformInput::builder()
        .set_sdk_input(Some( dynamodb::conversions::transact_get_items_input::from_dafny(dafny_value.sdkInput().clone())
 ))
        .build()
        .unwrap()
}