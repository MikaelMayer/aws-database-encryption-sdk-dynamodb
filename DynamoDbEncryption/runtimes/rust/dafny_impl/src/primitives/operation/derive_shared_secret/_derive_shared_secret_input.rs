// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeriveSharedSecretInput {
    #[allow(missing_docs)] // documentation missing in model
pub ecc_curve: ::std::option::Option<crate::primitives::types::EcdhCurveSpec>,
#[allow(missing_docs)] // documentation missing in model
pub private_key: ::std::option::Option<crate::primitives::types::EccPrivateKey>,
#[allow(missing_docs)] // documentation missing in model
pub public_key: ::std::option::Option<crate::primitives::types::EccPublicKey>,
}
impl DeriveSharedSecretInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn ecc_curve(&self) -> &::std::option::Option<crate::primitives::types::EcdhCurveSpec> {
    &self.ecc_curve
}
#[allow(missing_docs)] // documentation missing in model
pub fn private_key(&self) -> &::std::option::Option<crate::primitives::types::EccPrivateKey> {
    &self.private_key
}
#[allow(missing_docs)] // documentation missing in model
pub fn public_key(&self) -> &::std::option::Option<crate::primitives::types::EccPublicKey> {
    &self.public_key
}
}
impl DeriveSharedSecretInput {
    /// Creates a new builder-style object to manufacture [`DeriveSharedSecretInput`](crate::primitives::operation::derive_shared_secret::builders::DeriveSharedSecretInput).
    pub fn builder() -> crate::primitives::operation::derive_shared_secret::builders::DeriveSharedSecretInputBuilder {
        crate::primitives::operation::derive_shared_secret::builders::DeriveSharedSecretInputBuilder::default()
    }
}

/// A builder for [`DeriveSharedSecretInput`](crate::primitives::operation::operation::DeriveSharedSecretInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeriveSharedSecretInputBuilder {
    pub(crate) ecc_curve: ::std::option::Option<crate::primitives::types::EcdhCurveSpec>,
pub(crate) private_key: ::std::option::Option<crate::primitives::types::EccPrivateKey>,
pub(crate) public_key: ::std::option::Option<crate::primitives::types::EccPublicKey>,
}
impl DeriveSharedSecretInputBuilder {
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
#[allow(missing_docs)] // documentation missing in model
pub fn private_key(mut self, input: impl ::std::convert::Into<crate::primitives::types::EccPrivateKey>) -> Self {
    self.private_key = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_private_key(mut self, input: ::std::option::Option<crate::primitives::types::EccPrivateKey>) -> Self {
    self.private_key = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_private_key(&self) -> &::std::option::Option<crate::primitives::types::EccPrivateKey> {
    &self.private_key
}
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
    /// Consumes the builder and constructs a [`DeriveSharedSecretInput`](crate::primitives::operation::operation::DeriveSharedSecretInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::primitives::operation::derive_shared_secret::DeriveSharedSecretInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::primitives::operation::derive_shared_secret::DeriveSharedSecretInput {
            ecc_curve: self.ecc_curve,
private_key: self.private_key,
public_key: self.public_key,
        })
    }
}