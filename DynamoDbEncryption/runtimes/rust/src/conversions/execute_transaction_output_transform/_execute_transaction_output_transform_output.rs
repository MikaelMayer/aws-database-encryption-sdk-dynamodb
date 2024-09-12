// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::execute_transaction_output_transform::ExecuteTransactionOutputTransformOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ExecuteTransactionOutputTransformOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ExecuteTransactionOutputTransformOutput::ExecuteTransactionOutputTransformOutput {
        transformedOutput: dynamodb::conversions::execute_transaction_output::to_dafny(&value.transformed_output.clone().unwrap())
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ExecuteTransactionOutputTransformOutput,
    >,
) -> crate::operation::execute_transaction_output_transform::ExecuteTransactionOutputTransformOutput {
    crate::operation::execute_transaction_output_transform::ExecuteTransactionOutputTransformOutput::builder()
        .set_transformed_output(Some( dynamodb::conversions::execute_transaction_output::from_dafny(dafny_value.transformedOutput().clone())
 ))
        .build()
        .unwrap()
}
