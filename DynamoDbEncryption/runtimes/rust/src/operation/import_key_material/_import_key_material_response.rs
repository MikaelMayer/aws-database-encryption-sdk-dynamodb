// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ImportKeyMaterialResponse {

}
impl ImportKeyMaterialResponse {

}
impl ImportKeyMaterialResponse {
    /// Creates a new builder-style object to manufacture [`ImportKeyMaterialResponse`](crate::operation::import_key_material::builders::ImportKeyMaterialResponse).
    pub fn builder() -> crate::operation::import_key_material::builders::ImportKeyMaterialResponseBuilder {
        crate::operation::import_key_material::builders::ImportKeyMaterialResponseBuilder::default()
    }
}

/// A builder for [`ImportKeyMaterialResponse`](crate::operation::operation::ImportKeyMaterialResponse).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ImportKeyMaterialResponseBuilder {

}
impl ImportKeyMaterialResponseBuilder {

    /// Consumes the builder and constructs a [`ImportKeyMaterialResponse`](crate::operation::operation::ImportKeyMaterialResponse).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::import_key_material::ImportKeyMaterialResponse,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::import_key_material::ImportKeyMaterialResponse {

        })
    }
}
