// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`DescribeTableReplicaAutoScaling`](crate::operation::describe_table_replica_auto_scaling::builders::DescribeTableReplicaAutoScalingFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`table_name(impl Into<Option<::std::string::String>>)`](crate::operation::describe_table_replica_auto_scaling::builders::DescribeTableReplicaAutoScalingFluentBuilder::table_name) / [`set_table_name(Option<::std::string::String>)`](crate::operation::describe_table_replica_auto_scaling::builders::DescribeTableReplicaAutoScalingFluentBuilder::set_table_name): (undocumented)<br>
    /// - On success, responds with [`DescribeTableReplicaAutoScalingOutput`](crate::operation::describe_table_replica_auto_scaling::DescribeTableReplicaAutoScalingOutput) with field(s):
    ///   - [`table_auto_scaling_description(Option<dynamodb::types::TableAutoScalingDescription>)`](crate::operation::describe_table_replica_auto_scaling::DescribeTableReplicaAutoScalingOutput::table_auto_scaling_description): (undocumented)
    /// - On failure, responds with [`SdkError<DescribeTableReplicaAutoScalingError>`](crate::operation::describe_table_replica_auto_scaling::DescribeTableReplicaAutoScalingError)
    pub fn describe_table_replica_auto_scaling(&self) -> crate::operation::describe_table_replica_auto_scaling::builders::DescribeTableReplicaAutoScalingFluentBuilder {
        crate::operation::describe_table_replica_auto_scaling::builders::DescribeTableReplicaAutoScalingFluentBuilder::new(self.clone())
    }
}
