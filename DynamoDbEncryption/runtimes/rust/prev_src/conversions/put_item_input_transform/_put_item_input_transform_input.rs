// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::put_item_input_transform::PutItemInputTransformInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::PutItemInputTransformInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::PutItemInputTransformInput::PutItemInputTransformInput {
        sdkInput: dynamodb::conversions::put_item_input::to_dafny(&value.sdk_input.clone().unwrap())
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::PutItemInputTransformInput,
    >,
) -> crate::operation::put_item_input_transform::PutItemInputTransformInput {
    crate::operation::put_item_input_transform::PutItemInputTransformInput::builder()
        .set_sdk_input(Some( dynamodb::conversions::put_item_input::from_dafny(dafny_value.sdkInput().clone())
 ))
        .build()
        .unwrap()
}
