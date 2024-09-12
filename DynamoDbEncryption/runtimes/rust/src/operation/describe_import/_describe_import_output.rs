// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeImportOutput {
    #[allow(missing_docs)] // documentation missing in model
pub import_table_description: ::std::option::Option<dynamodb::types::ImportTableDescription>,
}
impl DescribeImportOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn import_table_description(&self) -> &::std::option::Option<dynamodb::types::ImportTableDescription> {
    &self.import_table_description
}
}
impl DescribeImportOutput {
    /// Creates a new builder-style object to manufacture [`DescribeImportOutput`](crate::operation::describe_import::builders::DescribeImportOutput).
    pub fn builder() -> crate::operation::describe_import::builders::DescribeImportOutputBuilder {
        crate::operation::describe_import::builders::DescribeImportOutputBuilder::default()
    }
}

/// A builder for [`DescribeImportOutput`](crate::operation::operation::DescribeImportOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeImportOutputBuilder {
    pub(crate) import_table_description: ::std::option::Option<dynamodb::types::ImportTableDescription>,
}
impl DescribeImportOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn import_table_description(mut self, input: impl ::std::convert::Into<dynamodb::types::ImportTableDescription>) -> Self {
    self.import_table_description = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_import_table_description(mut self, input: ::std::option::Option<dynamodb::types::ImportTableDescription>) -> Self {
    self.import_table_description = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_import_table_description(&self) -> &::std::option::Option<dynamodb::types::ImportTableDescription> {
    &self.import_table_description
}
    /// Consumes the builder and constructs a [`DescribeImportOutput`](crate::operation::operation::DescribeImportOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_import::DescribeImportOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_import::DescribeImportOutput {
            import_table_description: self.import_table_description,
        })
    }
}
