// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `GetParametersForImport`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetParametersForImport;
impl GetParametersForImport {
    /// Creates a new `GetParametersForImport`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::get_parameters_for_import::GetParametersForImportRequest,
    ) -> ::std::result::Result<
        crate::operation::get_parameters_for_import::GetParametersForImportResponse,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::get_parameters_for_import::_get_parameters_for_import_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).GetParametersForImport(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::get_parameters_for_import::_get_parameters_for_import_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::operation::get_parameters_for_import::_get_parameters_for_import_response::GetParametersForImportResponse;

pub use crate::operation::get_parameters_for_import::_get_parameters_for_import_request::GetParametersForImportRequest;

pub(crate) mod _get_parameters_for_import_response;

pub(crate) mod _get_parameters_for_import_request;

/// Builders
pub mod builders;
