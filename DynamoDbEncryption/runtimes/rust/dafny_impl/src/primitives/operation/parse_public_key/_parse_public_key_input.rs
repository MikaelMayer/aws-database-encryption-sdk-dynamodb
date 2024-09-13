// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ParsePublicKeyInput {
    #[allow(missing_docs)] // documentation missing in model
pub public_key: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl ParsePublicKeyInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn public_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.public_key
}
}
impl ParsePublicKeyInput {
    /// Creates a new builder-style object to manufacture [`ParsePublicKeyInput`](crate::primitives::operation::parse_public_key::builders::ParsePublicKeyInput).
    pub fn builder() -> crate::primitives::operation::parse_public_key::builders::ParsePublicKeyInputBuilder {
        crate::primitives::operation::parse_public_key::builders::ParsePublicKeyInputBuilder::default()
    }
}

/// A builder for [`ParsePublicKeyInput`](crate::primitives::operation::operation::ParsePublicKeyInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ParsePublicKeyInputBuilder {
    pub(crate) public_key: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl ParsePublicKeyInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn public_key(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
    self.public_key = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_public_key(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
    self.public_key = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_public_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.public_key
}
    /// Consumes the builder and constructs a [`ParsePublicKeyInput`](crate::primitives::operation::operation::ParsePublicKeyInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::primitives::operation::parse_public_key::ParsePublicKeyInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::primitives::operation::parse_public_key::ParsePublicKeyInput {
            public_key: self.public_key,
        })
    }
}