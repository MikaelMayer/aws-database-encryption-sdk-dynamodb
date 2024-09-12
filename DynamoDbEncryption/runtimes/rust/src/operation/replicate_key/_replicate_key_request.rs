// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ReplicateKeyRequest {
    #[allow(missing_docs)] // documentation missing in model
pub bypass_policy_lockout_safety_check: ::std::option::Option<::std::primitive::bool>,
#[allow(missing_docs)] // documentation missing in model
pub description: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub key_id: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub policy: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub replica_region: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub tags: ::std::option::Option<::std::vec::Vec<kms::types::Tag>>,
}
impl ReplicateKeyRequest {
    #[allow(missing_docs)] // documentation missing in model
pub fn bypass_policy_lockout_safety_check(&self) -> &::std::option::Option<::std::primitive::bool> {
    &self.bypass_policy_lockout_safety_check
}
#[allow(missing_docs)] // documentation missing in model
pub fn description(&self) -> &::std::option::Option<::std::string::String> {
    &self.description
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.key_id
}
#[allow(missing_docs)] // documentation missing in model
pub fn policy(&self) -> &::std::option::Option<::std::string::String> {
    &self.policy
}
#[allow(missing_docs)] // documentation missing in model
pub fn replica_region(&self) -> &::std::option::Option<::std::string::String> {
    &self.replica_region
}
#[allow(missing_docs)] // documentation missing in model
pub fn tags(&self) -> &::std::option::Option<::std::vec::Vec<kms::types::Tag>> {
    &self.tags
}
}
impl ReplicateKeyRequest {
    /// Creates a new builder-style object to manufacture [`ReplicateKeyRequest`](crate::operation::replicate_key::builders::ReplicateKeyRequest).
    pub fn builder() -> crate::operation::replicate_key::builders::ReplicateKeyRequestBuilder {
        crate::operation::replicate_key::builders::ReplicateKeyRequestBuilder::default()
    }
}

/// A builder for [`ReplicateKeyRequest`](crate::operation::operation::ReplicateKeyRequest).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ReplicateKeyRequestBuilder {
    pub(crate) bypass_policy_lockout_safety_check: ::std::option::Option<::std::primitive::bool>,
pub(crate) description: ::std::option::Option<::std::string::String>,
pub(crate) key_id: ::std::option::Option<::std::string::String>,
pub(crate) policy: ::std::option::Option<::std::string::String>,
pub(crate) replica_region: ::std::option::Option<::std::string::String>,
pub(crate) tags: ::std::option::Option<::std::vec::Vec<kms::types::Tag>>,
}
impl ReplicateKeyRequestBuilder {
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
pub fn key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.key_id = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.key_id = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.key_id
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
pub fn replica_region(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.replica_region = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_replica_region(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.replica_region = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_replica_region(&self) -> &::std::option::Option<::std::string::String> {
    &self.replica_region
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
    /// Consumes the builder and constructs a [`ReplicateKeyRequest`](crate::operation::operation::ReplicateKeyRequest).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::replicate_key::ReplicateKeyRequest,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::replicate_key::ReplicateKeyRequest {
            bypass_policy_lockout_safety_check: self.bypass_policy_lockout_safety_check,
description: self.description,
key_id: self.key_id,
policy: self.policy,
replica_region: self.replica_region,
tags: self.tags,
        })
    }
}
