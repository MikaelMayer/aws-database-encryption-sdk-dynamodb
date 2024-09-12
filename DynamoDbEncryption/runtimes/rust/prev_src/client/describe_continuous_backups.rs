// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`DescribeContinuousBackups`](crate::operation::describe_continuous_backups::builders::DescribeContinuousBackupsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`table_name(impl Into<Option<::std::string::String>>)`](crate::operation::describe_continuous_backups::builders::DescribeContinuousBackupsFluentBuilder::table_name) / [`set_table_name(Option<::std::string::String>)`](crate::operation::describe_continuous_backups::builders::DescribeContinuousBackupsFluentBuilder::set_table_name): (undocumented)<br>
    /// - On success, responds with [`DescribeContinuousBackupsOutput`](crate::operation::describe_continuous_backups::DescribeContinuousBackupsOutput) with field(s):
    ///   - [`continuous_backups_description(Option<dynamodb::types::ContinuousBackupsDescription>)`](crate::operation::describe_continuous_backups::DescribeContinuousBackupsOutput::continuous_backups_description): (undocumented)
    /// - On failure, responds with [`SdkError<DescribeContinuousBackupsError>`](crate::operation::describe_continuous_backups::DescribeContinuousBackupsError)
    pub fn describe_continuous_backups(&self) -> crate::operation::describe_continuous_backups::builders::DescribeContinuousBackupsFluentBuilder {
        crate::operation::describe_continuous_backups::builders::DescribeContinuousBackupsFluentBuilder::new(self.clone())
    }
}
