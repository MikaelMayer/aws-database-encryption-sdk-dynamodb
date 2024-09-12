// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `GenerateRandomBytes`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GenerateRandomBytes;
impl GenerateRandomBytes {
    /// Creates a new `GenerateRandomBytes`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::primitives::client::Client,
        input: crate::primitives::operation::generate_random_bytes::GenerateRandomBytesInput,
    ) -> ::std::result::Result<
        ::aws_smithy_types::Blob,
        crate::primitives::types::error::Error,
    > {
                let inner_input = crate::primitives::conversions::generate_random_bytes::_generate_random_bytes_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).GenerateRandomBytes(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::Wrappers::Result::Success { .. }
        ) {
            Ok(
                crate::primitives::conversions::generate_random_bytes::_generate_random_bytes_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::primitives::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::primitives::operation::generate_random_bytes::_generate_random_bytes_output::GenerateRandomBytesOutput;

pub use crate::primitives::operation::generate_random_bytes::_generate_random_bytes_input::GenerateRandomBytesInput;

pub(crate) mod _generate_random_bytes_output;

pub(crate) mod _generate_random_bytes_input;

/// Builders
pub mod builders;
