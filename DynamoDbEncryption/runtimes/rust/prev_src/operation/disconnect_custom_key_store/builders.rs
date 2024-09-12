// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::disconnect_custom_key_store::_disconnect_custom_key_store_response::DisconnectCustomKeyStoreResponseBuilder;

pub use crate::operation::disconnect_custom_key_store::_disconnect_custom_key_store_request::DisconnectCustomKeyStoreRequestBuilder;

impl DisconnectCustomKeyStoreRequestBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        crate::operation::disconnect_custom_key_store::DisconnectCustomKeyStoreResponse,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.disconnect_custom_key_store();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DisconnectCustomKeyStore`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DisconnectCustomKeyStoreFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::disconnect_custom_key_store::builders::DisconnectCustomKeyStoreRequestBuilder,
}
impl DisconnectCustomKeyStoreFluentBuilder {
    /// Creates a new `DisconnectCustomKeyStore`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the DisconnectCustomKeyStore as a reference.
    pub fn as_input(&self) -> &crate::operation::disconnect_custom_key_store::builders::DisconnectCustomKeyStoreRequestBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::disconnect_custom_key_store::DisconnectCustomKeyStoreResponse,
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
        crate::operation::disconnect_custom_key_store::DisconnectCustomKeyStore::send(&self.client, input).await
    }

    #[allow(missing_docs)] // documentation missing in model
pub fn custom_key_store_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.custom_key_store_id(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_custom_key_store_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_custom_key_store_id(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_custom_key_store_id(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_custom_key_store_id()
}
}
