// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `VerifyMac`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct VerifyMac;
impl VerifyMac {
    /// Creates a new `VerifyMac`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::verify_mac::VerifyMacRequest,
    ) -> ::std::result::Result<
        crate::operation::verify_mac::VerifyMacResponse,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::verify_mac::_verify_mac_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).VerifyMac(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::verify_mac::_verify_mac_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::operation::verify_mac::_verify_mac_response::VerifyMacResponse;

pub use crate::operation::verify_mac::_verify_mac_request::VerifyMacRequest;

pub(crate) mod _verify_mac_response;

pub(crate) mod _verify_mac_request;

/// Builders
pub mod builders;
