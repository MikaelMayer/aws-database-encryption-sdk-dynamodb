// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`DescribeBackup`](crate::operation::describe_backup::builders::DescribeBackupFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`backup_arn(impl Into<Option<::std::string::String>>)`](crate::operation::describe_backup::builders::DescribeBackupFluentBuilder::backup_arn) / [`set_backup_arn(Option<::std::string::String>)`](crate::operation::describe_backup::builders::DescribeBackupFluentBuilder::set_backup_arn): (undocumented)<br>
    /// - On success, responds with [`DescribeBackupOutput`](crate::operation::describe_backup::DescribeBackupOutput) with field(s):
    ///   - [`backup_description(Option<dynamodb::types::BackupDescription>)`](crate::operation::describe_backup::DescribeBackupOutput::backup_description): (undocumented)
    /// - On failure, responds with [`SdkError<DescribeBackupError>`](crate::operation::describe_backup::DescribeBackupError)
    pub fn describe_backup(&self) -> crate::operation::describe_backup::builders::DescribeBackupFluentBuilder {
        crate::operation::describe_backup::builders::DescribeBackupFluentBuilder::new(self.clone())
    }
}
