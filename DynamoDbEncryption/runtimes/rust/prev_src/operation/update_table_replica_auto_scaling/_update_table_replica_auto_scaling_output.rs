// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateTableReplicaAutoScalingOutput {
    #[allow(missing_docs)] // documentation missing in model
pub table_auto_scaling_description: ::std::option::Option<dynamodb::types::TableAutoScalingDescription>,
}
impl UpdateTableReplicaAutoScalingOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn table_auto_scaling_description(&self) -> &::std::option::Option<dynamodb::types::TableAutoScalingDescription> {
    &self.table_auto_scaling_description
}
}
impl UpdateTableReplicaAutoScalingOutput {
    /// Creates a new builder-style object to manufacture [`UpdateTableReplicaAutoScalingOutput`](crate::operation::update_table_replica_auto_scaling::builders::UpdateTableReplicaAutoScalingOutput).
    pub fn builder() -> crate::operation::update_table_replica_auto_scaling::builders::UpdateTableReplicaAutoScalingOutputBuilder {
        crate::operation::update_table_replica_auto_scaling::builders::UpdateTableReplicaAutoScalingOutputBuilder::default()
    }
}

/// A builder for [`UpdateTableReplicaAutoScalingOutput`](crate::operation::operation::UpdateTableReplicaAutoScalingOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateTableReplicaAutoScalingOutputBuilder {
    pub(crate) table_auto_scaling_description: ::std::option::Option<dynamodb::types::TableAutoScalingDescription>,
}
impl UpdateTableReplicaAutoScalingOutputBuilder {
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
    /// Consumes the builder and constructs a [`UpdateTableReplicaAutoScalingOutput`](crate::operation::operation::UpdateTableReplicaAutoScalingOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_table_replica_auto_scaling::UpdateTableReplicaAutoScalingOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::update_table_replica_auto_scaling::UpdateTableReplicaAutoScalingOutput {
            table_auto_scaling_description: self.table_auto_scaling_description,
        })
    }
}
