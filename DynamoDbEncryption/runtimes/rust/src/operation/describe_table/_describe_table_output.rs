// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeTableOutput {
    #[allow(missing_docs)] // documentation missing in model
pub table: ::std::option::Option<dynamodb::types::TableDescription>,
}
impl DescribeTableOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn table(&self) -> &::std::option::Option<dynamodb::types::TableDescription> {
    &self.table
}
}
impl DescribeTableOutput {
    /// Creates a new builder-style object to manufacture [`DescribeTableOutput`](crate::operation::describe_table::builders::DescribeTableOutput).
    pub fn builder() -> crate::operation::describe_table::builders::DescribeTableOutputBuilder {
        crate::operation::describe_table::builders::DescribeTableOutputBuilder::default()
    }
}

/// A builder for [`DescribeTableOutput`](crate::operation::operation::DescribeTableOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeTableOutputBuilder {
    pub(crate) table: ::std::option::Option<dynamodb::types::TableDescription>,
}
impl DescribeTableOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn table(mut self, input: impl ::std::convert::Into<dynamodb::types::TableDescription>) -> Self {
    self.table = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_table(mut self, input: ::std::option::Option<dynamodb::types::TableDescription>) -> Self {
    self.table = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_table(&self) -> &::std::option::Option<dynamodb::types::TableDescription> {
    &self.table
}
    /// Consumes the builder and constructs a [`DescribeTableOutput`](crate::operation::operation::DescribeTableOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_table::DescribeTableOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_table::DescribeTableOutput {
            table: self.table,
        })
    }
}
