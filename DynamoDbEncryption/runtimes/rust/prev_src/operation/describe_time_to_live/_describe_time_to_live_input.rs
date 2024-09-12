// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeTimeToLiveInput {
    #[allow(missing_docs)] // documentation missing in model
pub table_name: ::std::option::Option<::std::string::String>,
}
impl DescribeTimeToLiveInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.table_name
}
}
impl DescribeTimeToLiveInput {
    /// Creates a new builder-style object to manufacture [`DescribeTimeToLiveInput`](crate::operation::describe_time_to_live::builders::DescribeTimeToLiveInput).
    pub fn builder() -> crate::operation::describe_time_to_live::builders::DescribeTimeToLiveInputBuilder {
        crate::operation::describe_time_to_live::builders::DescribeTimeToLiveInputBuilder::default()
    }
}

/// A builder for [`DescribeTimeToLiveInput`](crate::operation::operation::DescribeTimeToLiveInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeTimeToLiveInputBuilder {
    pub(crate) table_name: ::std::option::Option<::std::string::String>,
}
impl DescribeTimeToLiveInputBuilder {
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
    /// Consumes the builder and constructs a [`DescribeTimeToLiveInput`](crate::operation::operation::DescribeTimeToLiveInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_time_to_live::DescribeTimeToLiveInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_time_to_live::DescribeTimeToLiveInput {
            table_name: self.table_name,
        })
    }
}
