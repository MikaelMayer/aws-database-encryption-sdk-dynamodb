// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GenerateEcdsaSignatureKeyInput {
    #[allow(missing_docs)] // documentation missing in model
pub signature_algorithm: ::std::option::Option<crate::primitives::types::EcdsaSignatureAlgorithm>,
}
impl GenerateEcdsaSignatureKeyInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn signature_algorithm(&self) -> &::std::option::Option<crate::primitives::types::EcdsaSignatureAlgorithm> {
    &self.signature_algorithm
}
}
impl GenerateEcdsaSignatureKeyInput {
    /// Creates a new builder-style object to manufacture [`GenerateEcdsaSignatureKeyInput`](crate::primitives::operation::generate_ecdsa_signature_key::builders::GenerateEcdsaSignatureKeyInput).
    pub fn builder() -> crate::primitives::operation::generate_ecdsa_signature_key::builders::GenerateEcdsaSignatureKeyInputBuilder {
        crate::primitives::operation::generate_ecdsa_signature_key::builders::GenerateEcdsaSignatureKeyInputBuilder::default()
    }
}

/// A builder for [`GenerateEcdsaSignatureKeyInput`](crate::primitives::operation::operation::GenerateEcdsaSignatureKeyInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GenerateEcdsaSignatureKeyInputBuilder {
    pub(crate) signature_algorithm: ::std::option::Option<crate::primitives::types::EcdsaSignatureAlgorithm>,
}
impl GenerateEcdsaSignatureKeyInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn signature_algorithm(mut self, input: impl ::std::convert::Into<crate::primitives::types::EcdsaSignatureAlgorithm>) -> Self {
    self.signature_algorithm = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_signature_algorithm(mut self, input: ::std::option::Option<crate::primitives::types::EcdsaSignatureAlgorithm>) -> Self {
    self.signature_algorithm = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_signature_algorithm(&self) -> &::std::option::Option<crate::primitives::types::EcdsaSignatureAlgorithm> {
    &self.signature_algorithm
}
    /// Consumes the builder and constructs a [`GenerateEcdsaSignatureKeyInput`](crate::primitives::operation::operation::GenerateEcdsaSignatureKeyInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::primitives::operation::generate_ecdsa_signature_key::GenerateEcdsaSignatureKeyInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::primitives::operation::generate_ecdsa_signature_key::GenerateEcdsaSignatureKeyInput {
            signature_algorithm: self.signature_algorithm,
        })
    }
}
