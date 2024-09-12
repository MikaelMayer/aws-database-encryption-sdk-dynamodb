// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`ListBackups`](crate::operation::list_backups::builders::ListBackupsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`backup_type(impl Into<Option<dynamodb::types::BackupTypeFilter>>)`](crate::operation::list_backups::builders::ListBackupsFluentBuilder::backup_type) / [`set_backup_type(Option<dynamodb::types::BackupTypeFilter>)`](crate::operation::list_backups::builders::ListBackupsFluentBuilder::set_backup_type): (undocumented)<br>
    ///   - [`exclusive_start_backup_arn(impl Into<Option<::std::string::String>>)`](crate::operation::list_backups::builders::ListBackupsFluentBuilder::exclusive_start_backup_arn) / [`set_exclusive_start_backup_arn(Option<::std::string::String>)`](crate::operation::list_backups::builders::ListBackupsFluentBuilder::set_exclusive_start_backup_arn): (undocumented)<br>
    ///   - [`limit(impl Into<Option<::std::primitive::i32>>)`](crate::operation::list_backups::builders::ListBackupsFluentBuilder::limit) / [`set_limit(Option<::std::primitive::i32>)`](crate::operation::list_backups::builders::ListBackupsFluentBuilder::set_limit): (undocumented)<br>
    ///   - [`table_name(impl Into<Option<::std::string::String>>)`](crate::operation::list_backups::builders::ListBackupsFluentBuilder::table_name) / [`set_table_name(Option<::std::string::String>)`](crate::operation::list_backups::builders::ListBackupsFluentBuilder::set_table_name): (undocumented)<br>
    ///   - [`time_range_lower_bound(impl Into<Option<::aws_smithy_types::DateTime>>)`](crate::operation::list_backups::builders::ListBackupsFluentBuilder::time_range_lower_bound) / [`set_time_range_lower_bound(Option<::aws_smithy_types::DateTime>)`](crate::operation::list_backups::builders::ListBackupsFluentBuilder::set_time_range_lower_bound): (undocumented)<br>
    ///   - [`time_range_upper_bound(impl Into<Option<::aws_smithy_types::DateTime>>)`](crate::operation::list_backups::builders::ListBackupsFluentBuilder::time_range_upper_bound) / [`set_time_range_upper_bound(Option<::aws_smithy_types::DateTime>)`](crate::operation::list_backups::builders::ListBackupsFluentBuilder::set_time_range_upper_bound): (undocumented)<br>
    /// - On success, responds with [`ListBackupsOutput`](crate::operation::list_backups::ListBackupsOutput) with field(s):
    ///   - [`backup_summaries(Option<::std::vec::Vec<dynamodb::types::BackupSummary>>)`](crate::operation::list_backups::ListBackupsOutput::backup_summaries): (undocumented)
    ///   - [`last_evaluated_backup_arn(Option<::std::string::String>)`](crate::operation::list_backups::ListBackupsOutput::last_evaluated_backup_arn): (undocumented)
    /// - On failure, responds with [`SdkError<ListBackupsError>`](crate::operation::list_backups::ListBackupsError)
    pub fn list_backups(&self) -> crate::operation::list_backups::builders::ListBackupsFluentBuilder {
        crate::operation::list_backups::builders::ListBackupsFluentBuilder::new(self.clone())
    }
}
