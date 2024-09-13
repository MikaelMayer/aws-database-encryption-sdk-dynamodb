// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`CreateDynamoDbEncryptionBranchKeyIdSupplier`](crate::operation::create_dynamo_db_encryption_branch_key_id_supplier::builders::CreateDynamoDbEncryptionBranchKeyIdSupplierFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`ddb_key_branch_key_id_supplier(impl Into<Option<dynamo_db::types::dynamo_db_key_branch_key_id_supplier::DynamoDbKeyBranchKeyIdSupplierRef>>)`](crate::operation::create_dynamo_db_encryption_branch_key_id_supplier::builders::CreateDynamoDbEncryptionBranchKeyIdSupplierFluentBuilder::ddb_key_branch_key_id_supplier) / [`set_ddb_key_branch_key_id_supplier(Option<dynamo_db::types::dynamo_db_key_branch_key_id_supplier::DynamoDbKeyBranchKeyIdSupplierRef>)`](crate::operation::create_dynamo_db_encryption_branch_key_id_supplier::builders::CreateDynamoDbEncryptionBranchKeyIdSupplierFluentBuilder::set_ddb_key_branch_key_id_supplier): (undocumented)<br>
    /// - On success, responds with [`CreateDynamoDbEncryptionBranchKeyIdSupplierOutput`](crate::operation::create_dynamo_db_encryption_branch_key_id_supplier::CreateDynamoDbEncryptionBranchKeyIdSupplierOutput) with field(s):
    ///   - [`branch_key_id_supplier(Option<material_providers::types::branch_key_id_supplier::BranchKeyIdSupplierRef>)`](crate::operation::create_dynamo_db_encryption_branch_key_id_supplier::CreateDynamoDbEncryptionBranchKeyIdSupplierOutput::branch_key_id_supplier): (undocumented)
    /// - On failure, responds with [`SdkError<CreateDynamoDbEncryptionBranchKeyIdSupplierError>`](crate::operation::create_dynamo_db_encryption_branch_key_id_supplier::CreateDynamoDbEncryptionBranchKeyIdSupplierError)
    pub fn create_dynamo_db_encryption_branch_key_id_supplier(&self) -> crate::operation::create_dynamo_db_encryption_branch_key_id_supplier::builders::CreateDynamoDbEncryptionBranchKeyIdSupplierFluentBuilder {
        crate::operation::create_dynamo_db_encryption_branch_key_id_supplier::builders::CreateDynamoDbEncryptionBranchKeyIdSupplierFluentBuilder::new(self.clone())
    }
}