// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub mod aes_decrypt;

 pub mod aes_encrypt;

 pub mod aes_kdf_counter_mode;

 pub mod batch_execute_statement;

 pub mod batch_execute_statement_input_transform;

 pub mod batch_execute_statement_output_transform;

 pub mod batch_get_item;

 pub mod batch_get_item_input_transform;

 pub mod batch_get_item_output_transform;

 pub mod batch_write_item;

 pub mod batch_write_item_input_transform;

 pub mod batch_write_item_output_transform;

 pub mod branch_key_id_supplier;

 pub mod cancel_key_deletion;

 pub mod client;

 pub mod client_supplier;

 pub mod compress_public_key;

 pub mod connect_custom_key_store;

 pub mod create_alias;

 pub mod create_aws_kms_discovery_keyring;

 pub mod create_aws_kms_discovery_multi_keyring;

 pub mod create_aws_kms_ecdh_keyring;

 pub mod create_aws_kms_hierarchical_keyring;

 pub mod create_aws_kms_keyring;

 pub mod create_aws_kms_mrk_discovery_keyring;

 pub mod create_aws_kms_mrk_discovery_multi_keyring;

 pub mod create_aws_kms_mrk_keyring;

 pub mod create_aws_kms_mrk_multi_keyring;

 pub mod create_aws_kms_multi_keyring;

 pub mod create_aws_kms_rsa_keyring;

 pub mod create_backup;

 pub mod create_cryptographic_materials_cache;

 pub mod create_custom_key_store;

 pub mod create_default_client_supplier;

 pub mod create_default_cryptographic_materials_manager;

 pub mod create_dynamo_db_encryption_branch_key_id_supplier;

 pub mod create_global_table;

 pub mod create_grant;

 pub mod create_key;

 pub mod create_key;

 pub mod create_key_store;

 pub mod create_multi_keyring;

 pub mod create_raw_aes_keyring;

 pub mod create_raw_ecdh_keyring;

 pub mod create_raw_rsa_keyring;

 pub mod create_required_encryption_context_cmm;

 pub mod create_table;

 pub mod cryptographic_materials_cache;

 pub mod cryptographic_materials_manager;

 pub mod decompress_public_key;

 pub mod decrypt;

 pub mod decrypt_item;

 pub mod decrypt_materials;

 pub mod decrypt_path_structure;

 pub mod decrypt_structure;

 pub mod decryption_materials_with_plaintext_data_key;

 pub mod delete_alias;

 pub mod delete_backup;

 pub mod delete_cache_entry;

 pub mod delete_custom_key_store;

 pub mod delete_imported_key_material;

 pub mod delete_item;

 pub mod delete_item_input_transform;

 pub mod delete_item_output_transform;

 pub mod delete_table;

 pub mod derive_shared_secret;

 pub mod derive_shared_secret;

 pub mod describe_backup;

 pub mod describe_continuous_backups;

 pub mod describe_contributor_insights;

 pub mod describe_custom_key_stores;

 pub mod describe_endpoints;

 pub mod describe_export;

 pub mod describe_global_table;

 pub mod describe_global_table_settings;

 pub mod describe_import;

 pub mod describe_key;

 pub mod describe_kinesis_streaming_destination;

 pub mod describe_limits;

 pub mod describe_table;

 pub mod describe_table_replica_auto_scaling;

 pub mod describe_time_to_live;

 pub mod digest;

 pub mod disable_key;

 pub mod disable_key_rotation;

 pub mod disable_kinesis_streaming_destination;

 pub mod disconnect_custom_key_store;

 pub mod dynamo_db_key_branch_key_id_supplier;

 pub mod ecdsa_sign;

 pub mod ecdsa_verify;

 pub mod enable_key;

 pub mod enable_key_rotation;

 pub mod enable_kinesis_streaming_destination;

 pub mod encrypt;

 pub mod encrypt_item;

 pub mod encrypt_path_structure;

 pub mod encrypt_structure;

 pub mod encryption_materials_has_plaintext_data_key;

 pub mod error;

 pub mod execute_statement;

 pub mod execute_statement_input_transform;

 pub mod execute_statement_output_transform;

 pub mod execute_transaction;

 pub mod execute_transaction_input_transform;

 pub mod execute_transaction_output_transform;

 pub mod export_table_to_point_in_time;

 pub mod generate_data_key;

 pub mod generate_data_key_pair;

 pub mod generate_data_key_pair_without_plaintext;

 pub mod generate_data_key_without_plaintext;

 pub mod generate_ecc_key_pair;

 pub mod generate_ecdsa_signature_key;

 pub mod generate_mac;

 pub mod generate_random;

 pub mod generate_random_bytes;

 pub mod generate_rsa_key_pair;

 pub mod get_active_branch_key;

 pub mod get_algorithm_suite_info;

 pub mod get_beacon_key;

 pub mod get_branch_key_id;

 pub mod get_branch_key_id_from_ddb_key;

 pub mod get_branch_key_version;

 pub mod get_cache_entry;

 pub mod get_client;

 pub mod get_encrypted_data_key_description;

 pub mod get_encryption_materials;

 pub mod get_item;

 pub mod get_item_input_transform;

 pub mod get_item_output_transform;

 pub mod get_key_policy;

 pub mod get_key_rotation_status;

 pub mod get_key_store_info;

 pub mod get_parameters_for_import;

 pub mod get_public_key;

 pub mod get_public_key_from_private_key;

 pub mod get_rsa_key_modulus_length;

 pub mod h_mac;

 pub mod hkdf;

 pub mod hkdf_expand;

 pub mod hkdf_extract;

 pub mod import_key_material;

 pub mod import_table;

 pub mod initialize_decryption_materials;

 pub mod initialize_encryption_materials;

 pub mod kdf_counter_mode;

 pub mod keyring;

 pub mod legacy_dynamo_db_encryptor;

 pub mod list_aliases;

 pub mod list_backups;

 pub mod list_contributor_insights;

 pub mod list_exports;

 pub mod list_global_tables;

 pub mod list_grants;

 pub mod list_imports;

 pub mod list_key_policies;

 pub mod list_key_rotations;

 pub mod list_keys;

 pub mod list_resource_tags;

 pub mod list_tables;

 pub mod list_tags_of_resource;

 pub mod on_decrypt;

 pub mod on_encrypt;

 pub mod parse_public_key;

 pub mod put_cache_entry;

 pub mod put_item;

 pub mod put_item_input_transform;

 pub mod put_item_output_transform;

 pub mod put_key_policy;

 pub mod query;

 pub mod query_input_transform;

 pub mod query_output_transform;

 pub mod re_encrypt;

 pub mod replicate_key;

 pub mod resolve_attributes;

 pub mod resolve_auth_actions;

 pub mod restore_table_from_backup;

 pub mod restore_table_to_point_in_time;

 pub mod retire_grant;

 pub mod revoke_grant;

 pub mod rotate_key_on_demand;

 pub mod rsa_decrypt;

 pub mod rsa_encrypt;

 pub mod scan;

 pub mod scan_input_transform;

 pub mod scan_output_transform;

 pub mod schedule_key_deletion;

 pub mod sign;

 pub mod tag_resource;

 pub mod tag_resource;

 pub mod transact_get_items;

 pub mod transact_get_items_input_transform;

 pub mod transact_get_items_output_transform;

 pub mod transact_write_items;

 pub mod transact_write_items_input_transform;

 pub mod transact_write_items_output_transform;

 pub mod untag_resource;

 pub mod untag_resource;

 pub mod update_alias;

 pub mod update_continuous_backups;

 pub mod update_contributor_insights;

 pub mod update_custom_key_store;

 pub mod update_global_table;

 pub mod update_global_table_settings;

 pub mod update_item;

 pub mod update_item_input_transform;

 pub mod update_item_output_transform;

 pub mod update_key_description;

 pub mod update_primary_region;

 pub mod update_table;

 pub mod update_table_replica_auto_scaling;

 pub mod update_time_to_live;

 pub mod update_usage_metadata;

 pub mod valid_algorithm_suite_info;

 pub mod valid_decryption_materials_transition;

 pub mod valid_encryption_materials_transition;

 pub mod validate_commitment_policy_on_decrypt;

 pub mod validate_commitment_policy_on_encrypt;

 pub mod validate_public_key;

 pub mod verify;

 pub mod verify_mac;

 pub mod version_key;

pub mod dynamo_db_tables_encryption_config;
