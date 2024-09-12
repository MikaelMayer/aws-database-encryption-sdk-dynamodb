// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeKeyResponse {
    #[allow(missing_docs)] // documentation missing in model
pub key_metadata: ::std::option::Option<kms::types::KeyMetadata>,
}
impl DescribeKeyResponse {
    #[allow(missing_docs)] // documentation missing in model
pub fn key_metadata(&self) -> &::std::option::Option<kms::types::KeyMetadata> {
    &self.key_metadata
}
}
impl DescribeKeyResponse {
    /// Creates a new builder-style object to manufacture [`DescribeKeyResponse`](crate::operation::describe_key::builders::DescribeKeyResponse).
    pub fn builder() -> crate::operation::describe_key::builders::DescribeKeyResponseBuilder {
        crate::operation::describe_key::builders::DescribeKeyResponseBuilder::default()
    }
}

/// A builder for [`DescribeKeyResponse`](crate::operation::operation::DescribeKeyResponse).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeKeyResponseBuilder {
    pub(crate) key_metadata: ::std::option::Option<kms::types::KeyMetadata>,
}
impl DescribeKeyResponseBuilder {
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
    /// Consumes the builder and constructs a [`DescribeKeyResponse`](crate::operation::operation::DescribeKeyResponse).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_key::DescribeKeyResponse,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_key::DescribeKeyResponse {
            key_metadata: self.key_metadata,
        })
    }
}
