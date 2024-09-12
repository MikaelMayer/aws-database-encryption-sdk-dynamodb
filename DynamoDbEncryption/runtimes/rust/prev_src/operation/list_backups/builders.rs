// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::list_backups::_list_backups_output::ListBackupsOutputBuilder;

pub use crate::operation::list_backups::_list_backups_input::ListBackupsInputBuilder;

impl ListBackupsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        crate::operation::list_backups::ListBackupsOutput,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.list_backups();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListBackups`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListBackupsFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::list_backups::builders::ListBackupsInputBuilder,
}
impl ListBackupsFluentBuilder {
    /// Creates a new `ListBackups`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the ListBackups as a reference.
    pub fn as_input(&self) -> &crate::operation::list_backups::builders::ListBackupsInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_backups::ListBackupsOutput,
        crate::types::error::Error,
    > {
        let input = self
            .inner
            .build()
            // Using Opaque since we don't have a validation-specific error yet.
            // Operations' models don't declare their own validation error,
            // and smithy-rs seems to not generate a ValidationError case unless there is.
            // Vanilla smithy-rs uses SdkError::construction_failure, but we aren't using SdkError.
            .map_err(|mut e| crate::types::error::Error::Opaque {
                obj: ::dafny_runtime::Object::from_ref(&mut e as &mut dyn ::std::any::Any)
            })?;
        crate::operation::list_backups::ListBackups::send(&self.client, input).await
    }

    #[allow(missing_docs)] // documentation missing in model
pub fn backup_type(mut self, input: impl ::std::convert::Into<dynamodb::types::BackupTypeFilter>) -> Self {
    self.inner = self.inner.backup_type(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_backup_type(mut self, input: ::std::option::Option<dynamodb::types::BackupTypeFilter>) -> Self {
    self.inner = self.inner.set_backup_type(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_backup_type(&self) -> &::std::option::Option<dynamodb::types::BackupTypeFilter> {
    self.inner.get_backup_type()
}
#[allow(missing_docs)] // documentation missing in model
pub fn exclusive_start_backup_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.exclusive_start_backup_arn(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_exclusive_start_backup_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_exclusive_start_backup_arn(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_exclusive_start_backup_arn(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_exclusive_start_backup_arn()
}
#[allow(missing_docs)] // documentation missing in model
pub fn limit(mut self, input: impl ::std::convert::Into<::std::primitive::i32>) -> Self {
    self.inner = self.inner.limit(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_limit(mut self, input: ::std::option::Option<::std::primitive::i32>) -> Self {
    self.inner = self.inner.set_limit(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_limit(&self) -> &::std::option::Option<::std::primitive::i32> {
    self.inner.get_limit()
}
#[allow(missing_docs)] // documentation missing in model
pub fn table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.table_name(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_table_name(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_table_name(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_table_name()
}
#[allow(missing_docs)] // documentation missing in model
pub fn time_range_lower_bound(mut self, input: impl ::std::convert::Into<::aws_smithy_types::DateTime>) -> Self {
    self.inner = self.inner.time_range_lower_bound(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_time_range_lower_bound(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
    self.inner = self.inner.set_time_range_lower_bound(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_time_range_lower_bound(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
    self.inner.get_time_range_lower_bound()
}
#[allow(missing_docs)] // documentation missing in model
pub fn time_range_upper_bound(mut self, input: impl ::std::convert::Into<::aws_smithy_types::DateTime>) -> Self {
    self.inner = self.inner.time_range_upper_bound(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_time_range_upper_bound(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
    self.inner = self.inner.set_time_range_upper_bound(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_time_range_upper_bound(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
    self.inner.get_time_range_upper_bound()
}
}
