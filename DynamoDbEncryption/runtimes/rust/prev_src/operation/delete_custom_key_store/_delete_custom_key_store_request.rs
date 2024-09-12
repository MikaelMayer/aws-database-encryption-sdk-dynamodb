// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteCustomKeyStoreRequest {
    #[allow(missing_docs)] // documentation missing in model
pub custom_key_store_id: ::std::option::Option<::std::string::String>,
}
impl DeleteCustomKeyStoreRequest {
    #[allow(missing_docs)] // documentation missing in model
pub fn custom_key_store_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.custom_key_store_id
}
}
impl DeleteCustomKeyStoreRequest {
    /// Creates a new builder-style object to manufacture [`DeleteCustomKeyStoreRequest`](crate::operation::delete_custom_key_store::builders::DeleteCustomKeyStoreRequest).
    pub fn builder() -> crate::operation::delete_custom_key_store::builders::DeleteCustomKeyStoreRequestBuilder {
        crate::operation::delete_custom_key_store::builders::DeleteCustomKeyStoreRequestBuilder::default()
    }
}

/// A builder for [`DeleteCustomKeyStoreRequest`](crate::operation::operation::DeleteCustomKeyStoreRequest).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteCustomKeyStoreRequestBuilder {
    pub(crate) custom_key_store_id: ::std::option::Option<::std::string::String>,
}
impl DeleteCustomKeyStoreRequestBuilder {
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
    /// Consumes the builder and constructs a [`DeleteCustomKeyStoreRequest`](crate::operation::operation::DeleteCustomKeyStoreRequest).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_custom_key_store::DeleteCustomKeyStoreRequest,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::delete_custom_key_store::DeleteCustomKeyStoreRequest {
            custom_key_store_id: self.custom_key_store_id,
        })
    }
}
