// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DecompressPublicKeyOutput {
    #[allow(missing_docs)] // documentation missing in model
pub public_key: ::std::option::Option<crate::primitives::types::EccPublicKey>,
}
impl DecompressPublicKeyOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn public_key(&self) -> &::std::option::Option<crate::primitives::types::EccPublicKey> {
    &self.public_key
}
}
impl DecompressPublicKeyOutput {
    /// Creates a new builder-style object to manufacture [`DecompressPublicKeyOutput`](crate::primitives::operation::decompress_public_key::builders::DecompressPublicKeyOutput).
    pub fn builder() -> crate::primitives::operation::decompress_public_key::builders::DecompressPublicKeyOutputBuilder {
        crate::primitives::operation::decompress_public_key::builders::DecompressPublicKeyOutputBuilder::default()
    }
}

/// A builder for [`DecompressPublicKeyOutput`](crate::primitives::operation::operation::DecompressPublicKeyOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DecompressPublicKeyOutputBuilder {
    pub(crate) public_key: ::std::option::Option<crate::primitives::types::EccPublicKey>,
}
impl DecompressPublicKeyOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn public_key(mut self, input: impl ::std::convert::Into<crate::primitives::types::EccPublicKey>) -> Self {
    self.public_key = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_public_key(mut self, input: ::std::option::Option<crate::primitives::types::EccPublicKey>) -> Self {
    self.public_key = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_public_key(&self) -> &::std::option::Option<crate::primitives::types::EccPublicKey> {
    &self.public_key
}
    /// Consumes the builder and constructs a [`DecompressPublicKeyOutput`](crate::primitives::operation::operation::DecompressPublicKeyOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::primitives::operation::decompress_public_key::DecompressPublicKeyOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::primitives::operation::decompress_public_key::DecompressPublicKeyOutput {
            public_key: self.public_key,
        })
    }
}
