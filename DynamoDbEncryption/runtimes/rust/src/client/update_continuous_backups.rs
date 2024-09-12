// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`UpdateContinuousBackups`](crate::operation::update_continuous_backups::builders::UpdateContinuousBackupsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`point_in_time_recovery_specification(impl Into<Option<dynamodb::types::PointInTimeRecoverySpecification>>)`](crate::operation::update_continuous_backups::builders::UpdateContinuousBackupsFluentBuilder::point_in_time_recovery_specification) / [`set_point_in_time_recovery_specification(Option<dynamodb::types::PointInTimeRecoverySpecification>)`](crate::operation::update_continuous_backups::builders::UpdateContinuousBackupsFluentBuilder::set_point_in_time_recovery_specification): (undocumented)<br>
    ///   - [`table_name(impl Into<Option<::std::string::String>>)`](crate::operation::update_continuous_backups::builders::UpdateContinuousBackupsFluentBuilder::table_name) / [`set_table_name(Option<::std::string::String>)`](crate::operation::update_continuous_backups::builders::UpdateContinuousBackupsFluentBuilder::set_table_name): (undocumented)<br>
    /// - On success, responds with [`UpdateContinuousBackupsOutput`](crate::operation::update_continuous_backups::UpdateContinuousBackupsOutput) with field(s):
    ///   - [`continuous_backups_description(Option<dynamodb::types::ContinuousBackupsDescription>)`](crate::operation::update_continuous_backups::UpdateContinuousBackupsOutput::continuous_backups_description): (undocumented)
    /// - On failure, responds with [`SdkError<UpdateContinuousBackupsError>`](crate::operation::update_continuous_backups::UpdateContinuousBackupsError)
    pub fn update_continuous_backups(&self) -> crate::operation::update_continuous_backups::builders::UpdateContinuousBackupsFluentBuilder {
        crate::operation::update_continuous_backups::builders::UpdateContinuousBackupsFluentBuilder::new(self.clone())
    }
}
