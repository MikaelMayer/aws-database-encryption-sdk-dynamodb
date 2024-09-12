// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeContinuousBackupsOutput {
    #[allow(missing_docs)] // documentation missing in model
pub continuous_backups_description: ::std::option::Option<dynamodb::types::ContinuousBackupsDescription>,
}
impl DescribeContinuousBackupsOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn continuous_backups_description(&self) -> &::std::option::Option<dynamodb::types::ContinuousBackupsDescription> {
    &self.continuous_backups_description
}
}
impl DescribeContinuousBackupsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeContinuousBackupsOutput`](crate::operation::describe_continuous_backups::builders::DescribeContinuousBackupsOutput).
    pub fn builder() -> crate::operation::describe_continuous_backups::builders::DescribeContinuousBackupsOutputBuilder {
        crate::operation::describe_continuous_backups::builders::DescribeContinuousBackupsOutputBuilder::default()
    }
}

/// A builder for [`DescribeContinuousBackupsOutput`](crate::operation::operation::DescribeContinuousBackupsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeContinuousBackupsOutputBuilder {
    pub(crate) continuous_backups_description: ::std::option::Option<dynamodb::types::ContinuousBackupsDescription>,
}
impl DescribeContinuousBackupsOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn continuous_backups_description(mut self, input: impl ::std::convert::Into<dynamodb::types::ContinuousBackupsDescription>) -> Self {
    self.continuous_backups_description = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_continuous_backups_description(mut self, input: ::std::option::Option<dynamodb::types::ContinuousBackupsDescription>) -> Self {
    self.continuous_backups_description = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_continuous_backups_description(&self) -> &::std::option::Option<dynamodb::types::ContinuousBackupsDescription> {
    &self.continuous_backups_description
}
    /// Consumes the builder and constructs a [`DescribeContinuousBackupsOutput`](crate::operation::operation::DescribeContinuousBackupsOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_continuous_backups::DescribeContinuousBackupsOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_continuous_backups::DescribeContinuousBackupsOutput {
            continuous_backups_description: self.continuous_backups_description,
        })
    }
}
