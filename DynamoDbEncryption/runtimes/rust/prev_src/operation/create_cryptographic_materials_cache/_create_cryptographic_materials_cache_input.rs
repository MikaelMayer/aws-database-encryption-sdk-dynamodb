// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateCryptographicMaterialsCacheInput {
    #[allow(missing_docs)] // documentation missing in model
pub cache: ::std::option::Option<material_providers::types::CacheType>,
}
impl CreateCryptographicMaterialsCacheInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn cache(&self) -> &::std::option::Option<material_providers::types::CacheType> {
    &self.cache
}
}
impl CreateCryptographicMaterialsCacheInput {
    /// Creates a new builder-style object to manufacture [`CreateCryptographicMaterialsCacheInput`](crate::operation::create_cryptographic_materials_cache::builders::CreateCryptographicMaterialsCacheInput).
    pub fn builder() -> crate::operation::create_cryptographic_materials_cache::builders::CreateCryptographicMaterialsCacheInputBuilder {
        crate::operation::create_cryptographic_materials_cache::builders::CreateCryptographicMaterialsCacheInputBuilder::default()
    }
}

/// A builder for [`CreateCryptographicMaterialsCacheInput`](crate::operation::operation::CreateCryptographicMaterialsCacheInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateCryptographicMaterialsCacheInputBuilder {
    pub(crate) cache: ::std::option::Option<material_providers::types::CacheType>,
}
impl CreateCryptographicMaterialsCacheInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn cache(mut self, input: impl ::std::convert::Into<material_providers::types::CacheType>) -> Self {
    self.cache = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_cache(mut self, input: ::std::option::Option<material_providers::types::CacheType>) -> Self {
    self.cache = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_cache(&self) -> &::std::option::Option<material_providers::types::CacheType> {
    &self.cache
}
    /// Consumes the builder and constructs a [`CreateCryptographicMaterialsCacheInput`](crate::operation::operation::CreateCryptographicMaterialsCacheInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_cryptographic_materials_cache::CreateCryptographicMaterialsCacheInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::create_cryptographic_materials_cache::CreateCryptographicMaterialsCacheInput {
            cache: self.cache,
        })
    }
}