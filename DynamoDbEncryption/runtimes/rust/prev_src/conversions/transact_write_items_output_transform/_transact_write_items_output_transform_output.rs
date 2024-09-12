// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::transact_write_items_output_transform::TransactWriteItemsOutputTransformOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::TransactWriteItemsOutputTransformOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::TransactWriteItemsOutputTransformOutput::TransactWriteItemsOutputTransformOutput {
        transformedOutput: dynamodb::conversions::transact_write_items_output::to_dafny(&value.transformed_output.clone().unwrap())
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::TransactWriteItemsOutputTransformOutput,
    >,
) -> crate::operation::transact_write_items_output_transform::TransactWriteItemsOutputTransformOutput {
    crate::operation::transact_write_items_output_transform::TransactWriteItemsOutputTransformOutput::builder()
        .set_transformed_output(Some( dynamodb::conversions::transact_write_items_output::from_dafny(dafny_value.transformedOutput().clone())
 ))
        .build()
        .unwrap()
}
