// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PutKeyPolicyRequest {
    #[allow(missing_docs)] // documentation missing in model
pub bypass_policy_lockout_safety_check: ::std::option::Option<::std::primitive::bool>,
#[allow(missing_docs)] // documentation missing in model
pub key_id: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub policy: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub policy_name: ::std::option::Option<::std::string::String>,
}
impl PutKeyPolicyRequest {
    #[allow(missing_docs)] // documentation missing in model
pub fn bypass_policy_lockout_safety_check(&self) -> &::std::option::Option<::std::primitive::bool> {
    &self.bypass_policy_lockout_safety_check
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
pub fn policy_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.policy_name
}
}
impl PutKeyPolicyRequest {
    /// Creates a new builder-style object to manufacture [`PutKeyPolicyRequest`](crate::operation::put_key_policy::builders::PutKeyPolicyRequest).
    pub fn builder() -> crate::operation::put_key_policy::builders::PutKeyPolicyRequestBuilder {
        crate::operation::put_key_policy::builders::PutKeyPolicyRequestBuilder::default()
    }
}

/// A builder for [`PutKeyPolicyRequest`](crate::operation::operation::PutKeyPolicyRequest).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct PutKeyPolicyRequestBuilder {
    pub(crate) bypass_policy_lockout_safety_check: ::std::option::Option<::std::primitive::bool>,
pub(crate) key_id: ::std::option::Option<::std::string::String>,
pub(crate) policy: ::std::option::Option<::std::string::String>,
pub(crate) policy_name: ::std::option::Option<::std::string::String>,
}
impl PutKeyPolicyRequestBuilder {
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
pub fn policy_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.policy_name = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_policy_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.policy_name = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_policy_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.policy_name
}
    /// Consumes the builder and constructs a [`PutKeyPolicyRequest`](crate::operation::operation::PutKeyPolicyRequest).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::put_key_policy::PutKeyPolicyRequest,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::put_key_policy::PutKeyPolicyRequest {
            bypass_policy_lockout_safety_check: self.bypass_policy_lockout_safety_check,
key_id: self.key_id,
policy: self.policy,
policy_name: self.policy_name,
        })
    }
}
