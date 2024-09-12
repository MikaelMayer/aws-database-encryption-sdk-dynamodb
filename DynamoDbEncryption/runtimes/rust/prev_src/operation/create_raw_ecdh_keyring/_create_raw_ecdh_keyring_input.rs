// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateRawEcdhKeyringInput {
    #[allow(missing_docs)] // documentation missing in model
pub key_agreement_scheme: ::std::option::Option<material_providers::types::RawEcdhStaticConfigurations>,
#[allow(missing_docs)] // documentation missing in model
pub curve_spec: ::std::option::Option<primitives::types::EcdhCurveSpec>,
}
impl CreateRawEcdhKeyringInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn key_agreement_scheme(&self) -> &::std::option::Option<material_providers::types::RawEcdhStaticConfigurations> {
    &self.key_agreement_scheme
}
#[allow(missing_docs)] // documentation missing in model
pub fn curve_spec(&self) -> &::std::option::Option<primitives::types::EcdhCurveSpec> {
    &self.curve_spec
}
}
impl CreateRawEcdhKeyringInput {
    /// Creates a new builder-style object to manufacture [`CreateRawEcdhKeyringInput`](crate::operation::create_raw_ecdh_keyring::builders::CreateRawEcdhKeyringInput).
    pub fn builder() -> crate::operation::create_raw_ecdh_keyring::builders::CreateRawEcdhKeyringInputBuilder {
        crate::operation::create_raw_ecdh_keyring::builders::CreateRawEcdhKeyringInputBuilder::default()
    }
}

/// A builder for [`CreateRawEcdhKeyringInput`](crate::operation::operation::CreateRawEcdhKeyringInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateRawEcdhKeyringInputBuilder {
    pub(crate) key_agreement_scheme: ::std::option::Option<material_providers::types::RawEcdhStaticConfigurations>,
pub(crate) curve_spec: ::std::option::Option<primitives::types::EcdhCurveSpec>,
}
impl CreateRawEcdhKeyringInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn key_agreement_scheme(mut self, input: impl ::std::convert::Into<material_providers::types::RawEcdhStaticConfigurations>) -> Self {
    self.key_agreement_scheme = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_key_agreement_scheme(mut self, input: ::std::option::Option<material_providers::types::RawEcdhStaticConfigurations>) -> Self {
    self.key_agreement_scheme = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_key_agreement_scheme(&self) -> &::std::option::Option<material_providers::types::RawEcdhStaticConfigurations> {
    &self.key_agreement_scheme
}
#[allow(missing_docs)] // documentation missing in model
pub fn curve_spec(mut self, input: impl ::std::convert::Into<primitives::types::EcdhCurveSpec>) -> Self {
    self.curve_spec = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_curve_spec(mut self, input: ::std::option::Option<primitives::types::EcdhCurveSpec>) -> Self {
    self.curve_spec = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_curve_spec(&self) -> &::std::option::Option<primitives::types::EcdhCurveSpec> {
    &self.curve_spec
}
    /// Consumes the builder and constructs a [`CreateRawEcdhKeyringInput`](crate::operation::operation::CreateRawEcdhKeyringInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_raw_ecdh_keyring::CreateRawEcdhKeyringInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::create_raw_ecdh_keyring::CreateRawEcdhKeyringInput {
            key_agreement_scheme: self.key_agreement_scheme,
curve_spec: self.curve_spec,
        })
    }
}
