// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateKeyRequest {
    #[allow(missing_docs)] // documentation missing in model
pub bypass_policy_lockout_safety_check: ::std::option::Option<::std::primitive::bool>,
#[allow(missing_docs)] // documentation missing in model
pub custom_key_store_id: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub customer_master_key_spec: ::std::option::Option<kms::types::CustomerMasterKeySpec>,
#[allow(missing_docs)] // documentation missing in model
pub description: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub key_spec: ::std::option::Option<kms::types::KeySpec>,
#[allow(missing_docs)] // documentation missing in model
pub key_usage: ::std::option::Option<kms::types::KeyUsageType>,
#[allow(missing_docs)] // documentation missing in model
pub multi_region: ::std::option::Option<::std::primitive::bool>,
#[allow(missing_docs)] // documentation missing in model
pub origin: ::std::option::Option<kms::types::OriginType>,
#[allow(missing_docs)] // documentation missing in model
pub policy: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub tags: ::std::option::Option<::std::vec::Vec<kms::types::Tag>>,
#[allow(missing_docs)] // documentation missing in model
pub xks_key_id: ::std::option::Option<::std::string::String>,
}
impl CreateKeyRequest {
    #[allow(missing_docs)] // documentation missing in model
pub fn bypass_policy_lockout_safety_check(&self) -> &::std::option::Option<::std::primitive::bool> {
    &self.bypass_policy_lockout_safety_check
}
#[allow(missing_docs)] // documentation missing in model
pub fn custom_key_store_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.custom_key_store_id
}
#[allow(missing_docs)] // documentation missing in model
pub fn customer_master_key_spec(&self) -> &::std::option::Option<kms::types::CustomerMasterKeySpec> {
    &self.customer_master_key_spec
}
#[allow(missing_docs)] // documentation missing in model
pub fn description(&self) -> &::std::option::Option<::std::string::String> {
    &self.description
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_spec(&self) -> &::std::option::Option<kms::types::KeySpec> {
    &self.key_spec
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_usage(&self) -> &::std::option::Option<kms::types::KeyUsageType> {
    &self.key_usage
}
#[allow(missing_docs)] // documentation missing in model
pub fn multi_region(&self) -> &::std::option::Option<::std::primitive::bool> {
    &self.multi_region
}
#[allow(missing_docs)] // documentation missing in model
pub fn origin(&self) -> &::std::option::Option<kms::types::OriginType> {
    &self.origin
}
#[allow(missing_docs)] // documentation missing in model
pub fn policy(&self) -> &::std::option::Option<::std::string::String> {
    &self.policy
}
#[allow(missing_docs)] // documentation missing in model
pub fn tags(&self) -> &::std::option::Option<::std::vec::Vec<kms::types::Tag>> {
    &self.tags
}
#[allow(missing_docs)] // documentation missing in model
pub fn xks_key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.xks_key_id
}
}
impl CreateKeyRequest {
    /// Creates a new builder-style object to manufacture [`CreateKeyRequest`](crate::operation::create_key::builders::CreateKeyRequest).
    pub fn builder() -> crate::operation::create_key::builders::CreateKeyRequestBuilder {
        crate::operation::create_key::builders::CreateKeyRequestBuilder::default()
    }
}

/// A builder for [`CreateKeyRequest`](crate::operation::operation::CreateKeyRequest).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateKeyRequestBuilder {
    pub(crate) bypass_policy_lockout_safety_check: ::std::option::Option<::std::primitive::bool>,
