// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateCustomKeyStoreRequest {
    #[allow(missing_docs)] // documentation missing in model
pub cloud_hsm_cluster_id: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub custom_key_store_name: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub custom_key_store_type: ::std::option::Option<kms::types::CustomKeyStoreType>,
#[allow(missing_docs)] // documentation missing in model
pub key_store_password: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub trust_anchor_certificate: ::std::option::Option<::std::string::String>,
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
impl CreateCustomKeyStoreRequest {
    #[allow(missing_docs)] // documentation missing in model
pub fn cloud_hsm_cluster_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.cloud_hsm_cluster_id
}
#[allow(missing_docs)] // documentation missing in model
pub fn custom_key_store_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.custom_key_store_name
}
#[allow(missing_docs)] // documentation missing in model
pub fn custom_key_store_type(&self) -> &::std::option::Option<kms::types::CustomKeyStoreType> {
    &self.custom_key_store_type
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_store_password(&self) -> &::std::option::Option<::std::string::String> {
    &self.key_store_password
}
#[allow(missing_docs)] // documentation missing in model
pub fn trust_anchor_certificate(&self) -> &::std::option::Option<::std::string::String> {
    &self.trust_anchor_certificate
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
impl CreateCustomKeyStoreRequest {
    /// Creates a new builder-style object to manufacture [`CreateCustomKeyStoreRequest`](crate::operation::create_custom_key_store::builders::CreateCustomKeyStoreRequest).
    pub fn builder() -> crate::operation::create_custom_key_store::builders::CreateCustomKeyStoreRequestBuilder {
        crate::operation::create_custom_key_store::builders::CreateCustomKeyStoreRequestBuilder::default()
    }
}

/// A builder for [`CreateCustomKeyStoreRequest`](crate::operation::operation::CreateCustomKeyStoreRequest).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateCustomKeyStoreRequestBuilder {
    pub(crate) cloud_hsm_cluster_id: ::std::option::Option<::std::string::String>,
pub(crate) custom_key_store_name: ::std::option::Option<::std::string::String>,
pub(crate) custom_key_store_type: ::std::option::Option<kms::types::CustomKeyStoreType>,
pub(crate) key_store_password: ::std::option::Option<::std::string::String>,
pub(crate) trust_anchor_certificate: ::std::option::Option<::std::string::String>,
pub(crate) xks_proxy_authentication_credential: ::std::option::Option<kms::types::XksProxyAuthenticationCredentialType>,
pub(crate) xks_proxy_connectivity: ::std::option::Option<kms::types::XksProxyConnectivityType>,
pub(crate) xks_proxy_uri_endpoint: ::std::option::Option<::std::string::String>,
pub(crate) xks_proxy_uri_path: ::std::option::Option<::std::string::String>,
pub(crate) xks_proxy_vpc_endpoint_service_name: ::std::option::Option<::std::string::String>,
}
impl CreateCustomKeyStoreRequestBuilder {
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
pub fn custom_key_store_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.custom_key_store_name = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_custom_key_store_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.custom_key_store_name = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_custom_key_store_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.custom_key_store_name
}
#[allow(missing_docs)] // documentation missing in model
pub fn custom_key_store_type(mut self, input: impl ::std::convert::Into<kms::types::CustomKeyStoreType>) -> Self {
    self.custom_key_store_type = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_custom_key_store_type(mut self, input: ::std::option::Option<kms::types::CustomKeyStoreType>) -> Self {
    self.custom_key_store_type = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_custom_key_store_type(&self) -> &::std::option::Option<kms::types::CustomKeyStoreType> {
    &self.custom_key_store_type
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
pub fn trust_anchor_certificate(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.trust_anchor_certificate = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_trust_anchor_certificate(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.trust_anchor_certificate = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_trust_anchor_certificate(&self) -> &::std::option::Option<::std::string::String> {
    &self.trust_anchor_certificate
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
    /// Consumes the builder and constructs a [`CreateCustomKeyStoreRequest`](crate::operation::operation::CreateCustomKeyStoreRequest).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_custom_key_store::CreateCustomKeyStoreRequest,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::create_custom_key_store::CreateCustomKeyStoreRequest {
            cloud_hsm_cluster_id: self.cloud_hsm_cluster_id,
custom_key_store_name: self.custom_key_store_name,
custom_key_store_type: self.custom_key_store_type,
key_store_password: self.key_store_password,
trust_anchor_certificate: self.trust_anchor_certificate,
xks_proxy_authentication_credential: self.xks_proxy_authentication_credential,
xks_proxy_connectivity: self.xks_proxy_connectivity,
xks_proxy_uri_endpoint: self.xks_proxy_uri_endpoint,
xks_proxy_uri_path: self.xks_proxy_uri_path,
xks_proxy_vpc_endpoint_service_name: self.xks_proxy_vpc_endpoint_service_name,
        })
    }
}
