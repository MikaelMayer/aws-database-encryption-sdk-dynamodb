// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::update_item_output_transform::UpdateItemOutputTransformOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::UpdateItemOutputTransformOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::UpdateItemOutputTransformOutput::UpdateItemOutputTransformOutput {
        transformedOutput: dynamodb::conversions::update_item_output::to_dafny(&value.transformed_output.clone().unwrap())
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::UpdateItemOutputTransformOutput,
    >,
) -> crate::operation::update_item_output_transform::UpdateItemOutputTransformOutput {
    crate::operation::update_item_output_transform::UpdateItemOutputTransformOutput::builder()
        .set_transformed_output(Some( dynamodb::conversions::update_item_output::from_dafny(dafny_value.transformedOutput().clone())
 ))
        .build()
        .unwrap()
}
