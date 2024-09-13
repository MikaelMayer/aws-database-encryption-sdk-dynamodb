// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `GetClient`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetClient;
impl GetClient {
    /// Creates a new `GetClient`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client_supplier: &crate::types::client_supplier::ClientSupplierRef,
        input: crate::operation::get_client::GetClientInput,
    ) -> ::std::result::Result<
        crate::operation::get_client::GetClientOutput,
        crate::types::error::Error,
    > {
        client_supplier.inner.borrow_mut().get_client(input)
    }
}

pub use crate::operation::get_client::_get_client_output::GetClientOutput;

pub use crate::operation::get_client::_get_client_input::GetClientInput;

pub(crate) mod _get_client_output;

pub(crate) mod _get_client_input;

/// Builders
pub mod builders;