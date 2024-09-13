// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct QueryOutputTransformOutput {
    #[allow(missing_docs)] // documentation missing in model
pub transformed_output: ::std::option::Option<dynamodb::types::QueryOutput>,
}
impl QueryOutputTransformOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn transformed_output(&self) -> &::std::option::Option<dynamodb::types::QueryOutput> {
    &self.transformed_output
}
}
impl QueryOutputTransformOutput {
    /// Creates a new builder-style object to manufacture [`QueryOutputTransformOutput`](crate::operation::query_output_transform::builders::QueryOutputTransformOutput).
    pub fn builder() -> crate::operation::query_output_transform::builders::QueryOutputTransformOutputBuilder {
        crate::operation::query_output_transform::builders::QueryOutputTransformOutputBuilder::default()
    }
}

/// A builder for [`QueryOutputTransformOutput`](crate::operation::operation::QueryOutputTransformOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct QueryOutputTransformOutputBuilder {
    pub(crate) transformed_output: ::std::option::Option<dynamodb::types::QueryOutput>,
}
impl QueryOutputTransformOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn transformed_output(mut self, input: impl ::std::convert::Into<dynamodb::types::QueryOutput>) -> Self {
    self.transformed_output = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_transformed_output(mut self, input: ::std::option::Option<dynamodb::types::QueryOutput>) -> Self {
    self.transformed_output = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_transformed_output(&self) -> &::std::option::Option<dynamodb::types::QueryOutput> {
    &self.transformed_output
}
    /// Consumes the builder and constructs a [`QueryOutputTransformOutput`](crate::operation::operation::QueryOutputTransformOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::query_output_transform::QueryOutputTransformOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::query_output_transform::QueryOutputTransformOutput {
            transformed_output: self.transformed_output,
        })
    }
}