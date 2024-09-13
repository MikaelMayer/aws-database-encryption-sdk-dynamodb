// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct QueryOutputTransformInput {
    #[allow(missing_docs)] // documentation missing in model
pub original_input: ::std::option::Option<dynamodb::types::QueryInput>,
#[allow(missing_docs)] // documentation missing in model
pub sdk_output: ::std::option::Option<dynamodb::types::QueryOutput>,
}
impl QueryOutputTransformInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn original_input(&self) -> &::std::option::Option<dynamodb::types::QueryInput> {
    &self.original_input
}
#[allow(missing_docs)] // documentation missing in model
pub fn sdk_output(&self) -> &::std::option::Option<dynamodb::types::QueryOutput> {
    &self.sdk_output
}
}
impl QueryOutputTransformInput {
    /// Creates a new builder-style object to manufacture [`QueryOutputTransformInput`](crate::operation::query_output_transform::builders::QueryOutputTransformInput).
    pub fn builder() -> crate::operation::query_output_transform::builders::QueryOutputTransformInputBuilder {
        crate::operation::query_output_transform::builders::QueryOutputTransformInputBuilder::default()
    }
}

/// A builder for [`QueryOutputTransformInput`](crate::operation::operation::QueryOutputTransformInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct QueryOutputTransformInputBuilder {
    pub(crate) original_input: ::std::option::Option<dynamodb::types::QueryInput>,
pub(crate) sdk_output: ::std::option::Option<dynamodb::types::QueryOutput>,
}
impl QueryOutputTransformInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn original_input(mut self, input: impl ::std::convert::Into<dynamodb::types::QueryInput>) -> Self {
    self.original_input = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_original_input(mut self, input: ::std::option::Option<dynamodb::types::QueryInput>) -> Self {
    self.original_input = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_original_input(&self) -> &::std::option::Option<dynamodb::types::QueryInput> {
    &self.original_input
}
#[allow(missing_docs)] // documentation missing in model
pub fn sdk_output(mut self, input: impl ::std::convert::Into<dynamodb::types::QueryOutput>) -> Self {
    self.sdk_output = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_sdk_output(mut self, input: ::std::option::Option<dynamodb::types::QueryOutput>) -> Self {
    self.sdk_output = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_sdk_output(&self) -> &::std::option::Option<dynamodb::types::QueryOutput> {
    &self.sdk_output
}
    /// Consumes the builder and constructs a [`QueryOutputTransformInput`](crate::operation::operation::QueryOutputTransformInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::query_output_transform::QueryOutputTransformInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::query_output_transform::QueryOutputTransformInput {
            original_input: self.original_input,
sdk_output: self.sdk_output,
        })
    }
}