// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeTableInput {
    #[allow(missing_docs)] // documentation missing in model
pub table_name: ::std::option::Option<::std::string::String>,
}
impl DescribeTableInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.table_name
}
}
impl DescribeTableInput {
    /// Creates a new builder-style object to manufacture [`DescribeTableInput`](crate::operation::describe_table::builders::DescribeTableInput).
    pub fn builder() -> crate::operation::describe_table::builders::DescribeTableInputBuilder {
        crate::operation::describe_table::builders::DescribeTableInputBuilder::default()
    }
}

/// A builder for [`DescribeTableInput`](crate::operation::operation::DescribeTableInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeTableInputBuilder {
    pub(crate) table_name: ::std::option::Option<::std::string::String>,
}
impl DescribeTableInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.table_name = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.table_name = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.table_name
}
    /// Consumes the builder and constructs a [`DescribeTableInput`](crate::operation::operation::DescribeTableInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_table::DescribeTableInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_table::DescribeTableInput {
            table_name: self.table_name,
        })
    }
}
