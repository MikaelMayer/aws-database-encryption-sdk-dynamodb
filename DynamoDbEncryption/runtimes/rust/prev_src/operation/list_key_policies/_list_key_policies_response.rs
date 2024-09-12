// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListKeyPoliciesResponse {
    #[allow(missing_docs)] // documentation missing in model
pub next_marker: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub policy_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
#[allow(missing_docs)] // documentation missing in model
pub truncated: ::std::option::Option<::std::primitive::bool>,
}
impl ListKeyPoliciesResponse {
    #[allow(missing_docs)] // documentation missing in model
pub fn next_marker(&self) -> &::std::option::Option<::std::string::String> {
    &self.next_marker
}
#[allow(missing_docs)] // documentation missing in model
pub fn policy_names(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
    &self.policy_names
}
#[allow(missing_docs)] // documentation missing in model
pub fn truncated(&self) -> &::std::option::Option<::std::primitive::bool> {
    &self.truncated
}
}
impl ListKeyPoliciesResponse {
    /// Creates a new builder-style object to manufacture [`ListKeyPoliciesResponse`](crate::operation::list_key_policies::builders::ListKeyPoliciesResponse).
    pub fn builder() -> crate::operation::list_key_policies::builders::ListKeyPoliciesResponseBuilder {
        crate::operation::list_key_policies::builders::ListKeyPoliciesResponseBuilder::default()
    }
}

/// A builder for [`ListKeyPoliciesResponse`](crate::operation::operation::ListKeyPoliciesResponse).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListKeyPoliciesResponseBuilder {
    pub(crate) next_marker: ::std::option::Option<::std::string::String>,
pub(crate) policy_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
pub(crate) truncated: ::std::option::Option<::std::primitive::bool>,
}
impl ListKeyPoliciesResponseBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn next_marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.next_marker = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_next_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.next_marker = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_next_marker(&self) -> &::std::option::Option<::std::string::String> {
    &self.next_marker
}
#[allow(missing_docs)] // documentation missing in model
pub fn policy_names(mut self, input: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>) -> Self {
    self.policy_names = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_policy_names(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
    self.policy_names = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_policy_names(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
    &self.policy_names
}
#[allow(missing_docs)] // documentation missing in model
pub fn truncated(mut self, input: impl ::std::convert::Into<::std::primitive::bool>) -> Self {
    self.truncated = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_truncated(mut self, input: ::std::option::Option<::std::primitive::bool>) -> Self {
    self.truncated = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_truncated(&self) -> &::std::option::Option<::std::primitive::bool> {
    &self.truncated
}
    /// Consumes the builder and constructs a [`ListKeyPoliciesResponse`](crate::operation::operation::ListKeyPoliciesResponse).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_key_policies::ListKeyPoliciesResponse,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::list_key_policies::ListKeyPoliciesResponse {
            next_marker: self.next_marker,
policy_names: self.policy_names,
truncated: self.truncated,
        })
    }
}
