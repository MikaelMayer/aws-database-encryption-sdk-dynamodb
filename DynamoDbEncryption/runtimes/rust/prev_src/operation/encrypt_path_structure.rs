// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `EncryptPathStructure`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct EncryptPathStructure;
impl EncryptPathStructure {
    /// Creates a new `EncryptPathStructure`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::encrypt_path_structure::EncryptPathStructureInput,
    ) -> ::std::result::Result<
        crate::operation::encrypt_path_structure::EncryptPathStructureOutput,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::encrypt_path_structure::_encrypt_path_structure_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).EncryptPathStructure(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::encrypt_path_structure::_encrypt_path_structure_output::from_dafny(
                    inner_result.value().clone(),
                ),
            )
        } else {
            Err(crate::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::operation::encrypt_path_structure::_encrypt_path_structure_output::EncryptPathStructureOutput;

pub use crate::operation::encrypt_path_structure::_encrypt_path_structure_input::EncryptPathStructureInput;

pub(crate) mod _encrypt_path_structure_output;

pub(crate) mod _encrypt_path_structure_input;

/// Builders
pub mod builders;
