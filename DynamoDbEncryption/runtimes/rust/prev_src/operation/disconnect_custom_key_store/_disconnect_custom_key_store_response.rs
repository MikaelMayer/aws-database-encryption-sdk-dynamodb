// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DisconnectCustomKeyStoreResponse {

}
impl DisconnectCustomKeyStoreResponse {

}
impl DisconnectCustomKeyStoreResponse {
    /// Creates a new builder-style object to manufacture [`DisconnectCustomKeyStoreResponse`](crate::operation::disconnect_custom_key_store::builders::DisconnectCustomKeyStoreResponse).
    pub fn builder() -> crate::operation::disconnect_custom_key_store::builders::DisconnectCustomKeyStoreResponseBuilder {
        crate::operation::disconnect_custom_key_store::builders::DisconnectCustomKeyStoreResponseBuilder::default()
    }
}

/// A builder for [`DisconnectCustomKeyStoreResponse`](crate::operation::operation::DisconnectCustomKeyStoreResponse).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DisconnectCustomKeyStoreResponseBuilder {

}
impl DisconnectCustomKeyStoreResponseBuilder {

    /// Consumes the builder and constructs a [`DisconnectCustomKeyStoreResponse`](crate::operation::operation::DisconnectCustomKeyStoreResponse).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::disconnect_custom_key_store::DisconnectCustomKeyStoreResponse,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::disconnect_custom_key_store::DisconnectCustomKeyStoreResponse {

        })
    }
}
