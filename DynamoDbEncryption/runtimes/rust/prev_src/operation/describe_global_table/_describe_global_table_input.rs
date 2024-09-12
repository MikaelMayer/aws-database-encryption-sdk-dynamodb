// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeGlobalTableInput {
    #[allow(missing_docs)] // documentation missing in model
pub global_table_name: ::std::option::Option<::std::string::String>,
}
impl DescribeGlobalTableInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn global_table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.global_table_name
}
}
impl DescribeGlobalTableInput {
    /// Creates a new builder-style object to manufacture [`DescribeGlobalTableInput`](crate::operation::describe_global_table::builders::DescribeGlobalTableInput).
    pub fn builder() -> crate::operation::describe_global_table::builders::DescribeGlobalTableInputBuilder {
        crate::operation::describe_global_table::builders::DescribeGlobalTableInputBuilder::default()
    }
}

/// A builder for [`DescribeGlobalTableInput`](crate::operation::operation::DescribeGlobalTableInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeGlobalTableInputBuilder {
    pub(crate) global_table_name: ::std::option::Option<::std::string::String>,
}
impl DescribeGlobalTableInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn global_table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.global_table_name = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_global_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.global_table_name = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_global_table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.global_table_name
}
    /// Consumes the builder and constructs a [`DescribeGlobalTableInput`](crate::operation::operation::DescribeGlobalTableInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_global_table::DescribeGlobalTableInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_global_table::DescribeGlobalTableInput {
            global_table_name: self.global_table_name,
        })
    }
}
