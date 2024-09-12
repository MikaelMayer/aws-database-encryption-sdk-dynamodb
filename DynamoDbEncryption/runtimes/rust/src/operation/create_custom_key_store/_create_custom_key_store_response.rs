// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateCustomKeyStoreResponse {
    #[allow(missing_docs)] // documentation missing in model
pub custom_key_store_id: ::std::option::Option<::std::string::String>,
}
impl CreateCustomKeyStoreResponse {
    #[allow(missing_docs)] // documentation missing in model
pub fn custom_key_store_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.custom_key_store_id
}
}
impl CreateCustomKeyStoreResponse {
    /// Creates a new builder-style object to manufacture [`CreateCustomKeyStoreResponse`](crate::operation::create_custom_key_store::builders::CreateCustomKeyStoreResponse).
    pub fn builder() -> crate::operation::create_custom_key_store::builders::CreateCustomKeyStoreResponseBuilder {
        crate::operation::create_custom_key_store::builders::CreateCustomKeyStoreResponseBuilder::default()
    }
}

/// A builder for [`CreateCustomKeyStoreResponse`](crate::operation::operation::CreateCustomKeyStoreResponse).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateCustomKeyStoreResponseBuilder {
    pub(crate) custom_key_store_id: ::std::option::Option<::std::string::String>,
}
impl CreateCustomKeyStoreResponseBuilder {
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
    /// Consumes the builder and constructs a [`CreateCustomKeyStoreResponse`](crate::operation::operation::CreateCustomKeyStoreResponse).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_custom_key_store::CreateCustomKeyStoreResponse,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::create_custom_key_store::CreateCustomKeyStoreResponse {
            custom_key_store_id: self.custom_key_store_id,
        })
    }
}
