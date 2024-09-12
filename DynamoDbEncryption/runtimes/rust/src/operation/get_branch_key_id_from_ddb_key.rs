// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `GetBranchKeyIdFromDdbKey`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetBranchKeyIdFromDdbKey;
impl GetBranchKeyIdFromDdbKey {
    /// Creates a new `GetBranchKeyIdFromDdbKey`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        dynamo_db_key_branch_key_id_supplier: &crate::types::dynamo_db_key_branch_key_id_supplier::DynamoDbKeyBranchKeyIdSupplierRef,
        input: crate::operation::get_branch_key_id_from_ddb_key::GetBranchKeyIdFromDdbKeyInput,
    ) -> ::std::result::Result<
        crate::operation::get_branch_key_id_from_ddb_key::GetBranchKeyIdFromDdbKeyOutput,
        crate::types::error::Error,
    > {
        dynamo_db_key_branch_key_id_supplier.inner.borrow_mut().get_branch_key_id_from_ddb_key(input)
    }
}

pub use crate::operation::get_branch_key_id_from_ddb_key::_get_branch_key_id_from_ddb_key_output::GetBranchKeyIdFromDdbKeyOutput;

pub use crate::operation::get_branch_key_id_from_ddb_key::_get_branch_key_id_from_ddb_key_input::GetBranchKeyIdFromDdbKeyInput;

pub(crate) mod _get_branch_key_id_from_ddb_key_output;

pub(crate) mod _get_branch_key_id_from_ddb_key_input;

/// Builders
pub mod builders;
