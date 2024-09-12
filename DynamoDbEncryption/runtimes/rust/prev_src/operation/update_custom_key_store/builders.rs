// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::update_custom_key_store::_update_custom_key_store_response::UpdateCustomKeyStoreResponseBuilder;

pub use crate::operation::update_custom_key_store::_update_custom_key_store_request::UpdateCustomKeyStoreRequestBuilder;

impl UpdateCustomKeyStoreRequestBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        crate::operation::update_custom_key_store::UpdateCustomKeyStoreResponse,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.update_custom_key_store();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateCustomKeyStore`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateCustomKeyStoreFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::update_custom_key_store::builders::UpdateCustomKeyStoreRequestBuilder,
}
impl UpdateCustomKeyStoreFluentBuilder {
    /// Creates a new `UpdateCustomKeyStore`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the UpdateCustomKeyStore as a reference.
    pub fn as_input(&self) -> &crate::operation::update_custom_key_store::builders::UpdateCustomKeyStoreRequestBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_custom_key_store::UpdateCustomKeyStoreResponse,
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
        crate::operation::update_custom_key_store::UpdateCustomKeyStore::send(&self.client, input).await
    }

    #[allow(missing_docs)] // documentation missing in model
pub fn cloud_hsm_cluster_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.cloud_hsm_cluster_id(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_cloud_hsm_cluster_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_cloud_hsm_cluster_id(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_cloud_hsm_cluster_id(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_cloud_hsm_cluster_id()
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
#[allow(missing_docs)] // documentation missing in model
pub fn key_store_password(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.key_store_password(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_key_store_password(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_key_store_password(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_key_store_password(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_key_store_password()
}
#[allow(missing_docs)] // documentation missing in model
pub fn new_custom_key_store_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.new_custom_key_store_name(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_new_custom_key_store_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_new_custom_key_store_name(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_new_custom_key_store_name(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_new_custom_key_store_name()
}
#[allow(missing_docs)] // documentation missing in model
pub fn xks_proxy_authentication_credential(mut self, input: impl ::std::convert::Into<kms::types::XksProxyAuthenticationCredentialType>) -> Self {
    self.inner = self.inner.xks_proxy_authentication_credential(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_xks_proxy_authentication_credential(mut self, input: ::std::option::Option<kms::types::XksProxyAuthenticationCredentialType>) -> Self {
    self.inner = self.inner.set_xks_proxy_authentication_credential(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_xks_proxy_authentication_credential(&self) -> &::std::option::Option<kms::types::XksProxyAuthenticationCredentialType> {
    self.inner.get_xks_proxy_authentication_credential()
}
#[allow(missing_docs)] // documentation missing in model
pub fn xks_proxy_connectivity(mut self, input: impl ::std::convert::Into<kms::types::XksProxyConnectivityType>) -> Self {
    self.inner = self.inner.xks_proxy_connectivity(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_xks_proxy_connectivity(mut self, input: ::std::option::Option<kms::types::XksProxyConnectivityType>) -> Self {
    self.inner = self.inner.set_xks_proxy_connectivity(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_xks_proxy_connectivity(&self) -> &::std::option::Option<kms::types::XksProxyConnectivityType> {
    self.inner.get_xks_proxy_connectivity()
}
#[allow(missing_docs)] // documentation missing in model
pub fn xks_proxy_uri_endpoint(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.xks_proxy_uri_endpoint(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_xks_proxy_uri_endpoint(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_xks_proxy_uri_endpoint(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_xks_proxy_uri_endpoint(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_xks_proxy_uri_endpoint()
}
#[allow(missing_docs)] // documentation missing in model
pub fn xks_proxy_uri_path(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.xks_proxy_uri_path(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_xks_proxy_uri_path(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_xks_proxy_uri_path(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_xks_proxy_uri_path(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_xks_proxy_uri_path()
}
#[allow(missing_docs)] // documentation missing in model
pub fn xks_proxy_vpc_endpoint_service_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.xks_proxy_vpc_endpoint_service_name(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_xks_proxy_vpc_endpoint_service_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_xks_proxy_vpc_endpoint_service_name(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_xks_proxy_vpc_endpoint_service_name(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_xks_proxy_vpc_endpoint_service_name()
}
}
