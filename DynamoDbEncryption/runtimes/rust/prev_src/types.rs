// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Types for the `DynamoDbTablesEncryptionConfig`
pub mod dynamo_db_tables_encryption_config;

pub mod builders;

pub mod dynamo_db_key_branch_key_id_supplier;
pub use dynamo_db_key_branch_key_id_supplier::DynamoDbKeyBranchKeyIdSupplier;
pub mod legacy_dynamo_db_encryptor;
pub use legacy_dynamo_db_encryptor::LegacyDynamoDbEncryptor;
pub mod branch_key_id_supplier;
pub use branch_key_id_supplier::BranchKeyIdSupplier;
pub mod client_supplier;
pub use client_supplier::ClientSupplier;
pub mod cryptographic_materials_cache;
pub use cryptographic_materials_cache::CryptographicMaterialsCache;
pub mod cryptographic_materials_manager;
pub use cryptographic_materials_manager::CryptographicMaterialsManager;
pub mod keyring;
pub use keyring::Keyring;



pub mod error;



