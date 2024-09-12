// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `Verify`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct Verify;
impl Verify {
    /// Creates a new `Verify`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::verify::VerifyRequest,
    ) -> ::std::result::Result<
        crate::operation::verify::VerifyResponse,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::verify::_verify_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).Verify(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::verify::_verify_output::from_dafny(
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

pub use crate::operation::verify::_verify_response::VerifyResponse;

pub use crate::operation::verify::_verify_request::VerifyRequest;

pub(crate) mod _verify_response;

pub(crate) mod _verify_request;

/// Builders
pub mod builders;
