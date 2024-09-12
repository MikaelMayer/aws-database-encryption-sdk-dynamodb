// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetKeyPolicyResponse {
    #[allow(missing_docs)] // documentation missing in model
pub policy: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub policy_name: ::std::option::Option<::std::string::String>,
}
impl GetKeyPolicyResponse {
    #[allow(missing_docs)] // documentation missing in model
pub fn policy(&self) -> &::std::option::Option<::std::string::String> {
    &self.policy
}
#[allow(missing_docs)] // documentation missing in model
pub fn policy_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.policy_name
}
}
impl GetKeyPolicyResponse {
    /// Creates a new builder-style object to manufacture [`GetKeyPolicyResponse`](crate::operation::get_key_policy::builders::GetKeyPolicyResponse).
    pub fn builder() -> crate::operation::get_key_policy::builders::GetKeyPolicyResponseBuilder {
        crate::operation::get_key_policy::builders::GetKeyPolicyResponseBuilder::default()
    }
}

/// A builder for [`GetKeyPolicyResponse`](crate::operation::operation::GetKeyPolicyResponse).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetKeyPolicyResponseBuilder {
    pub(crate) policy: ::std::option::Option<::std::string::String>,
pub(crate) policy_name: ::std::option::Option<::std::string::String>,
}
impl GetKeyPolicyResponseBuilder {
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
    /// Consumes the builder and constructs a [`GetKeyPolicyResponse`](crate::operation::operation::GetKeyPolicyResponse).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_key_policy::GetKeyPolicyResponse,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_key_policy::GetKeyPolicyResponse {
            policy: self.policy,
policy_name: self.policy_name,
        })
    }
}
