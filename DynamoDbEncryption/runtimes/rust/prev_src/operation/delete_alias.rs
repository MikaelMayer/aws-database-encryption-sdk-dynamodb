// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `DeleteAlias`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DeleteAlias;
impl DeleteAlias {
    /// Creates a new `DeleteAlias`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::delete_alias::DeleteAliasRequest,
    ) -> ::std::result::Result<
        crate::operation::delete_alias::Unit,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::delete_alias::_delete_alias_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).DeleteAlias(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::delete_alias::_delete_alias_output::from_dafny(
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

pub use crate::operation::delete_alias::_unit::Unit;

pub use crate::operation::delete_alias::_delete_alias_request::DeleteAliasRequest;

pub(crate) mod _unit;

pub(crate) mod _delete_alias_request;

/// Builders
pub mod builders;
