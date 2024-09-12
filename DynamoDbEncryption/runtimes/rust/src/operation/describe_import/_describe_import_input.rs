// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeImportInput {
    #[allow(missing_docs)] // documentation missing in model
pub import_arn: ::std::option::Option<::std::string::String>,
}
impl DescribeImportInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn import_arn(&self) -> &::std::option::Option<::std::string::String> {
    &self.import_arn
}
}
impl DescribeImportInput {
    /// Creates a new builder-style object to manufacture [`DescribeImportInput`](crate::operation::describe_import::builders::DescribeImportInput).
    pub fn builder() -> crate::operation::describe_import::builders::DescribeImportInputBuilder {
        crate::operation::describe_import::builders::DescribeImportInputBuilder::default()
    }
}

/// A builder for [`DescribeImportInput`](crate::operation::operation::DescribeImportInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeImportInputBuilder {
    pub(crate) import_arn: ::std::option::Option<::std::string::String>,
}
impl DescribeImportInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn import_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.import_arn = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_import_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.import_arn = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_import_arn(&self) -> &::std::option::Option<::std::string::String> {
    &self.import_arn
}
    /// Consumes the builder and constructs a [`DescribeImportInput`](crate::operation::operation::DescribeImportInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_import::DescribeImportInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_import::DescribeImportInput {
            import_arn: self.import_arn,
        })
    }
}
