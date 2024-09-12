// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::update_global_table_settings::_update_global_table_settings_output::UpdateGlobalTableSettingsOutputBuilder;

pub use crate::operation::update_global_table_settings::_update_global_table_settings_input::UpdateGlobalTableSettingsInputBuilder;

impl UpdateGlobalTableSettingsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        crate::operation::update_global_table_settings::UpdateGlobalTableSettingsOutput,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.update_global_table_settings();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateGlobalTableSettings`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateGlobalTableSettingsFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::update_global_table_settings::builders::UpdateGlobalTableSettingsInputBuilder,
}
impl UpdateGlobalTableSettingsFluentBuilder {
    /// Creates a new `UpdateGlobalTableSettings`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the UpdateGlobalTableSettings as a reference.
    pub fn as_input(&self) -> &crate::operation::update_global_table_settings::builders::UpdateGlobalTableSettingsInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_global_table_settings::UpdateGlobalTableSettingsOutput,
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
        crate::operation::update_global_table_settings::UpdateGlobalTableSettings::send(&self.client, input).await
    }

    #[allow(missing_docs)] // documentation missing in model
pub fn global_table_billing_mode(mut self, input: impl ::std::convert::Into<dynamodb::types::BillingMode>) -> Self {
    self.inner = self.inner.global_table_billing_mode(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_global_table_billing_mode(mut self, input: ::std::option::Option<dynamodb::types::BillingMode>) -> Self {
    self.inner = self.inner.set_global_table_billing_mode(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_global_table_billing_mode(&self) -> &::std::option::Option<dynamodb::types::BillingMode> {
    self.inner.get_global_table_billing_mode()
}
#[allow(missing_docs)] // documentation missing in model
pub fn global_table_global_secondary_index_settings_update(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::GlobalTableGlobalSecondaryIndexSettingsUpdate>>) -> Self {
    self.inner = self.inner.global_table_global_secondary_index_settings_update(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_global_table_global_secondary_index_settings_update(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::GlobalTableGlobalSecondaryIndexSettingsUpdate>>) -> Self {
    self.inner = self.inner.set_global_table_global_secondary_index_settings_update(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_global_table_global_secondary_index_settings_update(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::GlobalTableGlobalSecondaryIndexSettingsUpdate>> {
    self.inner.get_global_table_global_secondary_index_settings_update()
}
#[allow(missing_docs)] // documentation missing in model
pub fn global_table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.global_table_name(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_global_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_global_table_name(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_global_table_name(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_global_table_name()
}
#[allow(missing_docs)] // documentation missing in model
pub fn global_table_provisioned_write_capacity_auto_scaling_settings_update(mut self, input: impl ::std::convert::Into<dynamodb::types::AutoScalingSettingsUpdate>) -> Self {
    self.inner = self.inner.global_table_provisioned_write_capacity_auto_scaling_settings_update(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_global_table_provisioned_write_capacity_auto_scaling_settings_update(mut self, input: ::std::option::Option<dynamodb::types::AutoScalingSettingsUpdate>) -> Self {
    self.inner = self.inner.set_global_table_provisioned_write_capacity_auto_scaling_settings_update(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_global_table_provisioned_write_capacity_auto_scaling_settings_update(&self) -> &::std::option::Option<dynamodb::types::AutoScalingSettingsUpdate> {
    self.inner.get_global_table_provisioned_write_capacity_auto_scaling_settings_update()
}
#[allow(missing_docs)] // documentation missing in model
pub fn global_table_provisioned_write_capacity_units(mut self, input: impl ::std::convert::Into<::std::primitive::i64>) -> Self {
    self.inner = self.inner.global_table_provisioned_write_capacity_units(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_global_table_provisioned_write_capacity_units(mut self, input: ::std::option::Option<::std::primitive::i64>) -> Self {
    self.inner = self.inner.set_global_table_provisioned_write_capacity_units(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_global_table_provisioned_write_capacity_units(&self) -> &::std::option::Option<::std::primitive::i64> {
    self.inner.get_global_table_provisioned_write_capacity_units()
}
#[allow(missing_docs)] // documentation missing in model
pub fn replica_settings_update(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::ReplicaSettingsUpdate>>) -> Self {
    self.inner = self.inner.replica_settings_update(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_replica_settings_update(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::ReplicaSettingsUpdate>>) -> Self {
    self.inner = self.inner.set_replica_settings_update(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_replica_settings_update(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::ReplicaSettingsUpdate>> {
    self.inner.get_replica_settings_update()
}
}
