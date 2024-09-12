// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `Sign`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct Sign;
impl Sign {
    /// Creates a new `Sign`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::sign::SignRequest,
    ) -> ::std::result::Result<
        crate::operation::sign::SignResponse,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::sign::_sign_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).Sign(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::sign::_sign_output::from_dafny(
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

pub use crate::operation::sign::_sign_response::SignResponse;

pub use crate::operation::sign::_sign_request::SignRequest;

pub(crate) mod _sign_response;

pub(crate) mod _sign_request;

/// Builders
pub mod builders;
