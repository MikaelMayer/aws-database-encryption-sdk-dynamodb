// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `KdfCounterMode`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct KdfCounterMode;
impl KdfCounterMode {
    /// Creates a new `KdfCounterMode`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::primitives::client::Client,
        input: crate::primitives::operation::kdf_counter_mode::KdfCtrInput,
    ) -> ::std::result::Result<
        ::aws_smithy_types::Blob,
        crate::primitives::types::error::Error,
    > {
                let inner_input = crate::primitives::conversions::kdf_counter_mode::_kdf_counter_mode_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).KdfCounterMode(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::Wrappers::Result::Success { .. }
        ) {
            Ok(
                crate::ddb::standard_library_conversions::blob_from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::primitives::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::primitives::operation::kdf_counter_mode::_kdf_ctr_output::KdfCtrOutput;

pub use crate::primitives::operation::kdf_counter_mode::_kdf_ctr_input::KdfCtrInput;

pub(crate) mod _kdf_ctr_output;

pub(crate) mod _kdf_ctr_input;

/// Builders
pub mod builders;
