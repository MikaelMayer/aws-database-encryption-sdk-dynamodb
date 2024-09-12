// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`CreateCryptographicMaterialsCache`](crate::operation::create_cryptographic_materials_cache::builders::CreateCryptographicMaterialsCacheFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`cache(impl Into<Option<material_providers::types::CacheType>>)`](crate::operation::create_cryptographic_materials_cache::builders::CreateCryptographicMaterialsCacheFluentBuilder::cache) / [`set_cache(Option<material_providers::types::CacheType>)`](crate::operation::create_cryptographic_materials_cache::builders::CreateCryptographicMaterialsCacheFluentBuilder::set_cache): (undocumented)<br>
    /// - On success, responds with [`CreateCryptographicMaterialsCacheOutput`](crate::operation::create_cryptographic_materials_cache::CreateCryptographicMaterialsCacheOutput) with field(s):
    ///   - [`materials_cache(Option<material_providers::types::cryptographic_materials_cache::CryptographicMaterialsCacheRef>)`](crate::operation::create_cryptographic_materials_cache::CreateCryptographicMaterialsCacheOutput::materials_cache): (undocumented)
    /// - On failure, responds with [`SdkError<CreateCryptographicMaterialsCacheError>`](crate::operation::create_cryptographic_materials_cache::CreateCryptographicMaterialsCacheError)
    pub fn create_cryptographic_materials_cache(&self) -> crate::operation::create_cryptographic_materials_cache::builders::CreateCryptographicMaterialsCacheFluentBuilder {
        crate::operation::create_cryptographic_materials_cache::builders::CreateCryptographicMaterialsCacheFluentBuilder::new(self.clone())
    }
}
