// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`CreateBackup`](crate::operation::create_backup::builders::CreateBackupFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`backup_name(impl Into<Option<::std::string::String>>)`](crate::operation::create_backup::builders::CreateBackupFluentBuilder::backup_name) / [`set_backup_name(Option<::std::string::String>)`](crate::operation::create_backup::builders::CreateBackupFluentBuilder::set_backup_name): (undocumented)<br>
    ///   - [`table_name(impl Into<Option<::std::string::String>>)`](crate::operation::create_backup::builders::CreateBackupFluentBuilder::table_name) / [`set_table_name(Option<::std::string::String>)`](crate::operation::create_backup::builders::CreateBackupFluentBuilder::set_table_name): (undocumented)<br>
    /// - On success, responds with [`CreateBackupOutput`](crate::operation::create_backup::CreateBackupOutput) with field(s):
    ///   - [`backup_details(Option<dynamodb::types::BackupDetails>)`](crate::operation::create_backup::CreateBackupOutput::backup_details): (undocumented)
    /// - On failure, responds with [`SdkError<CreateBackupError>`](crate::operation::create_backup::CreateBackupError)
    pub fn create_backup(&self) -> crate::operation::create_backup::builders::CreateBackupFluentBuilder {
        crate::operation::create_backup::builders::CreateBackupFluentBuilder::new(self.clone())
    }
}