pub(crate) custom_key_store_id: ::std::option::Option<::std::string::String>,
pub(crate) customer_master_key_spec: ::std::option::Option<kms::types::CustomerMasterKeySpec>,
pub(crate) description: ::std::option::Option<::std::string::String>,
pub(crate) key_spec: ::std::option::Option<kms::types::KeySpec>,
pub(crate) key_usage: ::std::option::Option<kms::types::KeyUsageType>,
pub(crate) multi_region: ::std::option::Option<::std::primitive::bool>,
pub(crate) origin: ::std::option::Option<kms::types::OriginType>,
pub(crate) policy: ::std::option::Option<::std::string::String>,
pub(crate) tags: ::std::option::Option<::std::vec::Vec<kms::types::Tag>>,
pub(crate) xks_key_id: ::std::option::Option<::std::string::String>,
}
impl CreateKeyRequestBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn bypass_policy_lockout_safety_check(mut self, input: impl ::std::convert::Into<::std::primitive::bool>) -> Self {
    self.bypass_policy_lockout_safety_check = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_bypass_policy_lockout_safety_check(mut self, input: ::std::option::Option<::std::primitive::bool>) -> Self {
    self.bypass_policy_lockout_safety_check = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_bypass_policy_lockout_safety_check(&self) -> &::std::option::Option<::std::primitive::bool> {
    &self.bypass_policy_lockout_safety_check
}
#[allow(missing_docs)] // documentation missing in model
pub fn custom_key_store_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.custom_key_store_id = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_custom_key_store_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.custom_key_store_id = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_custom_key_store_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.custom_key_store_id
}
#[allow(missing_docs)] // documentation missing in model
pub fn customer_master_key_spec(mut self, input: impl ::std::convert::Into<kms::types::CustomerMasterKeySpec>) -> Self {
    self.customer_master_key_spec = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_customer_master_key_spec(mut self, input: ::std::option::Option<kms::types::CustomerMasterKeySpec>) -> Self {
    self.customer_master_key_spec = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_customer_master_key_spec(&self) -> &::std::option::Option<kms::types::CustomerMasterKeySpec> {
    &self.customer_master_key_spec
}
#[allow(missing_docs)] // documentation missing in model
pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.description = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.description = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
    &self.description
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_spec(mut self, input: impl ::std::convert::Into<kms::types::KeySpec>) -> Self {
    self.key_spec = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_key_spec(mut self, input: ::std::option::Option<kms::types::KeySpec>) -> Self {
    self.key_spec = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_key_spec(&self) -> &::std::option::Option<kms::types::KeySpec> {
    &self.key_spec
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_usage(mut self, input: impl ::std::convert::Into<kms::types::KeyUsageType>) -> Self {
    self.key_usage = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_key_usage(mut self, input: ::std::option::Option<kms::types::KeyUsageType>) -> Self {
    self.key_usage = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_key_usage(&self) -> &::std::option::Option<kms::types::KeyUsageType> {
    &self.key_usage
}
#[allow(missing_docs)] // documentation missing in model
pub fn multi_region(mut self, input: impl ::std::convert::Into<::std::primitive::bool>) -> Self {
    self.multi_region = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_multi_region(mut self, input: ::std::option::Option<::std::primitive::bool>) -> Self {
    self.multi_region = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_multi_region(&self) -> &::std::option::Option<::std::primitive::bool> {
    &self.multi_region
}
#[allow(missing_docs)] // documentation missing in model
pub fn origin(mut self, input: impl ::std::convert::Into<kms::types::OriginType>) -> Self {
    self.origin = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_origin(mut self, input: ::std::option::Option<kms::types::OriginType>) -> Self {
    self.origin = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_origin(&self) -> &::std::option::Option<kms::types::OriginType> {
    &self.origin
}
#[allow(missing_docs)] // documentation missing in model
pub fn policy(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.policy = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_policy(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.policy = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_policy(&self) -> &::std::option::Option<::std::string::String> {
    &self.policy
}
#[allow(missing_docs)] // documentation missing in model
pub fn tags(mut self, input: impl ::std::convert::Into<::std::vec::Vec<kms::types::Tag>>) -> Self {
    self.tags = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<kms::types::Tag>>) -> Self {
    self.tags = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<kms::types::Tag>> {
    &self.tags
}
#[allow(missing_docs)] // documentation missing in model
pub fn xks_key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.xks_key_id = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_xks_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.xks_key_id = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_xks_key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.xks_key_id
}
    /// Consumes the builder and constructs a [`CreateKeyRequest`](crate::operation::operation::CreateKeyRequest).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_key::CreateKeyRequest,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::create_key::CreateKeyRequest {
            bypass_policy_lockout_safety_check: self.bypass_policy_lockout_safety_check,
custom_key_store_id: self.custom_key_store_id,
customer_master_key_spec: self.customer_master_key_spec,
description: self.description,
key_spec: self.key_spec,
key_usage: self.key_usage,
multi_region: self.multi_region,
origin: self.origin,
policy: self.policy,
tags: self.tags,
xks_key_id: self.xks_key_id,
        })
    }
}
