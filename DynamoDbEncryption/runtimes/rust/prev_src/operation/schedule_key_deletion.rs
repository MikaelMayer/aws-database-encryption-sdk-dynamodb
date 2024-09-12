// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `ScheduleKeyDeletion`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ScheduleKeyDeletion;
impl ScheduleKeyDeletion {
    /// Creates a new `ScheduleKeyDeletion`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::schedule_key_deletion::ScheduleKeyDeletionRequest,
    ) -> ::std::result::Result<
        crate::operation::schedule_key_deletion::ScheduleKeyDeletionResponse,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::schedule_key_deletion::_schedule_key_deletion_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).ScheduleKeyDeletion(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::schedule_key_deletion::_schedule_key_deletion_output::from_dafny(
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

pub use crate::operation::schedule_key_deletion::_schedule_key_deletion_response::ScheduleKeyDeletionResponse;

pub use crate::operation::schedule_key_deletion::_schedule_key_deletion_request::ScheduleKeyDeletionRequest;

pub(crate) mod _schedule_key_deletion_response;

pub(crate) mod _schedule_key_deletion_request;

/// Builders
pub mod builders;
