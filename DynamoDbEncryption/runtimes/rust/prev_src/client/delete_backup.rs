// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`DeleteBackup`](crate::operation::delete_backup::builders::DeleteBackupFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`backup_arn(impl Into<Option<::std::string::String>>)`](crate::operation::delete_backup::builders::DeleteBackupFluentBuilder::backup_arn) / [`set_backup_arn(Option<::std::string::String>)`](crate::operation::delete_backup::builders::DeleteBackupFluentBuilder::set_backup_arn): (undocumented)<br>
    /// - On success, responds with [`DeleteBackupOutput`](crate::operation::delete_backup::DeleteBackupOutput) with field(s):
    ///   - [`backup_description(Option<dynamodb::types::BackupDescription>)`](crate::operation::delete_backup::DeleteBackupOutput::backup_description): (undocumented)
    /// - On failure, responds with [`SdkError<DeleteBackupError>`](crate::operation::delete_backup::DeleteBackupError)
    pub fn delete_backup(&self) -> crate::operation::delete_backup::builders::DeleteBackupFluentBuilder {
        crate::operation::delete_backup::builders::DeleteBackupFluentBuilder::new(self.clone())
    }
}
