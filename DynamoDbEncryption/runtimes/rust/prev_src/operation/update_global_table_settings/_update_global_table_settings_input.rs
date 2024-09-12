// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateGlobalTableSettingsInput {
    #[allow(missing_docs)] // documentation missing in model
pub global_table_billing_mode: ::std::option::Option<dynamodb::types::BillingMode>,
#[allow(missing_docs)] // documentation missing in model
pub global_table_global_secondary_index_settings_update: ::std::option::Option<::std::vec::Vec<dynamodb::types::GlobalTableGlobalSecondaryIndexSettingsUpdate>>,
#[allow(missing_docs)] // documentation missing in model
pub global_table_name: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub global_table_provisioned_write_capacity_auto_scaling_settings_update: ::std::option::Option<dynamodb::types::AutoScalingSettingsUpdate>,
#[allow(missing_docs)] // documentation missing in model
pub global_table_provisioned_write_capacity_units: ::std::option::Option<::std::primitive::i64>,
#[allow(missing_docs)] // documentation missing in model
pub replica_settings_update: ::std::option::Option<::std::vec::Vec<dynamodb::types::ReplicaSettingsUpdate>>,
}
impl UpdateGlobalTableSettingsInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn global_table_billing_mode(&self) -> &::std::option::Option<dynamodb::types::BillingMode> {
    &self.global_table_billing_mode
}
#[allow(missing_docs)] // documentation missing in model
pub fn global_table_global_secondary_index_settings_update(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::GlobalTableGlobalSecondaryIndexSettingsUpdate>> {
    &self.global_table_global_secondary_index_settings_update
}
#[allow(missing_docs)] // documentation missing in model
pub fn global_table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.global_table_name
}
#[allow(missing_docs)] // documentation missing in model
pub fn global_table_provisioned_write_capacity_auto_scaling_settings_update(&self) -> &::std::option::Option<dynamodb::types::AutoScalingSettingsUpdate> {
    &self.global_table_provisioned_write_capacity_auto_scaling_settings_update
}
#[allow(missing_docs)] // documentation missing in model
pub fn global_table_provisioned_write_capacity_units(&self) -> &::std::option::Option<::std::primitive::i64> {
    &self.global_table_provisioned_write_capacity_units
}
#[allow(missing_docs)] // documentation missing in model
pub fn replica_settings_update(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::ReplicaSettingsUpdate>> {
    &self.replica_settings_update
}
}
impl UpdateGlobalTableSettingsInput {
    /// Creates a new builder-style object to manufacture [`UpdateGlobalTableSettingsInput`](crate::operation::update_global_table_settings::builders::UpdateGlobalTableSettingsInput).
    pub fn builder() -> crate::operation::update_global_table_settings::builders::UpdateGlobalTableSettingsInputBuilder {
        crate::operation::update_global_table_settings::builders::UpdateGlobalTableSettingsInputBuilder::default()
    }
}

/// A builder for [`UpdateGlobalTableSettingsInput`](crate::operation::operation::UpdateGlobalTableSettingsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateGlobalTableSettingsInputBuilder {
    pub(crate) global_table_billing_mode: ::std::option::Option<dynamodb::types::BillingMode>,
pub(crate) global_table_global_secondary_index_settings_update: ::std::option::Option<::std::vec::Vec<dynamodb::types::GlobalTableGlobalSecondaryIndexSettingsUpdate>>,
pub(crate) global_table_name: ::std::option::Option<::std::string::String>,
pub(crate) global_table_provisioned_write_capacity_auto_scaling_settings_update: ::std::option::Option<dynamodb::types::AutoScalingSettingsUpdate>,
pub(crate) global_table_provisioned_write_capacity_units: ::std::option::Option<::std::primitive::i64>,
pub(crate) replica_settings_update: ::std::option::Option<::std::vec::Vec<dynamodb::types::ReplicaSettingsUpdate>>,
}
impl UpdateGlobalTableSettingsInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn global_table_billing_mode(mut self, input: impl ::std::convert::Into<dynamodb::types::BillingMode>) -> Self {
    self.global_table_billing_mode = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_global_table_billing_mode(mut self, input: ::std::option::Option<dynamodb::types::BillingMode>) -> Self {
    self.global_table_billing_mode = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_global_table_billing_mode(&self) -> &::std::option::Option<dynamodb::types::BillingMode> {
    &self.global_table_billing_mode
}
#[allow(missing_docs)] // documentation missing in model
pub fn global_table_global_secondary_index_settings_update(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::GlobalTableGlobalSecondaryIndexSettingsUpdate>>) -> Self {
    self.global_table_global_secondary_index_settings_update = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_global_table_global_secondary_index_settings_update(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::GlobalTableGlobalSecondaryIndexSettingsUpdate>>) -> Self {
    self.global_table_global_secondary_index_settings_update = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_global_table_global_secondary_index_settings_update(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::GlobalTableGlobalSecondaryIndexSettingsUpdate>> {
    &self.global_table_global_secondary_index_settings_update
}
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
pub fn global_table_provisioned_write_capacity_auto_scaling_settings_update(mut self, input: impl ::std::convert::Into<dynamodb::types::AutoScalingSettingsUpdate>) -> Self {
    self.global_table_provisioned_write_capacity_auto_scaling_settings_update = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_global_table_provisioned_write_capacity_auto_scaling_settings_update(mut self, input: ::std::option::Option<dynamodb::types::AutoScalingSettingsUpdate>) -> Self {
    self.global_table_provisioned_write_capacity_auto_scaling_settings_update = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_global_table_provisioned_write_capacity_auto_scaling_settings_update(&self) -> &::std::option::Option<dynamodb::types::AutoScalingSettingsUpdate> {
    &self.global_table_provisioned_write_capacity_auto_scaling_settings_update
}
#[allow(missing_docs)] // documentation missing in model
pub fn global_table_provisioned_write_capacity_units(mut self, input: impl ::std::convert::Into<::std::primitive::i64>) -> Self {
    self.global_table_provisioned_write_capacity_units = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_global_table_provisioned_write_capacity_units(mut self, input: ::std::option::Option<::std::primitive::i64>) -> Self {
    self.global_table_provisioned_write_capacity_units = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_global_table_provisioned_write_capacity_units(&self) -> &::std::option::Option<::std::primitive::i64> {
    &self.global_table_provisioned_write_capacity_units
}
#[allow(missing_docs)] // documentation missing in model
pub fn replica_settings_update(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::ReplicaSettingsUpdate>>) -> Self {
    self.replica_settings_update = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_replica_settings_update(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::ReplicaSettingsUpdate>>) -> Self {
    self.replica_settings_update = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_replica_settings_update(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::ReplicaSettingsUpdate>> {
    &self.replica_settings_update
}
    /// Consumes the builder and constructs a [`UpdateGlobalTableSettingsInput`](crate::operation::operation::UpdateGlobalTableSettingsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_global_table_settings::UpdateGlobalTableSettingsInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::update_global_table_settings::UpdateGlobalTableSettingsInput {
            global_table_billing_mode: self.global_table_billing_mode,
global_table_global_secondary_index_settings_update: self.global_table_global_secondary_index_settings_update,
global_table_name: self.global_table_name,
global_table_provisioned_write_capacity_auto_scaling_settings_update: self.global_table_provisioned_write_capacity_auto_scaling_settings_update,
global_table_provisioned_write_capacity_units: self.global_table_provisioned_write_capacity_units,
replica_settings_update: self.replica_settings_update,
        })
    }
}
