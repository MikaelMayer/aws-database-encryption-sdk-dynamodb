// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeExportInput {
    #[allow(missing_docs)] // documentation missing in model
pub export_arn: ::std::option::Option<::std::string::String>,
}
impl DescribeExportInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn export_arn(&self) -> &::std::option::Option<::std::string::String> {
    &self.export_arn
}
}
impl DescribeExportInput {
    /// Creates a new builder-style object to manufacture [`DescribeExportInput`](crate::operation::describe_export::builders::DescribeExportInput).
    pub fn builder() -> crate::operation::describe_export::builders::DescribeExportInputBuilder {
        crate::operation::describe_export::builders::DescribeExportInputBuilder::default()
    }
}

/// A builder for [`DescribeExportInput`](crate::operation::operation::DescribeExportInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeExportInputBuilder {
    pub(crate) export_arn: ::std::option::Option<::std::string::String>,
}
impl DescribeExportInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn export_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.export_arn = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_export_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.export_arn = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_export_arn(&self) -> &::std::option::Option<::std::string::String> {
    &self.export_arn
}
    /// Consumes the builder and constructs a [`DescribeExportInput`](crate::operation::operation::DescribeExportInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_export::DescribeExportInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_export::DescribeExportInput {
            export_arn: self.export_arn,
        })
    }
}
