// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeExportOutput {
    #[allow(missing_docs)] // documentation missing in model
pub export_description: ::std::option::Option<dynamodb::types::ExportDescription>,
}
impl DescribeExportOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn export_description(&self) -> &::std::option::Option<dynamodb::types::ExportDescription> {
    &self.export_description
}
}
impl DescribeExportOutput {
    /// Creates a new builder-style object to manufacture [`DescribeExportOutput`](crate::operation::describe_export::builders::DescribeExportOutput).
    pub fn builder() -> crate::operation::describe_export::builders::DescribeExportOutputBuilder {
        crate::operation::describe_export::builders::DescribeExportOutputBuilder::default()
    }
}

/// A builder for [`DescribeExportOutput`](crate::operation::operation::DescribeExportOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeExportOutputBuilder {
    pub(crate) export_description: ::std::option::Option<dynamodb::types::ExportDescription>,
}
impl DescribeExportOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn export_description(mut self, input: impl ::std::convert::Into<dynamodb::types::ExportDescription>) -> Self {
    self.export_description = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_export_description(mut self, input: ::std::option::Option<dynamodb::types::ExportDescription>) -> Self {
    self.export_description = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_export_description(&self) -> &::std::option::Option<dynamodb::types::ExportDescription> {
    &self.export_description
}
    /// Consumes the builder and constructs a [`DescribeExportOutput`](crate::operation::operation::DescribeExportOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_export::DescribeExportOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_export::DescribeExportOutput {
            export_description: self.export_description,
        })
    }
}
