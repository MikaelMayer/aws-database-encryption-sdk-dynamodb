// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetKeyRotationStatusResponse {
    #[allow(missing_docs)] // documentation missing in model
pub key_id: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub key_rotation_enabled: ::std::option::Option<::std::primitive::bool>,
#[allow(missing_docs)] // documentation missing in model
pub next_rotation_date: ::std::option::Option<::aws_smithy_types::DateTime>,
#[allow(missing_docs)] // documentation missing in model
pub on_demand_rotation_start_date: ::std::option::Option<::aws_smithy_types::DateTime>,
#[allow(missing_docs)] // documentation missing in model
pub rotation_period_in_days: ::std::option::Option<::std::primitive::i32>,
}
impl GetKeyRotationStatusResponse {
    #[allow(missing_docs)] // documentation missing in model
pub fn key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.key_id
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_rotation_enabled(&self) -> &::std::option::Option<::std::primitive::bool> {
    &self.key_rotation_enabled
}
#[allow(missing_docs)] // documentation missing in model
pub fn next_rotation_date(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
    &self.next_rotation_date
}
#[allow(missing_docs)] // documentation missing in model
pub fn on_demand_rotation_start_date(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
    &self.on_demand_rotation_start_date
}
#[allow(missing_docs)] // documentation missing in model
pub fn rotation_period_in_days(&self) -> &::std::option::Option<::std::primitive::i32> {
    &self.rotation_period_in_days
}
}
impl GetKeyRotationStatusResponse {
    /// Creates a new builder-style object to manufacture [`GetKeyRotationStatusResponse`](crate::operation::get_key_rotation_status::builders::GetKeyRotationStatusResponse).
    pub fn builder() -> crate::operation::get_key_rotation_status::builders::GetKeyRotationStatusResponseBuilder {
        crate::operation::get_key_rotation_status::builders::GetKeyRotationStatusResponseBuilder::default()
    }
}

/// A builder for [`GetKeyRotationStatusResponse`](crate::operation::operation::GetKeyRotationStatusResponse).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetKeyRotationStatusResponseBuilder {
    pub(crate) key_id: ::std::option::Option<::std::string::String>,
pub(crate) key_rotation_enabled: ::std::option::Option<::std::primitive::bool>,
pub(crate) next_rotation_date: ::std::option::Option<::aws_smithy_types::DateTime>,
pub(crate) on_demand_rotation_start_date: ::std::option::Option<::aws_smithy_types::DateTime>,
pub(crate) rotation_period_in_days: ::std::option::Option<::std::primitive::i32>,
}
impl GetKeyRotationStatusResponseBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.key_id = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.key_id = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.key_id
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_rotation_enabled(mut self, input: impl ::std::convert::Into<::std::primitive::bool>) -> Self {
    self.key_rotation_enabled = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_key_rotation_enabled(mut self, input: ::std::option::Option<::std::primitive::bool>) -> Self {
    self.key_rotation_enabled = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_key_rotation_enabled(&self) -> &::std::option::Option<::std::primitive::bool> {
    &self.key_rotation_enabled
}
#[allow(missing_docs)] // documentation missing in model
pub fn next_rotation_date(mut self, input: impl ::std::convert::Into<::aws_smithy_types::DateTime>) -> Self {
    self.next_rotation_date = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_next_rotation_date(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
    self.next_rotation_date = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_next_rotation_date(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
    &self.next_rotation_date
}
#[allow(missing_docs)] // documentation missing in model
pub fn on_demand_rotation_start_date(mut self, input: impl ::std::convert::Into<::aws_smithy_types::DateTime>) -> Self {
    self.on_demand_rotation_start_date = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_on_demand_rotation_start_date(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
    self.on_demand_rotation_start_date = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_on_demand_rotation_start_date(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
    &self.on_demand_rotation_start_date
}
#[allow(missing_docs)] // documentation missing in model
pub fn rotation_period_in_days(mut self, input: impl ::std::convert::Into<::std::primitive::i32>) -> Self {
    self.rotation_period_in_days = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_rotation_period_in_days(mut self, input: ::std::option::Option<::std::primitive::i32>) -> Self {
    self.rotation_period_in_days = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_rotation_period_in_days(&self) -> &::std::option::Option<::std::primitive::i32> {
    &self.rotation_period_in_days
}
    /// Consumes the builder and constructs a [`GetKeyRotationStatusResponse`](crate::operation::operation::GetKeyRotationStatusResponse).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_key_rotation_status::GetKeyRotationStatusResponse,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_key_rotation_status::GetKeyRotationStatusResponse {
            key_id: self.key_id,
key_rotation_enabled: self.key_rotation_enabled,
next_rotation_date: self.next_rotation_date,
on_demand_rotation_start_date: self.on_demand_rotation_start_date,
rotation_period_in_days: self.rotation_period_in_days,
        })
    }
}
