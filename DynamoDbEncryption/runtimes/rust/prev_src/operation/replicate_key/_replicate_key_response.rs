// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ReplicateKeyResponse {
    #[allow(missing_docs)] // documentation missing in model
pub replica_key_metadata: ::std::option::Option<kms::types::KeyMetadata>,
#[allow(missing_docs)] // documentation missing in model
pub replica_policy: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub replica_tags: ::std::option::Option<::std::vec::Vec<kms::types::Tag>>,
}
impl ReplicateKeyResponse {
    #[allow(missing_docs)] // documentation missing in model
pub fn replica_key_metadata(&self) -> &::std::option::Option<kms::types::KeyMetadata> {
    &self.replica_key_metadata
}
#[allow(missing_docs)] // documentation missing in model
pub fn replica_policy(&self) -> &::std::option::Option<::std::string::String> {
    &self.replica_policy
}
#[allow(missing_docs)] // documentation missing in model
pub fn replica_tags(&self) -> &::std::option::Option<::std::vec::Vec<kms::types::Tag>> {
    &self.replica_tags
}
}
impl ReplicateKeyResponse {
    /// Creates a new builder-style object to manufacture [`ReplicateKeyResponse`](crate::operation::replicate_key::builders::ReplicateKeyResponse).
    pub fn builder() -> crate::operation::replicate_key::builders::ReplicateKeyResponseBuilder {
        crate::operation::replicate_key::builders::ReplicateKeyResponseBuilder::default()
    }
}

/// A builder for [`ReplicateKeyResponse`](crate::operation::operation::ReplicateKeyResponse).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ReplicateKeyResponseBuilder {
    pub(crate) replica_key_metadata: ::std::option::Option<kms::types::KeyMetadata>,
pub(crate) replica_policy: ::std::option::Option<::std::string::String>,
pub(crate) replica_tags: ::std::option::Option<::std::vec::Vec<kms::types::Tag>>,
}
impl ReplicateKeyResponseBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn replica_key_metadata(mut self, input: impl ::std::convert::Into<kms::types::KeyMetadata>) -> Self {
    self.replica_key_metadata = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_replica_key_metadata(mut self, input: ::std::option::Option<kms::types::KeyMetadata>) -> Self {
    self.replica_key_metadata = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_replica_key_metadata(&self) -> &::std::option::Option<kms::types::KeyMetadata> {
    &self.replica_key_metadata
}
#[allow(missing_docs)] // documentation missing in model
pub fn replica_policy(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.replica_policy = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_replica_policy(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.replica_policy = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_replica_policy(&self) -> &::std::option::Option<::std::string::String> {
    &self.replica_policy
}
#[allow(missing_docs)] // documentation missing in model
pub fn replica_tags(mut self, input: impl ::std::convert::Into<::std::vec::Vec<kms::types::Tag>>) -> Self {
    self.replica_tags = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_replica_tags(mut self, input: ::std::option::Option<::std::vec::Vec<kms::types::Tag>>) -> Self {
    self.replica_tags = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_replica_tags(&self) -> &::std::option::Option<::std::vec::Vec<kms::types::Tag>> {
    &self.replica_tags
}
    /// Consumes the builder and constructs a [`ReplicateKeyResponse`](crate::operation::operation::ReplicateKeyResponse).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::replicate_key::ReplicateKeyResponse,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::replicate_key::ReplicateKeyResponse {
            replica_key_metadata: self.replica_key_metadata,
replica_policy: self.replica_policy,
replica_tags: self.replica_tags,
        })
    }
}
