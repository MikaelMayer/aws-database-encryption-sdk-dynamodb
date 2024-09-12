// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeContinuousBackupsInput {
    #[allow(missing_docs)] // documentation missing in model
pub table_name: ::std::option::Option<::std::string::String>,
}
impl DescribeContinuousBackupsInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.table_name
}
}
impl DescribeContinuousBackupsInput {
    /// Creates a new builder-style object to manufacture [`DescribeContinuousBackupsInput`](crate::operation::describe_continuous_backups::builders::DescribeContinuousBackupsInput).
    pub fn builder() -> crate::operation::describe_continuous_backups::builders::DescribeContinuousBackupsInputBuilder {
        crate::operation::describe_continuous_backups::builders::DescribeContinuousBackupsInputBuilder::default()
    }
}

/// A builder for [`DescribeContinuousBackupsInput`](crate::operation::operation::DescribeContinuousBackupsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeContinuousBackupsInputBuilder {
    pub(crate) table_name: ::std::option::Option<::std::string::String>,
}
impl DescribeContinuousBackupsInputBuilder {
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
    /// Consumes the builder and constructs a [`DescribeContinuousBackupsInput`](crate::operation::operation::DescribeContinuousBackupsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_continuous_backups::DescribeContinuousBackupsInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_continuous_backups::DescribeContinuousBackupsInput {
            table_name: self.table_name,
        })
    }
}
