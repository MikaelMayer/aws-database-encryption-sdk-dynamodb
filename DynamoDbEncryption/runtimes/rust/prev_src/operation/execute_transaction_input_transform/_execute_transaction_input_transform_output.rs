// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ExecuteTransactionInputTransformOutput {
    #[allow(missing_docs)] // documentation missing in model
pub transformed_input: ::std::option::Option<dynamodb::types::ExecuteTransactionInput>,
}
impl ExecuteTransactionInputTransformOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn transformed_input(&self) -> &::std::option::Option<dynamodb::types::ExecuteTransactionInput> {
    &self.transformed_input
}
}
impl ExecuteTransactionInputTransformOutput {
    /// Creates a new builder-style object to manufacture [`ExecuteTransactionInputTransformOutput`](crate::operation::execute_transaction_input_transform::builders::ExecuteTransactionInputTransformOutput).
    pub fn builder() -> crate::operation::execute_transaction_input_transform::builders::ExecuteTransactionInputTransformOutputBuilder {
        crate::operation::execute_transaction_input_transform::builders::ExecuteTransactionInputTransformOutputBuilder::default()
    }
}

/// A builder for [`ExecuteTransactionInputTransformOutput`](crate::operation::operation::ExecuteTransactionInputTransformOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ExecuteTransactionInputTransformOutputBuilder {
    pub(crate) transformed_input: ::std::option::Option<dynamodb::types::ExecuteTransactionInput>,
}
impl ExecuteTransactionInputTransformOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn transformed_input(mut self, input: impl ::std::convert::Into<dynamodb::types::ExecuteTransactionInput>) -> Self {
    self.transformed_input = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_transformed_input(mut self, input: ::std::option::Option<dynamodb::types::ExecuteTransactionInput>) -> Self {
    self.transformed_input = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_transformed_input(&self) -> &::std::option::Option<dynamodb::types::ExecuteTransactionInput> {
    &self.transformed_input
}
    /// Consumes the builder and constructs a [`ExecuteTransactionInputTransformOutput`](crate::operation::operation::ExecuteTransactionInputTransformOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::execute_transaction_input_transform::ExecuteTransactionInputTransformOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::execute_transaction_input_transform::ExecuteTransactionInputTransformOutput {
            transformed_input: self.transformed_input,
        })
    }
}