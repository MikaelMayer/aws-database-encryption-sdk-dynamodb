// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::execute_statement_input_transform::ExecuteStatementInputTransformInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ExecuteStatementInputTransformInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ExecuteStatementInputTransformInput::ExecuteStatementInputTransformInput {
        sdkInput: dynamodb::conversions::execute_statement_input::to_dafny(&value.sdk_input.clone().unwrap())
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ExecuteStatementInputTransformInput,
    >,
) -> crate::operation::execute_statement_input_transform::ExecuteStatementInputTransformInput {
    crate::operation::execute_statement_input_transform::ExecuteStatementInputTransformInput::builder()
        .set_sdk_input(Some( dynamodb::conversions::execute_statement_input::from_dafny(dafny_value.sdkInput().clone())
 ))
        .build()
        .unwrap()
}
