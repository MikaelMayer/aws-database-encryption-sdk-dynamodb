// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `HkdfExpand`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct HkdfExpand;
impl HkdfExpand {
    /// Creates a new `HkdfExpand`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::primitives::client::Client,
        input: crate::primitives::operation::hkdf_expand::HkdfExpandInput,
    ) -> ::std::result::Result<
        ::aws_smithy_types::Blob,
        crate::primitives::types::error::Error,
    > {
                let inner_input = crate::primitives::conversions::hkdf_expand::_hkdf_expand_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).HkdfExpand(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::Wrappers::Result::Success { .. }
        ) {
            Ok(
                crate::primitives::conversions::hkdf_expand::_hkdf_expand_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::primitives::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::primitives::operation::hkdf_expand::_hkdf_expand_output::HkdfExpandOutput;

pub use crate::primitives::operation::hkdf_expand::_hkdf_expand_input::HkdfExpandInput;

pub(crate) mod _hkdf_expand_output;

pub(crate) mod _hkdf_expand_input;

/// Builders
pub mod builders;
