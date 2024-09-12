// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateKeyResponse {
    #[allow(missing_docs)] // documentation missing in model
pub key_metadata: ::std::option::Option<kms::types::KeyMetadata>,
}
impl CreateKeyResponse {
    #[allow(missing_docs)] // documentation missing in model
pub fn key_metadata(&self) -> &::std::option::Option<kms::types::KeyMetadata> {
    &self.key_metadata
}
}
impl CreateKeyResponse {
    /// Creates a new builder-style object to manufacture [`CreateKeyResponse`](crate::operation::create_key::builders::CreateKeyResponse).
    pub fn builder() -> crate::operation::create_key::builders::CreateKeyResponseBuilder {
        crate::operation::create_key::builders::CreateKeyResponseBuilder::default()
    }
}

/// A builder for [`CreateKeyResponse`](crate::operation::operation::CreateKeyResponse).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateKeyResponseBuilder {
    pub(crate) key_metadata: ::std::option::Option<kms::types::KeyMetadata>,
}
impl CreateKeyResponseBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn key_metadata(mut self, input: impl ::std::convert::Into<kms::types::KeyMetadata>) -> Self {
    self.key_metadata = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_key_metadata(mut self, input: ::std::option::Option<kms::types::KeyMetadata>) -> Self {
    self.key_metadata = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_key_metadata(&self) -> &::std::option::Option<kms::types::KeyMetadata> {
    &self.key_metadata
}
    /// Consumes the builder and constructs a [`CreateKeyResponse`](crate::operation::operation::CreateKeyResponse).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_key::CreateKeyResponse,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::create_key::CreateKeyResponse {
            key_metadata: self.key_metadata,
        })
    }
}
