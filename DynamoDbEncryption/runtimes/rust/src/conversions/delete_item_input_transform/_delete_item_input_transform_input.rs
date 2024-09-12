// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::delete_item_input_transform::DeleteItemInputTransformInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DeleteItemInputTransformInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DeleteItemInputTransformInput::DeleteItemInputTransformInput {
        sdkInput: dynamodb::conversions::delete_item_input::to_dafny(&value.sdk_input.clone().unwrap())
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DeleteItemInputTransformInput,
    >,
) -> crate::operation::delete_item_input_transform::DeleteItemInputTransformInput {
    crate::operation::delete_item_input_transform::DeleteItemInputTransformInput::builder()
        .set_sdk_input(Some( dynamodb::conversions::delete_item_input::from_dafny(dafny_value.sdkInput().clone())
 ))
        .build()
        .unwrap()
}
