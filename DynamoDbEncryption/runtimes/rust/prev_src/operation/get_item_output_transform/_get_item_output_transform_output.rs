// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetItemOutputTransformOutput {
    #[allow(missing_docs)] // documentation missing in model
pub transformed_output: ::std::option::Option<dynamodb::types::GetItemOutput>,
}
impl GetItemOutputTransformOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn transformed_output(&self) -> &::std::option::Option<dynamodb::types::GetItemOutput> {
    &self.transformed_output
}
}
impl GetItemOutputTransformOutput {
    /// Creates a new builder-style object to manufacture [`GetItemOutputTransformOutput`](crate::operation::get_item_output_transform::builders::GetItemOutputTransformOutput).
    pub fn builder() -> crate::operation::get_item_output_transform::builders::GetItemOutputTransformOutputBuilder {
        crate::operation::get_item_output_transform::builders::GetItemOutputTransformOutputBuilder::default()
    }
}

/// A builder for [`GetItemOutputTransformOutput`](crate::operation::operation::GetItemOutputTransformOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetItemOutputTransformOutputBuilder {
    pub(crate) transformed_output: ::std::option::Option<dynamodb::types::GetItemOutput>,
}
impl GetItemOutputTransformOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn transformed_output(mut self, input: impl ::std::convert::Into<dynamodb::types::GetItemOutput>) -> Self {
    self.transformed_output = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_transformed_output(mut self, input: ::std::option::Option<dynamodb::types::GetItemOutput>) -> Self {
    self.transformed_output = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_transformed_output(&self) -> &::std::option::Option<dynamodb::types::GetItemOutput> {
    &self.transformed_output
}
    /// Consumes the builder and constructs a [`GetItemOutputTransformOutput`](crate::operation::operation::GetItemOutputTransformOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_item_output_transform::GetItemOutputTransformOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_item_output_transform::GetItemOutputTransformOutput {
            transformed_output: self.transformed_output,
        })
    }
}