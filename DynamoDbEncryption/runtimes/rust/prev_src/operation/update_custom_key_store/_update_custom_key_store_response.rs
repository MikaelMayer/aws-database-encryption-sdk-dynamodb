// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateCustomKeyStoreResponse {

}
impl UpdateCustomKeyStoreResponse {

}
impl UpdateCustomKeyStoreResponse {
    /// Creates a new builder-style object to manufacture [`UpdateCustomKeyStoreResponse`](crate::operation::update_custom_key_store::builders::UpdateCustomKeyStoreResponse).
    pub fn builder() -> crate::operation::update_custom_key_store::builders::UpdateCustomKeyStoreResponseBuilder {
        crate::operation::update_custom_key_store::builders::UpdateCustomKeyStoreResponseBuilder::default()
    }
}

/// A builder for [`UpdateCustomKeyStoreResponse`](crate::operation::operation::UpdateCustomKeyStoreResponse).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateCustomKeyStoreResponseBuilder {

}
impl UpdateCustomKeyStoreResponseBuilder {

    /// Consumes the builder and constructs a [`UpdateCustomKeyStoreResponse`](crate::operation::operation::UpdateCustomKeyStoreResponse).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_custom_key_store::UpdateCustomKeyStoreResponse,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::update_custom_key_store::UpdateCustomKeyStoreResponse {

        })
    }
}
