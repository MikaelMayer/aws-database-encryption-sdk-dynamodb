// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TransactWriteItemsOutputTransformOutput {
    #[allow(missing_docs)] // documentation missing in model
pub transformed_output: ::std::option::Option<dynamodb::types::TransactWriteItemsOutput>,
}
impl TransactWriteItemsOutputTransformOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn transformed_output(&self) -> &::std::option::Option<dynamodb::types::TransactWriteItemsOutput> {
    &self.transformed_output
}
}
impl TransactWriteItemsOutputTransformOutput {
    /// Creates a new builder-style object to manufacture [`TransactWriteItemsOutputTransformOutput`](crate::operation::transact_write_items_output_transform::builders::TransactWriteItemsOutputTransformOutput).
    pub fn builder() -> crate::operation::transact_write_items_output_transform::builders::TransactWriteItemsOutputTransformOutputBuilder {
        crate::operation::transact_write_items_output_transform::builders::TransactWriteItemsOutputTransformOutputBuilder::default()
    }
}

/// A builder for [`TransactWriteItemsOutputTransformOutput`](crate::operation::operation::TransactWriteItemsOutputTransformOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct TransactWriteItemsOutputTransformOutputBuilder {
    pub(crate) transformed_output: ::std::option::Option<dynamodb::types::TransactWriteItemsOutput>,
}
impl TransactWriteItemsOutputTransformOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn transformed_output(mut self, input: impl ::std::convert::Into<dynamodb::types::TransactWriteItemsOutput>) -> Self {
    self.transformed_output = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_transformed_output(mut self, input: ::std::option::Option<dynamodb::types::TransactWriteItemsOutput>) -> Self {
    self.transformed_output = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_transformed_output(&self) -> &::std::option::Option<dynamodb::types::TransactWriteItemsOutput> {
    &self.transformed_output
}
    /// Consumes the builder and constructs a [`TransactWriteItemsOutputTransformOutput`](crate::operation::operation::TransactWriteItemsOutputTransformOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::transact_write_items_output_transform::TransactWriteItemsOutputTransformOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::transact_write_items_output_transform::TransactWriteItemsOutputTransformOutput {
            transformed_output: self.transformed_output,
        })
    }
}