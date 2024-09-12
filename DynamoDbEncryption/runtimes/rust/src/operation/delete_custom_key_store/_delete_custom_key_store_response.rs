// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteCustomKeyStoreResponse {

}
impl DeleteCustomKeyStoreResponse {

}
impl DeleteCustomKeyStoreResponse {
    /// Creates a new builder-style object to manufacture [`DeleteCustomKeyStoreResponse`](crate::operation::delete_custom_key_store::builders::DeleteCustomKeyStoreResponse).
    pub fn builder() -> crate::operation::delete_custom_key_store::builders::DeleteCustomKeyStoreResponseBuilder {
        crate::operation::delete_custom_key_store::builders::DeleteCustomKeyStoreResponseBuilder::default()
    }
}

/// A builder for [`DeleteCustomKeyStoreResponse`](crate::operation::operation::DeleteCustomKeyStoreResponse).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteCustomKeyStoreResponseBuilder {

}
impl DeleteCustomKeyStoreResponseBuilder {

    /// Consumes the builder and constructs a [`DeleteCustomKeyStoreResponse`](crate::operation::operation::DeleteCustomKeyStoreResponse).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_custom_key_store::DeleteCustomKeyStoreResponse,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::delete_custom_key_store::DeleteCustomKeyStoreResponse {

        })
    }
}
