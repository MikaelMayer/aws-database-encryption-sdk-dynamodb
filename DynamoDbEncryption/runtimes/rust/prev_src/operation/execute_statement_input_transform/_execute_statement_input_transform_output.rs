// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ExecuteStatementInputTransformOutput {
    #[allow(missing_docs)] // documentation missing in model
pub transformed_input: ::std::option::Option<dynamodb::types::ExecuteStatementInput>,
}
impl ExecuteStatementInputTransformOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn transformed_input(&self) -> &::std::option::Option<dynamodb::types::ExecuteStatementInput> {
    &self.transformed_input
}
}
impl ExecuteStatementInputTransformOutput {
    /// Creates a new builder-style object to manufacture [`ExecuteStatementInputTransformOutput`](crate::operation::execute_statement_input_transform::builders::ExecuteStatementInputTransformOutput).
    pub fn builder() -> crate::operation::execute_statement_input_transform::builders::ExecuteStatementInputTransformOutputBuilder {
        crate::operation::execute_statement_input_transform::builders::ExecuteStatementInputTransformOutputBuilder::default()
    }
}

/// A builder for [`ExecuteStatementInputTransformOutput`](crate::operation::operation::ExecuteStatementInputTransformOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ExecuteStatementInputTransformOutputBuilder {
    pub(crate) transformed_input: ::std::option::Option<dynamodb::types::ExecuteStatementInput>,
}
impl ExecuteStatementInputTransformOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn transformed_input(mut self, input: impl ::std::convert::Into<dynamodb::types::ExecuteStatementInput>) -> Self {
    self.transformed_input = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_transformed_input(mut self, input: ::std::option::Option<dynamodb::types::ExecuteStatementInput>) -> Self {
    self.transformed_input = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_transformed_input(&self) -> &::std::option::Option<dynamodb::types::ExecuteStatementInput> {
    &self.transformed_input
}
    /// Consumes the builder and constructs a [`ExecuteStatementInputTransformOutput`](crate::operation::operation::ExecuteStatementInputTransformOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::execute_statement_input_transform::ExecuteStatementInputTransformOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::execute_statement_input_transform::ExecuteStatementInputTransformOutput {
            transformed_input: self.transformed_input,
        })
    }
}