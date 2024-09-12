// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `GenerateMac`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GenerateMac;
impl GenerateMac {
    /// Creates a new `GenerateMac`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::generate_mac::GenerateMacRequest,
    ) -> ::std::result::Result<
        crate::operation::generate_mac::GenerateMacResponse,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::generate_mac::_generate_mac_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).GenerateMac(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::generate_mac::_generate_mac_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::operation::generate_mac::_generate_mac_response::GenerateMacResponse;

pub use crate::operation::generate_mac::_generate_mac_request::GenerateMacRequest;

pub(crate) mod _generate_mac_response;

pub(crate) mod _generate_mac_request;

/// Builders
pub mod builders;
