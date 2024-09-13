// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::batch_execute_statement_input_transform::BatchExecuteStatementInputTransformOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::BatchExecuteStatementInputTransformOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::BatchExecuteStatementInputTransformOutput::BatchExecuteStatementInputTransformOutput {
        transformedInput: dynamodb::conversions::batch_execute_statement_input::to_dafny(&value.transformed_input.clone().unwrap())
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::BatchExecuteStatementInputTransformOutput,
    >,
) -> crate::operation::batch_execute_statement_input_transform::BatchExecuteStatementInputTransformOutput {
    crate::operation::batch_execute_statement_input_transform::BatchExecuteStatementInputTransformOutput::builder()
        .set_transformed_input(Some( dynamodb::conversions::batch_execute_statement_input::from_dafny(dafny_value.transformedInput().clone())
 ))
        .build()
        .unwrap()
}