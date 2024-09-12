// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeTableReplicaAutoScalingOutput {
    #[allow(missing_docs)] // documentation missing in model
pub table_auto_scaling_description: ::std::option::Option<dynamodb::types::TableAutoScalingDescription>,
}
impl DescribeTableReplicaAutoScalingOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn table_auto_scaling_description(&self) -> &::std::option::Option<dynamodb::types::TableAutoScalingDescription> {
    &self.table_auto_scaling_description
}
}
impl DescribeTableReplicaAutoScalingOutput {
    /// Creates a new builder-style object to manufacture [`DescribeTableReplicaAutoScalingOutput`](crate::operation::describe_table_replica_auto_scaling::builders::DescribeTableReplicaAutoScalingOutput).
    pub fn builder() -> crate::operation::describe_table_replica_auto_scaling::builders::DescribeTableReplicaAutoScalingOutputBuilder {
        crate::operation::describe_table_replica_auto_scaling::builders::DescribeTableReplicaAutoScalingOutputBuilder::default()
    }
}

/// A builder for [`DescribeTableReplicaAutoScalingOutput`](crate::operation::operation::DescribeTableReplicaAutoScalingOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeTableReplicaAutoScalingOutputBuilder {
    pub(crate) table_auto_scaling_description: ::std::option::Option<dynamodb::types::TableAutoScalingDescription>,
}
impl DescribeTableReplicaAutoScalingOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn table_auto_scaling_description(mut self, input: impl ::std::convert::Into<dynamodb::types::TableAutoScalingDescription>) -> Self {
    self.table_auto_scaling_description = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_table_auto_scaling_description(mut self, input: ::std::option::Option<dynamodb::types::TableAutoScalingDescription>) -> Self {
    self.table_auto_scaling_description = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_table_auto_scaling_description(&self) -> &::std::option::Option<dynamodb::types::TableAutoScalingDescription> {
    &self.table_auto_scaling_description
}
    /// Consumes the builder and constructs a [`DescribeTableReplicaAutoScalingOutput`](crate::operation::operation::DescribeTableReplicaAutoScalingOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_table_replica_auto_scaling::DescribeTableReplicaAutoScalingOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_table_replica_auto_scaling::DescribeTableReplicaAutoScalingOutput {
            table_auto_scaling_description: self.table_auto_scaling_description,
        })
    }
}
