// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateTimeToLiveInput {
    #[allow(missing_docs)] // documentation missing in model
pub table_name: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub time_to_live_specification: ::std::option::Option<dynamodb::types::TimeToLiveSpecification>,
}
impl UpdateTimeToLiveInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.table_name
}
#[allow(missing_docs)] // documentation missing in model
pub fn time_to_live_specification(&self) -> &::std::option::Option<dynamodb::types::TimeToLiveSpecification> {
    &self.time_to_live_specification
}
}
impl UpdateTimeToLiveInput {
    /// Creates a new builder-style object to manufacture [`UpdateTimeToLiveInput`](crate::operation::update_time_to_live::builders::UpdateTimeToLiveInput).
    pub fn builder() -> crate::operation::update_time_to_live::builders::UpdateTimeToLiveInputBuilder {
        crate::operation::update_time_to_live::builders::UpdateTimeToLiveInputBuilder::default()
    }
}

/// A builder for [`UpdateTimeToLiveInput`](crate::operation::operation::UpdateTimeToLiveInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateTimeToLiveInputBuilder {
    pub(crate) table_name: ::std::option::Option<::std::string::String>,
pub(crate) time_to_live_specification: ::std::option::Option<dynamodb::types::TimeToLiveSpecification>,
}
impl UpdateTimeToLiveInputBuilder {
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
    /// Consumes the builder and constructs a [`UpdateTimeToLiveInput`](crate::operation::operation::UpdateTimeToLiveInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_time_to_live::UpdateTimeToLiveInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::update_time_to_live::UpdateTimeToLiveInput {
            table_name: self.table_name,
time_to_live_specification: self.time_to_live_specification,
        })
    }
}
