// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetKeyPolicyRequest {
    #[allow(missing_docs)] // documentation missing in model
pub key_id: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub policy_name: ::std::option::Option<::std::string::String>,
}
impl GetKeyPolicyRequest {
    #[allow(missing_docs)] // documentation missing in model
pub fn key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.key_id
}
#[allow(missing_docs)] // documentation missing in model
pub fn policy_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.policy_name
}
}
impl GetKeyPolicyRequest {
    /// Creates a new builder-style object to manufacture [`GetKeyPolicyRequest`](crate::operation::get_key_policy::builders::GetKeyPolicyRequest).
    pub fn builder() -> crate::operation::get_key_policy::builders::GetKeyPolicyRequestBuilder {
        crate::operation::get_key_policy::builders::GetKeyPolicyRequestBuilder::default()
    }
}

/// A builder for [`GetKeyPolicyRequest`](crate::operation::operation::GetKeyPolicyRequest).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetKeyPolicyRequestBuilder {
    pub(crate) key_id: ::std::option::Option<::std::string::String>,
pub(crate) policy_name: ::std::option::Option<::std::string::String>,
}
impl GetKeyPolicyRequestBuilder {
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
    /// Consumes the builder and constructs a [`GetKeyPolicyRequest`](crate::operation::operation::GetKeyPolicyRequest).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_key_policy::GetKeyPolicyRequest,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_key_policy::GetKeyPolicyRequest {
            key_id: self.key_id,
policy_name: self.policy_name,
        })
    }
}
