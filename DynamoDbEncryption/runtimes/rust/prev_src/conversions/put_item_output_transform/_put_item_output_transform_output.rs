// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::put_item_output_transform::PutItemOutputTransformOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::PutItemOutputTransformOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::PutItemOutputTransformOutput::PutItemOutputTransformOutput {
        transformedOutput: dynamodb::conversions::put_item_output::to_dafny(&value.transformed_output.clone().unwrap())
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::PutItemOutputTransformOutput,
    >,
) -> crate::operation::put_item_output_transform::PutItemOutputTransformOutput {
    crate::operation::put_item_output_transform::PutItemOutputTransformOutput::builder()
        .set_transformed_output(Some( dynamodb::conversions::put_item_output::from_dafny(dafny_value.transformedOutput().clone())
 ))
        .build()
        .unwrap()
}
