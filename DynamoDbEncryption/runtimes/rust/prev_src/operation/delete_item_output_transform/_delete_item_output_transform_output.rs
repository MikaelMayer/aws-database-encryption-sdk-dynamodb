// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteItemOutputTransformOutput {
    #[allow(missing_docs)] // documentation missing in model
pub transformed_output: ::std::option::Option<dynamodb::types::DeleteItemOutput>,
}
impl DeleteItemOutputTransformOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn transformed_output(&self) -> &::std::option::Option<dynamodb::types::DeleteItemOutput> {
    &self.transformed_output
}
}
impl DeleteItemOutputTransformOutput {
    /// Creates a new builder-style object to manufacture [`DeleteItemOutputTransformOutput`](crate::operation::delete_item_output_transform::builders::DeleteItemOutputTransformOutput).
    pub fn builder() -> crate::operation::delete_item_output_transform::builders::DeleteItemOutputTransformOutputBuilder {
        crate::operation::delete_item_output_transform::builders::DeleteItemOutputTransformOutputBuilder::default()
    }
}

/// A builder for [`DeleteItemOutputTransformOutput`](crate::operation::operation::DeleteItemOutputTransformOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteItemOutputTransformOutputBuilder {
    pub(crate) transformed_output: ::std::option::Option<dynamodb::types::DeleteItemOutput>,
}
impl DeleteItemOutputTransformOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn transformed_output(mut self, input: impl ::std::convert::Into<dynamodb::types::DeleteItemOutput>) -> Self {
    self.transformed_output = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_transformed_output(mut self, input: ::std::option::Option<dynamodb::types::DeleteItemOutput>) -> Self {
    self.transformed_output = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_transformed_output(&self) -> &::std::option::Option<dynamodb::types::DeleteItemOutput> {
    &self.transformed_output
}
    /// Consumes the builder and constructs a [`DeleteItemOutputTransformOutput`](crate::operation::operation::DeleteItemOutputTransformOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_item_output_transform::DeleteItemOutputTransformOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::delete_item_output_transform::DeleteItemOutputTransformOutput {
            transformed_output: self.transformed_output,
        })
    }
}