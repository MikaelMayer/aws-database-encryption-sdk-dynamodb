// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::update_item_input_transform::UpdateItemInputTransformOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::UpdateItemInputTransformOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::UpdateItemInputTransformOutput::UpdateItemInputTransformOutput {
        transformedInput: dynamodb::conversions::update_item_input::to_dafny(&value.transformed_input.clone().unwrap())
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::UpdateItemInputTransformOutput,
    >,
) -> crate::operation::update_item_input_transform::UpdateItemInputTransformOutput {
    crate::operation::update_item_input_transform::UpdateItemInputTransformOutput::builder()
        .set_transformed_input(Some( dynamodb::conversions::update_item_input::from_dafny(dafny_value.transformedInput().clone())
 ))
        .build()
        .unwrap()
}
