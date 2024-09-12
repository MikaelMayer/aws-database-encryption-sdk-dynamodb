// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeTimeToLiveOutput {
    #[allow(missing_docs)] // documentation missing in model
pub time_to_live_description: ::std::option::Option<dynamodb::types::TimeToLiveDescription>,
}
impl DescribeTimeToLiveOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn time_to_live_description(&self) -> &::std::option::Option<dynamodb::types::TimeToLiveDescription> {
    &self.time_to_live_description
}
}
impl DescribeTimeToLiveOutput {
    /// Creates a new builder-style object to manufacture [`DescribeTimeToLiveOutput`](crate::operation::describe_time_to_live::builders::DescribeTimeToLiveOutput).
    pub fn builder() -> crate::operation::describe_time_to_live::builders::DescribeTimeToLiveOutputBuilder {
        crate::operation::describe_time_to_live::builders::DescribeTimeToLiveOutputBuilder::default()
    }
}

/// A builder for [`DescribeTimeToLiveOutput`](crate::operation::operation::DescribeTimeToLiveOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeTimeToLiveOutputBuilder {
    pub(crate) time_to_live_description: ::std::option::Option<dynamodb::types::TimeToLiveDescription>,
}
impl DescribeTimeToLiveOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn time_to_live_description(mut self, input: impl ::std::convert::Into<dynamodb::types::TimeToLiveDescription>) -> Self {
    self.time_to_live_description = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_time_to_live_description(mut self, input: ::std::option::Option<dynamodb::types::TimeToLiveDescription>) -> Self {
    self.time_to_live_description = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_time_to_live_description(&self) -> &::std::option::Option<dynamodb::types::TimeToLiveDescription> {
    &self.time_to_live_description
}
    /// Consumes the builder and constructs a [`DescribeTimeToLiveOutput`](crate::operation::operation::DescribeTimeToLiveOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_time_to_live::DescribeTimeToLiveOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_time_to_live::DescribeTimeToLiveOutput {
            time_to_live_description: self.time_to_live_description,
        })
    }
}
