// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::restore_table_from_backup::_restore_table_from_backup_output::RestoreTableFromBackupOutputBuilder;

pub use crate::operation::restore_table_from_backup::_restore_table_from_backup_input::RestoreTableFromBackupInputBuilder;

impl RestoreTableFromBackupInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        crate::operation::restore_table_from_backup::RestoreTableFromBackupOutput,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.restore_table_from_backup();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `RestoreTableFromBackup`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct RestoreTableFromBackupFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::restore_table_from_backup::builders::RestoreTableFromBackupInputBuilder,
}
impl RestoreTableFromBackupFluentBuilder {
    /// Creates a new `RestoreTableFromBackup`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the RestoreTableFromBackup as a reference.
    pub fn as_input(&self) -> &crate::operation::restore_table_from_backup::builders::RestoreTableFromBackupInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::restore_table_from_backup::RestoreTableFromBackupOutput,
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
        crate::operation::restore_table_from_backup::RestoreTableFromBackup::send(&self.client, input).await
    }

    #[allow(missing_docs)] // documentation missing in model
pub fn backup_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.backup_arn(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_backup_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_backup_arn(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_backup_arn(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_backup_arn()
}
#[allow(missing_docs)] // documentation missing in model
pub fn billing_mode_override(mut self, input: impl ::std::convert::Into<dynamodb::types::BillingMode>) -> Self {
    self.inner = self.inner.billing_mode_override(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_billing_mode_override(mut self, input: ::std::option::Option<dynamodb::types::BillingMode>) -> Self {
    self.inner = self.inner.set_billing_mode_override(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_billing_mode_override(&self) -> &::std::option::Option<dynamodb::types::BillingMode> {
    self.inner.get_billing_mode_override()
}
#[allow(missing_docs)] // documentation missing in model
pub fn global_secondary_index_override(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::GlobalSecondaryIndex>>) -> Self {
    self.inner = self.inner.global_secondary_index_override(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_global_secondary_index_override(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::GlobalSecondaryIndex>>) -> Self {
    self.inner = self.inner.set_global_secondary_index_override(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_global_secondary_index_override(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::GlobalSecondaryIndex>> {
    self.inner.get_global_secondary_index_override()
}
#[allow(missing_docs)] // documentation missing in model
pub fn local_secondary_index_override(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::LocalSecondaryIndex>>) -> Self {
    self.inner = self.inner.local_secondary_index_override(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_local_secondary_index_override(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::LocalSecondaryIndex>>) -> Self {
    self.inner = self.inner.set_local_secondary_index_override(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_local_secondary_index_override(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::LocalSecondaryIndex>> {
    self.inner.get_local_secondary_index_override()
}
#[allow(missing_docs)] // documentation missing in model
pub fn provisioned_throughput_override(mut self, input: impl ::std::convert::Into<dynamodb::types::ProvisionedThroughput>) -> Self {
    self.inner = self.inner.provisioned_throughput_override(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_provisioned_throughput_override(mut self, input: ::std::option::Option<dynamodb::types::ProvisionedThroughput>) -> Self {
    self.inner = self.inner.set_provisioned_throughput_override(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_provisioned_throughput_override(&self) -> &::std::option::Option<dynamodb::types::ProvisionedThroughput> {
    self.inner.get_provisioned_throughput_override()
}
#[allow(missing_docs)] // documentation missing in model
pub fn sse_specification_override(mut self, input: impl ::std::convert::Into<dynamodb::types::SseSpecification>) -> Self {
    self.inner = self.inner.sse_specification_override(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_sse_specification_override(mut self, input: ::std::option::Option<dynamodb::types::SseSpecification>) -> Self {
    self.inner = self.inner.set_sse_specification_override(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_sse_specification_override(&self) -> &::std::option::Option<dynamodb::types::SseSpecification> {
    self.inner.get_sse_specification_override()
}
#[allow(missing_docs)] // documentation missing in model
pub fn target_table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.target_table_name(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_target_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_target_table_name(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_target_table_name(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_target_table_name()
}
}