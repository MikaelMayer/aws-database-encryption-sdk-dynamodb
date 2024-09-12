// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateCustomKeyStoreRequest {
    #[allow(missing_docs)] // documentation missing in model
pub cloud_hsm_cluster_id: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub custom_key_store_id: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub key_store_password: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub new_custom_key_store_name: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub xks_proxy_authentication_credential: ::std::option::Option<kms::types::XksProxyAuthenticationCredentialType>,
#[allow(missing_docs)] // documentation missing in model
pub xks_proxy_connectivity: ::std::option::Option<kms::types::XksProxyConnectivityType>,
#[allow(missing_docs)] // documentation missing in model
pub xks_proxy_uri_endpoint: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub xks_proxy_uri_path: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub xks_proxy_vpc_endpoint_service_name: ::std::option::Option<::std::string::String>,
}
impl UpdateCustomKeyStoreRequest {
    #[allow(missing_docs)] // documentation missing in model
pub fn cloud_hsm_cluster_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.cloud_hsm_cluster_id
}
#[allow(missing_docs)] // documentation missing in model
pub fn custom_key_store_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.custom_key_store_id
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_store_password(&self) -> &::std::option::Option<::std::string::String> {
    &self.key_store_password
}
#[allow(missing_docs)] // documentation missing in model
pub fn new_custom_key_store_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.new_custom_key_store_name
}
#[allow(missing_docs)] // documentation missing in model
pub fn xks_proxy_authentication_credential(&self) -> &::std::option::Option<kms::types::XksProxyAuthenticationCredentialType> {
    &self.xks_proxy_authentication_credential
}
#[allow(missing_docs)] // documentation missing in model
pub fn xks_proxy_connectivity(&self) -> &::std::option::Option<kms::types::XksProxyConnectivityType> {
    &self.xks_proxy_connectivity
}
#[allow(missing_docs)] // documentation missing in model
pub fn xks_proxy_uri_endpoint(&self) -> &::std::option::Option<::std::string::String> {
    &self.xks_proxy_uri_endpoint
}
#[allow(missing_docs)] // documentation missing in model
pub fn xks_proxy_uri_path(&self) -> &::std::option::Option<::std::string::String> {
    &self.xks_proxy_uri_path
}
#[allow(missing_docs)] // documentation missing in model
pub fn xks_proxy_vpc_endpoint_service_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.xks_proxy_vpc_endpoint_service_name
}
}
impl UpdateCustomKeyStoreRequest {
    /// Creates a new builder-style object to manufacture [`UpdateCustomKeyStoreRequest`](crate::operation::update_custom_key_store::builders::UpdateCustomKeyStoreRequest).
    pub fn builder() -> crate::operation::update_custom_key_store::builders::UpdateCustomKeyStoreRequestBuilder {
        crate::operation::update_custom_key_store::builders::UpdateCustomKeyStoreRequestBuilder::default()
    }
}

/// A builder for [`UpdateCustomKeyStoreRequest`](crate::operation::operation::UpdateCustomKeyStoreRequest).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateCustomKeyStoreRequestBuilder {
    pub(crate) cloud_hsm_cluster_id: ::std::option::Option<::std::string::String>,
