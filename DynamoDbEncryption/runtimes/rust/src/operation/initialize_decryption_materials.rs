// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `InitializeDecryptionMaterials`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct InitializeDecryptionMaterials;
impl InitializeDecryptionMaterials {
    /// Creates a new `InitializeDecryptionMaterials`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::initialize_decryption_materials::InitializeDecryptionMaterialsInput,
    ) -> ::std::result::Result<
        crate::operation::initialize_decryption_materials::DecryptionMaterials,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::initialize_decryption_materials::_initialize_decryption_materials_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).InitializeDecryptionMaterials(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::initialize_decryption_materials::_initialize_decryption_materials_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::operation::initialize_decryption_materials::_decryption_materials::DecryptionMaterials;

pub use crate::operation::initialize_decryption_materials::_initialize_decryption_materials_input::InitializeDecryptionMaterialsInput;

pub(crate) mod _decryption_materials;

pub(crate) mod _initialize_decryption_materials_input;

/// Builders
pub mod builders;