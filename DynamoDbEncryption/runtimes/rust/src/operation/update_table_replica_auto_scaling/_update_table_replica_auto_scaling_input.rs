// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateTableReplicaAutoScalingInput {
    #[allow(missing_docs)] // documentation missing in model
pub global_secondary_index_updates: ::std::option::Option<::std::vec::Vec<dynamodb::types::GlobalSecondaryIndexAutoScalingUpdate>>,
#[allow(missing_docs)] // documentation missing in model
pub provisioned_write_capacity_auto_scaling_update: ::std::option::Option<dynamodb::types::AutoScalingSettingsUpdate>,
#[allow(missing_docs)] // documentation missing in model
pub replica_updates: ::std::option::Option<::std::vec::Vec<dynamodb::types::ReplicaAutoScalingUpdate>>,
#[allow(missing_docs)] // documentation missing in model
pub table_name: ::std::option::Option<::std::string::String>,
}
impl UpdateTableReplicaAutoScalingInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn global_secondary_index_updates(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::GlobalSecondaryIndexAutoScalingUpdate>> {
    &self.global_secondary_index_updates
}
#[allow(missing_docs)] // documentation missing in model
pub fn provisioned_write_capacity_auto_scaling_update(&self) -> &::std::option::Option<dynamodb::types::AutoScalingSettingsUpdate> {
    &self.provisioned_write_capacity_auto_scaling_update
}
#[allow(missing_docs)] // documentation missing in model
pub fn replica_updates(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::ReplicaAutoScalingUpdate>> {
    &self.replica_updates
}
#[allow(missing_docs)] // documentation missing in model
pub fn table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.table_name
}
}
impl UpdateTableReplicaAutoScalingInput {
    /// Creates a new builder-style object to manufacture [`UpdateTableReplicaAutoScalingInput`](crate::operation::update_table_replica_auto_scaling::builders::UpdateTableReplicaAutoScalingInput).
    pub fn builder() -> crate::operation::update_table_replica_auto_scaling::builders::UpdateTableReplicaAutoScalingInputBuilder {
        crate::operation::update_table_replica_auto_scaling::builders::UpdateTableReplicaAutoScalingInputBuilder::default()
    }
}

/// A builder for [`UpdateTableReplicaAutoScalingInput`](crate::operation::operation::UpdateTableReplicaAutoScalingInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateTableReplicaAutoScalingInputBuilder {
    pub(crate) global_secondary_index_updates: ::std::option::Option<::std::vec::Vec<dynamodb::types::GlobalSecondaryIndexAutoScalingUpdate>>,
pub(crate) provisioned_write_capacity_auto_scaling_update: ::std::option::Option<dynamodb::types::AutoScalingSettingsUpdate>,
pub(crate) replica_updates: ::std::option::Option<::std::vec::Vec<dynamodb::types::ReplicaAutoScalingUpdate>>,
pub(crate) table_name: ::std::option::Option<::std::string::String>,
}
impl UpdateTableReplicaAutoScalingInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn global_secondary_index_updates(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::GlobalSecondaryIndexAutoScalingUpdate>>) -> Self {
    self.global_secondary_index_updates = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_global_secondary_index_updates(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::GlobalSecondaryIndexAutoScalingUpdate>>) -> Self {
    self.global_secondary_index_updates = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_global_secondary_index_updates(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::GlobalSecondaryIndexAutoScalingUpdate>> {
    &self.global_secondary_index_updates
}
#[allow(missing_docs)] // documentation missing in model
pub fn provisioned_write_capacity_auto_scaling_update(mut self, input: impl ::std::convert::Into<dynamodb::types::AutoScalingSettingsUpdate>) -> Self {
    self.provisioned_write_capacity_auto_scaling_update = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_provisioned_write_capacity_auto_scaling_update(mut self, input: ::std::option::Option<dynamodb::types::AutoScalingSettingsUpdate>) -> Self {
    self.provisioned_write_capacity_auto_scaling_update = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_provisioned_write_capacity_auto_scaling_update(&self) -> &::std::option::Option<dynamodb::types::AutoScalingSettingsUpdate> {
    &self.provisioned_write_capacity_auto_scaling_update
}
#[allow(missing_docs)] // documentation missing in model
pub fn replica_updates(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::ReplicaAutoScalingUpdate>>) -> Self {
    self.replica_updates = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_replica_updates(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::ReplicaAutoScalingUpdate>>) -> Self {
    self.replica_updates = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_replica_updates(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::ReplicaAutoScalingUpdate>> {
    &self.replica_updates
}
#[allow(missing_docs)] // documentation missing in model
pub fn table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.table_name = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.table_name = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.table_name
}
    /// Consumes the builder and constructs a [`UpdateTableReplicaAutoScalingInput`](crate::operation::operation::UpdateTableReplicaAutoScalingInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_table_replica_auto_scaling::UpdateTableReplicaAutoScalingInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::update_table_replica_auto_scaling::UpdateTableReplicaAutoScalingInput {
            global_secondary_index_updates: self.global_secondary_index_updates,
provisioned_write_capacity_auto_scaling_update: self.provisioned_write_capacity_auto_scaling_update,
replica_updates: self.replica_updates,
table_name: self.table_name,
        })
    }
}
