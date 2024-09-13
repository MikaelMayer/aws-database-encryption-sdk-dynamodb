// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::batch_get_item_input_transform::BatchGetItemInputTransformOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::BatchGetItemInputTransformOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::BatchGetItemInputTransformOutput::BatchGetItemInputTransformOutput {
        transformedInput: dynamodb::conversions::batch_get_item_input::to_dafny(&value.transformed_input.clone().unwrap())
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::BatchGetItemInputTransformOutput,
    >,
) -> crate::operation::batch_get_item_input_transform::BatchGetItemInputTransformOutput {
    crate::operation::batch_get_item_input_transform::BatchGetItemInputTransformOutput::builder()
        .set_transformed_input(Some( dynamodb::conversions::batch_get_item_input::from_dafny(dafny_value.transformedInput().clone())
 ))
        .build()
        .unwrap()
}