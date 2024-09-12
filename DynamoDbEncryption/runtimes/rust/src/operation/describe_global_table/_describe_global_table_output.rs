// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeGlobalTableOutput {
    #[allow(missing_docs)] // documentation missing in model
pub global_table_description: ::std::option::Option<dynamodb::types::GlobalTableDescription>,
}
impl DescribeGlobalTableOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn global_table_description(&self) -> &::std::option::Option<dynamodb::types::GlobalTableDescription> {
    &self.global_table_description
}
}
impl DescribeGlobalTableOutput {
    /// Creates a new builder-style object to manufacture [`DescribeGlobalTableOutput`](crate::operation::describe_global_table::builders::DescribeGlobalTableOutput).
    pub fn builder() -> crate::operation::describe_global_table::builders::DescribeGlobalTableOutputBuilder {
        crate::operation::describe_global_table::builders::DescribeGlobalTableOutputBuilder::default()
    }
}

/// A builder for [`DescribeGlobalTableOutput`](crate::operation::operation::DescribeGlobalTableOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeGlobalTableOutputBuilder {
    pub(crate) global_table_description: ::std::option::Option<dynamodb::types::GlobalTableDescription>,
}
impl DescribeGlobalTableOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn global_table_description(mut self, input: impl ::std::convert::Into<dynamodb::types::GlobalTableDescription>) -> Self {
    self.global_table_description = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_global_table_description(mut self, input: ::std::option::Option<dynamodb::types::GlobalTableDescription>) -> Self {
    self.global_table_description = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_global_table_description(&self) -> &::std::option::Option<dynamodb::types::GlobalTableDescription> {
    &self.global_table_description
}
    /// Consumes the builder and constructs a [`DescribeGlobalTableOutput`](crate::operation::operation::DescribeGlobalTableOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_global_table::DescribeGlobalTableOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_global_table::DescribeGlobalTableOutput {
            global_table_description: self.global_table_description,
        })
    }
}