pub(crate) custom_key_store_id: ::std::option::Option<::std::string::String>,
pub(crate) key_store_password: ::std::option::Option<::std::string::String>,
pub(crate) new_custom_key_store_name: ::std::option::Option<::std::string::String>,
pub(crate) xks_proxy_authentication_credential: ::std::option::Option<kms::types::XksProxyAuthenticationCredentialType>,
pub(crate) xks_proxy_connectivity: ::std::option::Option<kms::types::XksProxyConnectivityType>,
pub(crate) xks_proxy_uri_endpoint: ::std::option::Option<::std::string::String>,
pub(crate) xks_proxy_uri_path: ::std::option::Option<::std::string::String>,
pub(crate) xks_proxy_vpc_endpoint_service_name: ::std::option::Option<::std::string::String>,
}
impl UpdateCustomKeyStoreRequestBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn cloud_hsm_cluster_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.cloud_hsm_cluster_id = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_cloud_hsm_cluster_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.cloud_hsm_cluster_id = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_cloud_hsm_cluster_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.cloud_hsm_cluster_id
}
#[allow(missing_docs)] // documentation missing in model
pub fn custom_key_store_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.custom_key_store_id = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_custom_key_store_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.custom_key_store_id = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_custom_key_store_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.custom_key_store_id
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_store_password(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.key_store_password = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_key_store_password(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.key_store_password = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_key_store_password(&self) -> &::std::option::Option<::std::string::String> {
    &self.key_store_password
}
#[allow(missing_docs)] // documentation missing in model
pub fn new_custom_key_store_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.new_custom_key_store_name = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_new_custom_key_store_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.new_custom_key_store_name = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_new_custom_key_store_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.new_custom_key_store_name
}
#[allow(missing_docs)] // documentation missing in model
pub fn xks_proxy_authentication_credential(mut self, input: impl ::std::convert::Into<kms::types::XksProxyAuthenticationCredentialType>) -> Self {
    self.xks_proxy_authentication_credential = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_xks_proxy_authentication_credential(mut self, input: ::std::option::Option<kms::types::XksProxyAuthenticationCredentialType>) -> Self {
    self.xks_proxy_authentication_credential = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_xks_proxy_authentication_credential(&self) -> &::std::option::Option<kms::types::XksProxyAuthenticationCredentialType> {
    &self.xks_proxy_authentication_credential
}
#[allow(missing_docs)] // documentation missing in model
pub fn xks_proxy_connectivity(mut self, input: impl ::std::convert::Into<kms::types::XksProxyConnectivityType>) -> Self {
    self.xks_proxy_connectivity = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_xks_proxy_connectivity(mut self, input: ::std::option::Option<kms::types::XksProxyConnectivityType>) -> Self {
    self.xks_proxy_connectivity = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_xks_proxy_connectivity(&self) -> &::std::option::Option<kms::types::XksProxyConnectivityType> {
    &self.xks_proxy_connectivity
}
#[allow(missing_docs)] // documentation missing in model
pub fn xks_proxy_uri_endpoint(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.xks_proxy_uri_endpoint = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_xks_proxy_uri_endpoint(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.xks_proxy_uri_endpoint = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_xks_proxy_uri_endpoint(&self) -> &::std::option::Option<::std::string::String> {
    &self.xks_proxy_uri_endpoint
}
#[allow(missing_docs)] // documentation missing in model
pub fn xks_proxy_uri_path(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.xks_proxy_uri_path = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_xks_proxy_uri_path(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.xks_proxy_uri_path = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_xks_proxy_uri_path(&self) -> &::std::option::Option<::std::string::String> {
    &self.xks_proxy_uri_path
}
#[allow(missing_docs)] // documentation missing in model
pub fn xks_proxy_vpc_endpoint_service_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.xks_proxy_vpc_endpoint_service_name = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_xks_proxy_vpc_endpoint_service_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.xks_proxy_vpc_endpoint_service_name = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_xks_proxy_vpc_endpoint_service_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.xks_proxy_vpc_endpoint_service_name
}
    /// Consumes the builder and constructs a [`UpdateCustomKeyStoreRequest`](crate::operation::operation::UpdateCustomKeyStoreRequest).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_custom_key_store::UpdateCustomKeyStoreRequest,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::update_custom_key_store::UpdateCustomKeyStoreRequest {
            cloud_hsm_cluster_id: self.cloud_hsm_cluster_id,
custom_key_store_id: self.custom_key_store_id,
key_store_password: self.key_store_password,
new_custom_key_store_name: self.new_custom_key_store_name,
xks_proxy_authentication_credential: self.xks_proxy_authentication_credential,
xks_proxy_connectivity: self.xks_proxy_connectivity,
xks_proxy_uri_endpoint: self.xks_proxy_uri_endpoint,
xks_proxy_uri_path: self.xks_proxy_uri_path,
xks_proxy_vpc_endpoint_service_name: self.xks_proxy_vpc_endpoint_service_name,
        })
    }
}
