// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `DecryptMaterials`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DecryptMaterials;
impl DecryptMaterials {
    /// Creates a new `DecryptMaterials`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        cryptographic_materials_manager: &crate::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef,
        input: crate::operation::decrypt_materials::DecryptMaterialsInput,
    ) -> ::std::result::Result<
        crate::operation::decrypt_materials::DecryptMaterialsOutput,
        crate::types::error::Error,
    > {
        cryptographic_materials_manager.inner.borrow_mut().decrypt_materials(input)
    }
}

pub use crate::operation::decrypt_materials::_decrypt_materials_output::DecryptMaterialsOutput;

pub use crate::operation::decrypt_materials::_decrypt_materials_input::DecryptMaterialsInput;

pub(crate) mod _decrypt_materials_output;

pub(crate) mod _decrypt_materials_input;

/// Builders
pub mod builders;
