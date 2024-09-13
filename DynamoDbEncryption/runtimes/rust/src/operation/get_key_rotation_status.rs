// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `GetKeyRotationStatus`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetKeyRotationStatus;
impl GetKeyRotationStatus {
    /// Creates a new `GetKeyRotationStatus`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::get_key_rotation_status::GetKeyRotationStatusRequest,
    ) -> ::std::result::Result<
        crate::operation::get_key_rotation_status::GetKeyRotationStatusResponse,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::get_key_rotation_status::_get_key_rotation_status_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).GetKeyRotationStatus(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::get_key_rotation_status::_get_key_rotation_status_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::operation::get_key_rotation_status::_get_key_rotation_status_response::GetKeyRotationStatusResponse;

pub use crate::operation::get_key_rotation_status::_get_key_rotation_status_request::GetKeyRotationStatusRequest;

pub(crate) mod _get_key_rotation_status_response;

pub(crate) mod _get_key_rotation_status_request;

/// Builders
pub mod builders;