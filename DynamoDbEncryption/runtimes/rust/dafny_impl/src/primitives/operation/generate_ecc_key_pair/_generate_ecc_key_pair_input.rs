// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GenerateEccKeyPairInput {
    #[allow(missing_docs)] // documentation missing in model
pub ecc_curve: ::std::option::Option<crate::primitives::types::EcdhCurveSpec>,
}
impl GenerateEccKeyPairInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn ecc_curve(&self) -> &::std::option::Option<crate::primitives::types::EcdhCurveSpec> {
    &self.ecc_curve
}
}
impl GenerateEccKeyPairInput {
    /// Creates a new builder-style object to manufacture [`GenerateEccKeyPairInput`](crate::primitives::operation::generate_ecc_key_pair::builders::GenerateEccKeyPairInput).
    pub fn builder() -> crate::primitives::operation::generate_ecc_key_pair::builders::GenerateEccKeyPairInputBuilder {
        crate::primitives::operation::generate_ecc_key_pair::builders::GenerateEccKeyPairInputBuilder::default()
    }
}

/// A builder for [`GenerateEccKeyPairInput`](crate::primitives::operation::operation::GenerateEccKeyPairInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GenerateEccKeyPairInputBuilder {
    pub(crate) ecc_curve: ::std::option::Option<crate::primitives::types::EcdhCurveSpec>,
}
impl GenerateEccKeyPairInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn ecc_curve(mut self, input: impl ::std::convert::Into<crate::primitives::types::EcdhCurveSpec>) -> Self {
    self.ecc_curve = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_ecc_curve(mut self, input: ::std::option::Option<crate::primitives::types::EcdhCurveSpec>) -> Self {
    self.ecc_curve = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_ecc_curve(&self) -> &::std::option::Option<crate::primitives::types::EcdhCurveSpec> {
    &self.ecc_curve
}
    /// Consumes the builder and constructs a [`GenerateEccKeyPairInput`](crate::primitives::operation::operation::GenerateEccKeyPairInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::primitives::operation::generate_ecc_key_pair::GenerateEccKeyPairInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::primitives::operation::generate_ecc_key_pair::GenerateEccKeyPairInput {
            ecc_curve: self.ecc_curve,
        })
    }
}
