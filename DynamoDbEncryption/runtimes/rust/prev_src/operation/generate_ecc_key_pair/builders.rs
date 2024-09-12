// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::generate_ecc_key_pair::_generate_ecc_key_pair_output::GenerateEccKeyPairOutputBuilder;

pub use crate::operation::generate_ecc_key_pair::_generate_ecc_key_pair_input::GenerateEccKeyPairInputBuilder;

impl GenerateEccKeyPairInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        crate::operation::generate_ecc_key_pair::GenerateEccKeyPairOutput,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.generate_ecc_key_pair();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GenerateEccKeyPair`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GenerateEccKeyPairFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::generate_ecc_key_pair::builders::GenerateEccKeyPairInputBuilder,
}
impl GenerateEccKeyPairFluentBuilder {
    /// Creates a new `GenerateEccKeyPair`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the GenerateEccKeyPair as a reference.
    pub fn as_input(&self) -> &crate::operation::generate_ecc_key_pair::builders::GenerateEccKeyPairInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::generate_ecc_key_pair::GenerateEccKeyPairOutput,
        crate::types::error::Error,
    > {
        let input = self
            .inner
            .build()
            // Using Opaque since we don't have a validation-specific error yet.
            // Operations' models don't declare their own validation error,
            // and smithy-rs seems to not generate a ValidationError case unless there is.
            // Vanilla smithy-rs uses SdkError::construction_failure, but we aren't using SdkError.
            .map_err(|mut e| crate::types::error::Error::Opaque {
                obj: ::dafny_runtime::Object::from_ref(&mut e as &mut dyn ::std::any::Any)
            })?;
        crate::operation::generate_ecc_key_pair::GenerateEccKeyPair::send(&self.client, input).await
    }

    #[allow(missing_docs)] // documentation missing in model
pub fn ecc_curve(mut self, input: impl ::std::convert::Into<primitives::types::EcdhCurveSpec>) -> Self {
    self.inner = self.inner.ecc_curve(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_ecc_curve(mut self, input: ::std::option::Option<primitives::types::EcdhCurveSpec>) -> Self {
    self.inner = self.inner.set_ecc_curve(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_ecc_curve(&self) -> &::std::option::Option<primitives::types::EcdhCurveSpec> {
    self.inner.get_ecc_curve()
}
}
