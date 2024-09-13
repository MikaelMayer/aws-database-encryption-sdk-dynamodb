// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::rotate_key_on_demand::_rotate_key_on_demand_response::RotateKeyOnDemandResponseBuilder;

pub use crate::operation::rotate_key_on_demand::_rotate_key_on_demand_request::RotateKeyOnDemandRequestBuilder;

impl RotateKeyOnDemandRequestBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        crate::operation::rotate_key_on_demand::RotateKeyOnDemandResponse,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.rotate_key_on_demand();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `RotateKeyOnDemand`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct RotateKeyOnDemandFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::rotate_key_on_demand::builders::RotateKeyOnDemandRequestBuilder,
}
impl RotateKeyOnDemandFluentBuilder {
    /// Creates a new `RotateKeyOnDemand`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the RotateKeyOnDemand as a reference.
    pub fn as_input(&self) -> &crate::operation::rotate_key_on_demand::builders::RotateKeyOnDemandRequestBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::rotate_key_on_demand::RotateKeyOnDemandResponse,
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
        crate::operation::rotate_key_on_demand::RotateKeyOnDemand::send(&self.client, input).await
    }

    #[allow(missing_docs)] // documentation missing in model
pub fn key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.key_id(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_key_id(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_key_id(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_key_id()
}
}