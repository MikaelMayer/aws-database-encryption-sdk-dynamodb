// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GenerateEccKeyPairOutput {
    #[allow(missing_docs)] // documentation missing in model
pub ecc_curve: ::std::option::Option<primitives::types::EcdhCurveSpec>,
#[allow(missing_docs)] // documentation missing in model
pub private_key: ::std::option::Option<primitives::types::EccPrivateKey>,
#[allow(missing_docs)] // documentation missing in model
pub public_key: ::std::option::Option<primitives::types::EccPublicKey>,
}
impl GenerateEccKeyPairOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn ecc_curve(&self) -> &::std::option::Option<primitives::types::EcdhCurveSpec> {
    &self.ecc_curve
}
#[allow(missing_docs)] // documentation missing in model
pub fn private_key(&self) -> &::std::option::Option<primitives::types::EccPrivateKey> {
    &self.private_key
}
#[allow(missing_docs)] // documentation missing in model
pub fn public_key(&self) -> &::std::option::Option<primitives::types::EccPublicKey> {
    &self.public_key
}
}
impl GenerateEccKeyPairOutput {
    /// Creates a new builder-style object to manufacture [`GenerateEccKeyPairOutput`](crate::operation::generate_ecc_key_pair::builders::GenerateEccKeyPairOutput).
    pub fn builder() -> crate::operation::generate_ecc_key_pair::builders::GenerateEccKeyPairOutputBuilder {
        crate::operation::generate_ecc_key_pair::builders::GenerateEccKeyPairOutputBuilder::default()
    }
}

/// A builder for [`GenerateEccKeyPairOutput`](crate::operation::operation::GenerateEccKeyPairOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GenerateEccKeyPairOutputBuilder {
    pub(crate) ecc_curve: ::std::option::Option<primitives::types::EcdhCurveSpec>,
pub(crate) private_key: ::std::option::Option<primitives::types::EccPrivateKey>,
pub(crate) public_key: ::std::option::Option<primitives::types::EccPublicKey>,
}
impl GenerateEccKeyPairOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn ecc_curve(mut self, input: impl ::std::convert::Into<primitives::types::EcdhCurveSpec>) -> Self {
    self.ecc_curve = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_ecc_curve(mut self, input: ::std::option::Option<primitives::types::EcdhCurveSpec>) -> Self {
    self.ecc_curve = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_ecc_curve(&self) -> &::std::option::Option<primitives::types::EcdhCurveSpec> {
    &self.ecc_curve
}
#[allow(missing_docs)] // documentation missing in model
pub fn private_key(mut self, input: impl ::std::convert::Into<primitives::types::EccPrivateKey>) -> Self {
    self.private_key = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_private_key(mut self, input: ::std::option::Option<primitives::types::EccPrivateKey>) -> Self {
    self.private_key = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_private_key(&self) -> &::std::option::Option<primitives::types::EccPrivateKey> {
    &self.private_key
}
#[allow(missing_docs)] // documentation missing in model
pub fn public_key(mut self, input: impl ::std::convert::Into<primitives::types::EccPublicKey>) -> Self {
    self.public_key = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_public_key(mut self, input: ::std::option::Option<primitives::types::EccPublicKey>) -> Self {
    self.public_key = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_public_key(&self) -> &::std::option::Option<primitives::types::EccPublicKey> {
    &self.public_key
}
    /// Consumes the builder and constructs a [`GenerateEccKeyPairOutput`](crate::operation::operation::GenerateEccKeyPairOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::generate_ecc_key_pair::GenerateEccKeyPairOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::generate_ecc_key_pair::GenerateEccKeyPairOutput {
            ecc_curve: self.ecc_curve,
private_key: self.private_key,
public_key: self.public_key,
        })
    }
}