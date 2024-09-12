// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateContinuousBackupsInput {
    #[allow(missing_docs)] // documentation missing in model
pub point_in_time_recovery_specification: ::std::option::Option<dynamodb::types::PointInTimeRecoverySpecification>,
#[allow(missing_docs)] // documentation missing in model
pub table_name: ::std::option::Option<::std::string::String>,
}
impl UpdateContinuousBackupsInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn point_in_time_recovery_specification(&self) -> &::std::option::Option<dynamodb::types::PointInTimeRecoverySpecification> {
    &self.point_in_time_recovery_specification
}
#[allow(missing_docs)] // documentation missing in model
pub fn table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.table_name
}
}
impl UpdateContinuousBackupsInput {
    /// Creates a new builder-style object to manufacture [`UpdateContinuousBackupsInput`](crate::operation::update_continuous_backups::builders::UpdateContinuousBackupsInput).
    pub fn builder() -> crate::operation::update_continuous_backups::builders::UpdateContinuousBackupsInputBuilder {
        crate::operation::update_continuous_backups::builders::UpdateContinuousBackupsInputBuilder::default()
    }
}

/// A builder for [`UpdateContinuousBackupsInput`](crate::operation::operation::UpdateContinuousBackupsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateContinuousBackupsInputBuilder {
    pub(crate) point_in_time_recovery_specification: ::std::option::Option<dynamodb::types::PointInTimeRecoverySpecification>,
pub(crate) table_name: ::std::option::Option<::std::string::String>,
}
impl UpdateContinuousBackupsInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn point_in_time_recovery_specification(mut self, input: impl ::std::convert::Into<dynamodb::types::PointInTimeRecoverySpecification>) -> Self {
    self.point_in_time_recovery_specification = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_point_in_time_recovery_specification(mut self, input: ::std::option::Option<dynamodb::types::PointInTimeRecoverySpecification>) -> Self {
    self.point_in_time_recovery_specification = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_point_in_time_recovery_specification(&self) -> &::std::option::Option<dynamodb::types::PointInTimeRecoverySpecification> {
    &self.point_in_time_recovery_specification
}
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
    /// Consumes the builder and constructs a [`UpdateContinuousBackupsInput`](crate::operation::operation::UpdateContinuousBackupsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_continuous_backups::UpdateContinuousBackupsInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::update_continuous_backups::UpdateContinuousBackupsInput {
            point_in_time_recovery_specification: self.point_in_time_recovery_specification,
table_name: self.table_name,
        })
    }
}
