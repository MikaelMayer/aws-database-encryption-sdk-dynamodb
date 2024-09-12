// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::update_table_replica_auto_scaling::_update_table_replica_auto_scaling_output::UpdateTableReplicaAutoScalingOutputBuilder;

pub use crate::operation::update_table_replica_auto_scaling::_update_table_replica_auto_scaling_input::UpdateTableReplicaAutoScalingInputBuilder;

impl UpdateTableReplicaAutoScalingInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        crate::operation::update_table_replica_auto_scaling::UpdateTableReplicaAutoScalingOutput,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.update_table_replica_auto_scaling();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateTableReplicaAutoScaling`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateTableReplicaAutoScalingFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::update_table_replica_auto_scaling::builders::UpdateTableReplicaAutoScalingInputBuilder,
}
impl UpdateTableReplicaAutoScalingFluentBuilder {
    /// Creates a new `UpdateTableReplicaAutoScaling`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the UpdateTableReplicaAutoScaling as a reference.
    pub fn as_input(&self) -> &crate::operation::update_table_replica_auto_scaling::builders::UpdateTableReplicaAutoScalingInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_table_replica_auto_scaling::UpdateTableReplicaAutoScalingOutput,
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
        crate::operation::update_table_replica_auto_scaling::UpdateTableReplicaAutoScaling::send(&self.client, input).await
    }

    #[allow(missing_docs)] // documentation missing in model
pub fn global_secondary_index_updates(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::GlobalSecondaryIndexAutoScalingUpdate>>) -> Self {
    self.inner = self.inner.global_secondary_index_updates(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_global_secondary_index_updates(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::GlobalSecondaryIndexAutoScalingUpdate>>) -> Self {
    self.inner = self.inner.set_global_secondary_index_updates(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_global_secondary_index_updates(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::GlobalSecondaryIndexAutoScalingUpdate>> {
    self.inner.get_global_secondary_index_updates()
}
#[allow(missing_docs)] // documentation missing in model
pub fn provisioned_write_capacity_auto_scaling_update(mut self, input: impl ::std::convert::Into<dynamodb::types::AutoScalingSettingsUpdate>) -> Self {
    self.inner = self.inner.provisioned_write_capacity_auto_scaling_update(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_provisioned_write_capacity_auto_scaling_update(mut self, input: ::std::option::Option<dynamodb::types::AutoScalingSettingsUpdate>) -> Self {
    self.inner = self.inner.set_provisioned_write_capacity_auto_scaling_update(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_provisioned_write_capacity_auto_scaling_update(&self) -> &::std::option::Option<dynamodb::types::AutoScalingSettingsUpdate> {
    self.inner.get_provisioned_write_capacity_auto_scaling_update()
}
#[allow(missing_docs)] // documentation missing in model
pub fn replica_updates(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::ReplicaAutoScalingUpdate>>) -> Self {
    self.inner = self.inner.replica_updates(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_replica_updates(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::ReplicaAutoScalingUpdate>>) -> Self {
    self.inner = self.inner.set_replica_updates(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_replica_updates(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::ReplicaAutoScalingUpdate>> {
    self.inner.get_replica_updates()
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
}
