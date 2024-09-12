// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateGlobalTableInput {
    #[allow(missing_docs)] // documentation missing in model
pub global_table_name: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub replica_updates: ::std::option::Option<::std::vec::Vec<dynamodb::types::ReplicaUpdate>>,
}
impl UpdateGlobalTableInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn global_table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.global_table_name
}
#[allow(missing_docs)] // documentation missing in model
pub fn replica_updates(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::ReplicaUpdate>> {
    &self.replica_updates
}
}
impl UpdateGlobalTableInput {
    /// Creates a new builder-style object to manufacture [`UpdateGlobalTableInput`](crate::operation::update_global_table::builders::UpdateGlobalTableInput).
    pub fn builder() -> crate::operation::update_global_table::builders::UpdateGlobalTableInputBuilder {
        crate::operation::update_global_table::builders::UpdateGlobalTableInputBuilder::default()
    }
}

/// A builder for [`UpdateGlobalTableInput`](crate::operation::operation::UpdateGlobalTableInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateGlobalTableInputBuilder {
    pub(crate) global_table_name: ::std::option::Option<::std::string::String>,
pub(crate) replica_updates: ::std::option::Option<::std::vec::Vec<dynamodb::types::ReplicaUpdate>>,
}
impl UpdateGlobalTableInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn global_table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.global_table_name = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_global_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.global_table_name = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_global_table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.global_table_name
}
#[allow(missing_docs)] // documentation missing in model
pub fn replica_updates(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::ReplicaUpdate>>) -> Self {
    self.replica_updates = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_replica_updates(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::ReplicaUpdate>>) -> Self {
    self.replica_updates = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_replica_updates(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::ReplicaUpdate>> {
    &self.replica_updates
}
    /// Consumes the builder and constructs a [`UpdateGlobalTableInput`](crate::operation::operation::UpdateGlobalTableInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_global_table::UpdateGlobalTableInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::update_global_table::UpdateGlobalTableInput {
            global_table_name: self.global_table_name,
replica_updates: self.replica_updates,
        })
    }
}
