// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::execute_transaction_input_transform::ExecuteTransactionInputTransformInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ExecuteTransactionInputTransformInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ExecuteTransactionInputTransformInput::ExecuteTransactionInputTransformInput {
        sdkInput: dynamodb::conversions::execute_transaction_input::to_dafny(&value.sdk_input.clone().unwrap())
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ExecuteTransactionInputTransformInput,
    >,
) -> crate::operation::execute_transaction_input_transform::ExecuteTransactionInputTransformInput {
    crate::operation::execute_transaction_input_transform::ExecuteTransactionInputTransformInput::builder()
        .set_sdk_input(Some( dynamodb::conversions::execute_transaction_input::from_dafny(dafny_value.sdkInput().clone())
 ))
        .build()
        .unwrap()
}
