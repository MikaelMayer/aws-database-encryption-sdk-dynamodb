// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateTimeToLiveOutput {
    #[allow(missing_docs)] // documentation missing in model
pub time_to_live_specification: ::std::option::Option<dynamodb::types::TimeToLiveSpecification>,
}
impl UpdateTimeToLiveOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn time_to_live_specification(&self) -> &::std::option::Option<dynamodb::types::TimeToLiveSpecification> {
    &self.time_to_live_specification
}
}
impl UpdateTimeToLiveOutput {
    /// Creates a new builder-style object to manufacture [`UpdateTimeToLiveOutput`](crate::operation::update_time_to_live::builders::UpdateTimeToLiveOutput).
    pub fn builder() -> crate::operation::update_time_to_live::builders::UpdateTimeToLiveOutputBuilder {
        crate::operation::update_time_to_live::builders::UpdateTimeToLiveOutputBuilder::default()
    }
}

/// A builder for [`UpdateTimeToLiveOutput`](crate::operation::operation::UpdateTimeToLiveOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateTimeToLiveOutputBuilder {
    pub(crate) time_to_live_specification: ::std::option::Option<dynamodb::types::TimeToLiveSpecification>,
}
impl UpdateTimeToLiveOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn time_to_live_specification(mut self, input: impl ::std::convert::Into<dynamodb::types::TimeToLiveSpecification>) -> Self {
    self.time_to_live_specification = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_time_to_live_specification(mut self, input: ::std::option::Option<dynamodb::types::TimeToLiveSpecification>) -> Self {
    self.time_to_live_specification = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_time_to_live_specification(&self) -> &::std::option::Option<dynamodb::types::TimeToLiveSpecification> {
    &self.time_to_live_specification
}
    /// Consumes the builder and constructs a [`UpdateTimeToLiveOutput`](crate::operation::operation::UpdateTimeToLiveOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_time_to_live::UpdateTimeToLiveOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::update_time_to_live::UpdateTimeToLiveOutput {
            time_to_live_specification: self.time_to_live_specification,
        })
    }
}
