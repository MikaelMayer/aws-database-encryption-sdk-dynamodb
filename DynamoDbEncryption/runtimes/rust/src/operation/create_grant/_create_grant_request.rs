// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateGrantRequest {
    #[allow(missing_docs)] // documentation missing in model
pub constraints: ::std::option::Option<kms::types::GrantConstraints>,
#[allow(missing_docs)] // documentation missing in model
pub dry_run: ::std::option::Option<::std::primitive::bool>,
#[allow(missing_docs)] // documentation missing in model
pub grant_tokens: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
#[allow(missing_docs)] // documentation missing in model
pub grantee_principal: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub key_id: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub name: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub operations: ::std::option::Option<::std::vec::Vec<kms::types::GrantOperation>>,
#[allow(missing_docs)] // documentation missing in model
pub retiring_principal: ::std::option::Option<::std::string::String>,
}
impl CreateGrantRequest {
    #[allow(missing_docs)] // documentation missing in model
pub fn constraints(&self) -> &::std::option::Option<kms::types::GrantConstraints> {
    &self.constraints
}
#[allow(missing_docs)] // documentation missing in model
pub fn dry_run(&self) -> &::std::option::Option<::std::primitive::bool> {
    &self.dry_run
}
#[allow(missing_docs)] // documentation missing in model
pub fn grant_tokens(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
    &self.grant_tokens
}
#[allow(missing_docs)] // documentation missing in model
pub fn grantee_principal(&self) -> &::std::option::Option<::std::string::String> {
    &self.grantee_principal
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.key_id
}
#[allow(missing_docs)] // documentation missing in model
pub fn name(&self) -> &::std::option::Option<::std::string::String> {
    &self.name
}
#[allow(missing_docs)] // documentation missing in model
pub fn operations(&self) -> &::std::option::Option<::std::vec::Vec<kms::types::GrantOperation>> {
    &self.operations
}
#[allow(missing_docs)] // documentation missing in model
pub fn retiring_principal(&self) -> &::std::option::Option<::std::string::String> {
    &self.retiring_principal
}
}
impl CreateGrantRequest {
    /// Creates a new builder-style object to manufacture [`CreateGrantRequest`](crate::operation::create_grant::builders::CreateGrantRequest).
    pub fn builder() -> crate::operation::create_grant::builders::CreateGrantRequestBuilder {
        crate::operation::create_grant::builders::CreateGrantRequestBuilder::default()
    }
}

/// A builder for [`CreateGrantRequest`](crate::operation::operation::CreateGrantRequest).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateGrantRequestBuilder {
    pub(crate) constraints: ::std::option::Option<kms::types::GrantConstraints>,
pub(crate) dry_run: ::std::option::Option<::std::primitive::bool>,
pub(crate) grant_tokens: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
pub(crate) grantee_principal: ::std::option::Option<::std::string::String>,
pub(crate) key_id: ::std::option::Option<::std::string::String>,
pub(crate) name: ::std::option::Option<::std::string::String>,
pub(crate) operations: ::std::option::Option<::std::vec::Vec<kms::types::GrantOperation>>,
pub(crate) retiring_principal: ::std::option::Option<::std::string::String>,
}
impl CreateGrantRequestBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn constraints(mut self, input: impl ::std::convert::Into<kms::types::GrantConstraints>) -> Self {
    self.constraints = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_constraints(mut self, input: ::std::option::Option<kms::types::GrantConstraints>) -> Self {
    self.constraints = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_constraints(&self) -> &::std::option::Option<kms::types::GrantConstraints> {
    &self.constraints
}
#[allow(missing_docs)] // documentation missing in model
pub fn dry_run(mut self, input: impl ::std::convert::Into<::std::primitive::bool>) -> Self {
    self.dry_run = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_dry_run(mut self, input: ::std::option::Option<::std::primitive::bool>) -> Self {
    self.dry_run = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_dry_run(&self) -> &::std::option::Option<::std::primitive::bool> {
    &self.dry_run
}
#[allow(missing_docs)] // documentation missing in model
pub fn grant_tokens(mut self, input: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>) -> Self {
    self.grant_tokens = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_grant_tokens(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
    self.grant_tokens = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_grant_tokens(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
    &self.grant_tokens
}
#[allow(missing_docs)] // documentation missing in model
pub fn grantee_principal(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.grantee_principal = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_grantee_principal(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.grantee_principal = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_grantee_principal(&self) -> &::std::option::Option<::std::string::String> {
    &self.grantee_principal
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
pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.name = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.name = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.name
}
#[allow(missing_docs)] // documentation missing in model
pub fn operations(mut self, input: impl ::std::convert::Into<::std::vec::Vec<kms::types::GrantOperation>>) -> Self {
    self.operations = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_operations(mut self, input: ::std::option::Option<::std::vec::Vec<kms::types::GrantOperation>>) -> Self {
    self.operations = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_operations(&self) -> &::std::option::Option<::std::vec::Vec<kms::types::GrantOperation>> {
    &self.operations
}
#[allow(missing_docs)] // documentation missing in model
pub fn retiring_principal(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.retiring_principal = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_retiring_principal(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.retiring_principal = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_retiring_principal(&self) -> &::std::option::Option<::std::string::String> {
    &self.retiring_principal
}
    /// Consumes the builder and constructs a [`CreateGrantRequest`](crate::operation::operation::CreateGrantRequest).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_grant::CreateGrantRequest,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::create_grant::CreateGrantRequest {
            constraints: self.constraints,
dry_run: self.dry_run,
grant_tokens: self.grant_tokens,
grantee_principal: self.grantee_principal,
key_id: self.key_id,
name: self.name,
operations: self.operations,
retiring_principal: self.retiring_principal,
        })
    }
}
