// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateAwsKmsHierarchicalKeyringInput {
    #[allow(missing_docs)] // documentation missing in model
pub branch_key_id: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub branch_key_id_supplier: ::std::option::Option<material_providers::types::branch_key_id_supplier::BranchKeyIdSupplierRef>,
#[allow(missing_docs)] // documentation missing in model
pub cache: ::std::option::Option<material_providers::types::CacheType>,
#[allow(missing_docs)] // documentation missing in model
pub key_store: ::std::option::Option<key_store::client::Client>,
#[allow(missing_docs)] // documentation missing in model
pub ttl_seconds: ::std::option::Option<::std::primitive::i64>,
}
impl CreateAwsKmsHierarchicalKeyringInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn branch_key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.branch_key_id
}
#[allow(missing_docs)] // documentation missing in model
pub fn branch_key_id_supplier(&self) -> &::std::option::Option<material_providers::types::branch_key_id_supplier::BranchKeyIdSupplierRef> {
    &self.branch_key_id_supplier
}
#[allow(missing_docs)] // documentation missing in model
pub fn cache(&self) -> &::std::option::Option<material_providers::types::CacheType> {
    &self.cache
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_store(&self) -> &::std::option::Option<key_store::client::Client> {
    &self.key_store
}
#[allow(missing_docs)] // documentation missing in model
pub fn ttl_seconds(&self) -> &::std::option::Option<::std::primitive::i64> {
    &self.ttl_seconds
}
}
impl CreateAwsKmsHierarchicalKeyringInput {
    /// Creates a new builder-style object to manufacture [`CreateAwsKmsHierarchicalKeyringInput`](crate::operation::create_aws_kms_hierarchical_keyring::builders::CreateAwsKmsHierarchicalKeyringInput).
    pub fn builder() -> crate::operation::create_aws_kms_hierarchical_keyring::builders::CreateAwsKmsHierarchicalKeyringInputBuilder {
        crate::operation::create_aws_kms_hierarchical_keyring::builders::CreateAwsKmsHierarchicalKeyringInputBuilder::default()
    }
}

/// A builder for [`CreateAwsKmsHierarchicalKeyringInput`](crate::operation::operation::CreateAwsKmsHierarchicalKeyringInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateAwsKmsHierarchicalKeyringInputBuilder {
    pub(crate) branch_key_id: ::std::option::Option<::std::string::String>,
pub(crate) branch_key_id_supplier: ::std::option::Option<material_providers::types::branch_key_id_supplier::BranchKeyIdSupplierRef>,
pub(crate) cache: ::std::option::Option<material_providers::types::CacheType>,
pub(crate) key_store: ::std::option::Option<key_store::client::Client>,
pub(crate) ttl_seconds: ::std::option::Option<::std::primitive::i64>,
}
impl CreateAwsKmsHierarchicalKeyringInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn branch_key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.branch_key_id = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_branch_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.branch_key_id = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_branch_key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.branch_key_id
}
#[allow(missing_docs)] // documentation missing in model
pub fn branch_key_id_supplier(mut self, input: impl ::std::convert::Into<material_providers::types::branch_key_id_supplier::BranchKeyIdSupplierRef>) -> Self {
    self.branch_key_id_supplier = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_branch_key_id_supplier(mut self, input: ::std::option::Option<material_providers::types::branch_key_id_supplier::BranchKeyIdSupplierRef>) -> Self {
    self.branch_key_id_supplier = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_branch_key_id_supplier(&self) -> &::std::option::Option<material_providers::types::branch_key_id_supplier::BranchKeyIdSupplierRef> {
    &self.branch_key_id_supplier
}
#[allow(missing_docs)] // documentation missing in model
pub fn cache(mut self, input: impl ::std::convert::Into<material_providers::types::CacheType>) -> Self {
    self.cache = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_cache(mut self, input: ::std::option::Option<material_providers::types::CacheType>) -> Self {
    self.cache = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_cache(&self) -> &::std::option::Option<material_providers::types::CacheType> {
    &self.cache
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_store(mut self, input: impl ::std::convert::Into<key_store::client::Client>) -> Self {
    self.key_store = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_key_store(mut self, input: ::std::option::Option<key_store::client::Client>) -> Self {
    self.key_store = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_key_store(&self) -> &::std::option::Option<key_store::client::Client> {
    &self.key_store
}
#[allow(missing_docs)] // documentation missing in model
pub fn ttl_seconds(mut self, input: impl ::std::convert::Into<::std::primitive::i64>) -> Self {
    self.ttl_seconds = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_ttl_seconds(mut self, input: ::std::option::Option<::std::primitive::i64>) -> Self {
    self.ttl_seconds = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_ttl_seconds(&self) -> &::std::option::Option<::std::primitive::i64> {
    &self.ttl_seconds
}
    /// Consumes the builder and constructs a [`CreateAwsKmsHierarchicalKeyringInput`](crate::operation::operation::CreateAwsKmsHierarchicalKeyringInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_aws_kms_hierarchical_keyring::CreateAwsKmsHierarchicalKeyringInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::create_aws_kms_hierarchical_keyring::CreateAwsKmsHierarchicalKeyringInput {
            branch_key_id: self.branch_key_id,
branch_key_id_supplier: self.branch_key_id_supplier,
cache: self.cache,
key_store: self.key_store,
ttl_seconds: self.ttl_seconds,
        })
    }
}