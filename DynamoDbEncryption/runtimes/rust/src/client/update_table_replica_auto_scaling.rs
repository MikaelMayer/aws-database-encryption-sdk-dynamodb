// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`UpdateTableReplicaAutoScaling`](crate::operation::update_table_replica_auto_scaling::builders::UpdateTableReplicaAutoScalingFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`global_secondary_index_updates(impl Into<Option<::std::vec::Vec<dynamodb::types::GlobalSecondaryIndexAutoScalingUpdate>>>)`](crate::operation::update_table_replica_auto_scaling::builders::UpdateTableReplicaAutoScalingFluentBuilder::global_secondary_index_updates) / [`set_global_secondary_index_updates(Option<::std::vec::Vec<dynamodb::types::GlobalSecondaryIndexAutoScalingUpdate>>)`](crate::operation::update_table_replica_auto_scaling::builders::UpdateTableReplicaAutoScalingFluentBuilder::set_global_secondary_index_updates): (undocumented)<br>
    ///   - [`provisioned_write_capacity_auto_scaling_update(impl Into<Option<dynamodb::types::AutoScalingSettingsUpdate>>)`](crate::operation::update_table_replica_auto_scaling::builders::UpdateTableReplicaAutoScalingFluentBuilder::provisioned_write_capacity_auto_scaling_update) / [`set_provisioned_write_capacity_auto_scaling_update(Option<dynamodb::types::AutoScalingSettingsUpdate>)`](crate::operation::update_table_replica_auto_scaling::builders::UpdateTableReplicaAutoScalingFluentBuilder::set_provisioned_write_capacity_auto_scaling_update): (undocumented)<br>
    ///   - [`replica_updates(impl Into<Option<::std::vec::Vec<dynamodb::types::ReplicaAutoScalingUpdate>>>)`](crate::operation::update_table_replica_auto_scaling::builders::UpdateTableReplicaAutoScalingFluentBuilder::replica_updates) / [`set_replica_updates(Option<::std::vec::Vec<dynamodb::types::ReplicaAutoScalingUpdate>>)`](crate::operation::update_table_replica_auto_scaling::builders::UpdateTableReplicaAutoScalingFluentBuilder::set_replica_updates): (undocumented)<br>
    ///   - [`table_name(impl Into<Option<::std::string::String>>)`](crate::operation::update_table_replica_auto_scaling::builders::UpdateTableReplicaAutoScalingFluentBuilder::table_name) / [`set_table_name(Option<::std::string::String>)`](crate::operation::update_table_replica_auto_scaling::builders::UpdateTableReplicaAutoScalingFluentBuilder::set_table_name): (undocumented)<br>
    /// - On success, responds with [`UpdateTableReplicaAutoScalingOutput`](crate::operation::update_table_replica_auto_scaling::UpdateTableReplicaAutoScalingOutput) with field(s):
    ///   - [`table_auto_scaling_description(Option<dynamodb::types::TableAutoScalingDescription>)`](crate::operation::update_table_replica_auto_scaling::UpdateTableReplicaAutoScalingOutput::table_auto_scaling_description): (undocumented)
    /// - On failure, responds with [`SdkError<UpdateTableReplicaAutoScalingError>`](crate::operation::update_table_replica_auto_scaling::UpdateTableReplicaAutoScalingError)
    pub fn update_table_replica_auto_scaling(&self) -> crate::operation::update_table_replica_auto_scaling::builders::UpdateTableReplicaAutoScalingFluentBuilder {
        crate::operation::update_table_replica_auto_scaling::builders::UpdateTableReplicaAutoScalingFluentBuilder::new(self.clone())
    }
}
