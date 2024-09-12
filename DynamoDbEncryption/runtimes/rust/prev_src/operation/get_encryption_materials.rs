// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `GetEncryptionMaterials`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetEncryptionMaterials;
impl GetEncryptionMaterials {
    /// Creates a new `GetEncryptionMaterials`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        cryptographic_materials_manager: &crate::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef,
        input: crate::operation::get_encryption_materials::GetEncryptionMaterialsInput,
    ) -> ::std::result::Result<
        crate::operation::get_encryption_materials::GetEncryptionMaterialsOutput,
        crate::types::error::Error,
    > {
        cryptographic_materials_manager.inner.borrow_mut().get_encryption_materials(input)
    }
}

pub use crate::operation::get_encryption_materials::_get_encryption_materials_output::GetEncryptionMaterialsOutput;

pub use crate::operation::get_encryption_materials::_get_encryption_materials_input::GetEncryptionMaterialsInput;

pub(crate) mod _get_encryption_materials_output;

pub(crate) mod _get_encryption_materials_input;

/// Builders
pub mod builders;
