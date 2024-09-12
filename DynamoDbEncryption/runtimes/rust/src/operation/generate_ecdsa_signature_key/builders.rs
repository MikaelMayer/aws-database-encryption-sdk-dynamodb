// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::generate_ecdsa_signature_key::_generate_ecdsa_signature_key_output::GenerateEcdsaSignatureKeyOutputBuilder;

pub use crate::operation::generate_ecdsa_signature_key::_generate_ecdsa_signature_key_input::GenerateEcdsaSignatureKeyInputBuilder;

impl GenerateEcdsaSignatureKeyInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        crate::operation::generate_ecdsa_signature_key::GenerateEcdsaSignatureKeyOutput,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.generate_ecdsa_signature_key();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GenerateEcdsaSignatureKey`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GenerateEcdsaSignatureKeyFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::generate_ecdsa_signature_key::builders::GenerateEcdsaSignatureKeyInputBuilder,
}
impl GenerateEcdsaSignatureKeyFluentBuilder {
    /// Creates a new `GenerateEcdsaSignatureKey`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the GenerateEcdsaSignatureKey as a reference.
    pub fn as_input(&self) -> &crate::operation::generate_ecdsa_signature_key::builders::GenerateEcdsaSignatureKeyInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::generate_ecdsa_signature_key::GenerateEcdsaSignatureKeyOutput,
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
        crate::operation::generate_ecdsa_signature_key::GenerateEcdsaSignatureKey::send(&self.client, input).await
    }

    #[allow(missing_docs)] // documentation missing in model
pub fn signature_algorithm(mut self, input: impl ::std::convert::Into<primitives::types::EcdsaSignatureAlgorithm>) -> Self {
    self.inner = self.inner.signature_algorithm(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_signature_algorithm(mut self, input: ::std::option::Option<primitives::types::EcdsaSignatureAlgorithm>) -> Self {
    self.inner = self.inner.set_signature_algorithm(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_signature_algorithm(&self) -> &::std::option::Option<primitives::types::EcdsaSignatureAlgorithm> {
    self.inner.get_signature_algorithm()
}
}
