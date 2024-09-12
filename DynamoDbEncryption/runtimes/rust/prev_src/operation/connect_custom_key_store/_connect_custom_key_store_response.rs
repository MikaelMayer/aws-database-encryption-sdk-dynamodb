// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ConnectCustomKeyStoreResponse {

}
impl ConnectCustomKeyStoreResponse {

}
impl ConnectCustomKeyStoreResponse {
    /// Creates a new builder-style object to manufacture [`ConnectCustomKeyStoreResponse`](crate::operation::connect_custom_key_store::builders::ConnectCustomKeyStoreResponse).
    pub fn builder() -> crate::operation::connect_custom_key_store::builders::ConnectCustomKeyStoreResponseBuilder {
        crate::operation::connect_custom_key_store::builders::ConnectCustomKeyStoreResponseBuilder::default()
    }
}

/// A builder for [`ConnectCustomKeyStoreResponse`](crate::operation::operation::ConnectCustomKeyStoreResponse).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ConnectCustomKeyStoreResponseBuilder {

}
impl ConnectCustomKeyStoreResponseBuilder {

    /// Consumes the builder and constructs a [`ConnectCustomKeyStoreResponse`](crate::operation::operation::ConnectCustomKeyStoreResponse).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::connect_custom_key_store::ConnectCustomKeyStoreResponse,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::connect_custom_key_store::ConnectCustomKeyStoreResponse {

        })
    }
}
