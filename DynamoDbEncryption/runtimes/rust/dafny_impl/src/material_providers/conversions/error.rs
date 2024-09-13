// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Wraps up an arbitrary Rust Error value as a Dafny Error
pub fn to_opaque_error<E: 'static>(value: E) ->
    ::std::rc::Rc<crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error>
{
    let error_obj: ::dafny_runtime::Object<dyn::std::any::Any> = ::dafny_runtime::Object(Some(
        ::std::rc::Rc::new(::std::cell::UnsafeCell::new(value)),
    ));
    ::std::rc::Rc::new(
        crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::Opaque {
            obj: error_obj,
        },
    )
}

/// Wraps up an arbitrary Rust Error value as a Dafny Result<T, Error>.Failure
pub fn to_opaque_error_result<T: ::dafny_runtime::DafnyType, E: 'static>(value: E) ->
    ::std::rc::Rc<
        crate::Wrappers::Result<
            T,
            ::std::rc::Rc<crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error>
        >
    >
{
    ::std::rc::Rc::new(crate::Wrappers::Result::Failure {
        error: to_opaque_error(value),
    })
}
pub fn to_dafny(
    value: crate::material_providers::types::error::Error,
) -> ::std::rc::Rc<crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error> {
    ::std::rc::Rc::new(match value {
        crate::material_providers::types::error::Error::KeyStoreException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::KeyStoreException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::DependencyTimeoutException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::DependencyTimeoutException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::InvalidMarkerException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::InvalidMarkerException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::KmsInternalException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::KMSInternalException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::InvalidArnException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::InvalidArnException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::InvalidGrantIdException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::InvalidGrantIdException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::KmsInvalidStateException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::KMSInvalidStateException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::NotFoundException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::NotFoundException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::InternalServerError { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::InternalServerError {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::ResourceNotFoundException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::ResourceNotFoundException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::LimitExceededException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::LimitExceededException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::ContinuousBackupsUnavailableException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::ContinuousBackupsUnavailableException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::InvalidEndpointException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::InvalidEndpointException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::TableNotFoundException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::TableNotFoundException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::AlreadyExistsException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::AlreadyExistsException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::InvalidAliasNameException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::InvalidAliasNameException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::LimitExceededException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::LimitExceededException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::ExpiredImportTokenException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::ExpiredImportTokenException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::IncorrectKeyMaterialException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::IncorrectKeyMaterialException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::InvalidCiphertextException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::InvalidCiphertextException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::InvalidImportTokenException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::InvalidImportTokenException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::UnsupportedOperationException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::UnsupportedOperationException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::DisabledException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::DisabledException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::ExportNotFoundException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::ExportNotFoundException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::DryRunOperationException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::DryRunOperationException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::InvalidGrantTokenException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::InvalidGrantTokenException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::InvalidKeyUsageException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::InvalidKeyUsageException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::KeyUnavailableException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::KeyUnavailableException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::KmsInvalidMacException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::KMSInvalidMacException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::KmsInvalidSignatureException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::KMSInvalidSignatureException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::InvalidEncryptionMaterials { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::InvalidEncryptionMaterials {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::InvalidAlgorithmSuiteInfo { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::InvalidAlgorithmSuiteInfo {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::ResourceInUseException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::ResourceInUseException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::CloudHsmClusterInvalidConfigurationException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::CloudHsmClusterInvalidConfigurationException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::CustomKeyStoreInvalidStateException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::CustomKeyStoreInvalidStateException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::CustomKeyStoreNotFoundException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::CustomKeyStoreNotFoundException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::MalformedPolicyDocumentException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::MalformedPolicyDocumentException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::TagException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::TagException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::XksKeyAlreadyInUseException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::XksKeyAlreadyInUseException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::XksKeyInvalidConfigurationException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::XksKeyInvalidConfigurationException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::XksKeyNotFoundException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::XksKeyNotFoundException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::InvalidDecryptionMaterials { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::InvalidDecryptionMaterials {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::ConditionalCheckFailedException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::ConditionalCheckFailedException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::ItemCollectionSizeLimitExceededException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::ItemCollectionSizeLimitExceededException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::ProvisionedThroughputExceededException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::ProvisionedThroughputExceededException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::RequestLimitExceeded { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::RequestLimitExceeded {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::TransactionConflictException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::TransactionConflictException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::GlobalTableNotFoundException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::GlobalTableNotFoundException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::IndexNotFoundException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::IndexNotFoundException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::ReplicaNotFoundException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::ReplicaNotFoundException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::InvalidDecryptionMaterialsTransition { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::InvalidDecryptionMaterialsTransition {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::IdempotentParameterMismatchException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::IdempotentParameterMismatchException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::TransactionCanceledException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::TransactionCanceledException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::TransactionInProgressException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::TransactionInProgressException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::IncorrectKeyException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::IncorrectKeyException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::ImportNotFoundException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::ImportNotFoundException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::ConflictException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::ConflictException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::ExportConflictException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::ExportConflictException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::InvalidExportTimeException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::InvalidExportTimeException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::PointInTimeRecoveryUnavailableException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::PointInTimeRecoveryUnavailableException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::BackupInUseException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::BackupInUseException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::TableInUseException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::TableInUseException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::BackupNotFoundException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::BackupNotFoundException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::TableAlreadyExistsException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::TableAlreadyExistsException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::InvalidRestoreTimeException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::InvalidRestoreTimeException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::GlobalTableAlreadyExistsException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::GlobalTableAlreadyExistsException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::CustomKeyStoreHasCmKsException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::CustomKeyStoreHasCMKsException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::CloudHsmClusterNotActiveException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::CloudHsmClusterNotActiveException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::CloudHsmClusterNotFoundException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::CloudHsmClusterNotFoundException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::CloudHsmClusterNotRelatedException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::CloudHsmClusterNotRelatedException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::CustomKeyStoreNameInUseException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::CustomKeyStoreNameInUseException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::XksProxyIncorrectAuthenticationCredentialException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::XksProxyIncorrectAuthenticationCredentialException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::XksProxyInvalidConfigurationException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::XksProxyInvalidConfigurationException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::XksProxyInvalidResponseException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::XksProxyInvalidResponseException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::XksProxyUriEndpointInUseException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::XksProxyUriEndpointInUseException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::XksProxyUriInUseException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::XksProxyUriInUseException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::XksProxyUriUnreachableException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::XksProxyUriUnreachableException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::XksProxyVpcEndpointServiceInUseException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::XksProxyVpcEndpointServiceInUseException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::XksProxyVpcEndpointServiceInvalidConfigurationException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::XksProxyVpcEndpointServiceInvalidConfigurationException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::XksProxyVpcEndpointServiceNotFoundException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::XksProxyVpcEndpointServiceNotFoundException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::InvalidAlgorithmSuiteInfoOnEncrypt { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::InvalidAlgorithmSuiteInfoOnEncrypt {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::ReplicaAlreadyExistsException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::ReplicaAlreadyExistsException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::CloudHsmClusterInUseException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::CloudHsmClusterInUseException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::IncorrectTrustAnchorException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::IncorrectTrustAnchorException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::InvalidAlgorithmSuiteInfoOnDecrypt { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::InvalidAlgorithmSuiteInfoOnDecrypt {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::InvalidEncryptionMaterialsTransition { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::InvalidEncryptionMaterialsTransition {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::DuplicateItemException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::DuplicateItemException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
crate::material_providers::types::error::Error::ImportConflictException { message } =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::ImportConflictException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
        crate::material_providers::types::error::Error::CollectionOfErrors { list, message } =>
            crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::CollectionOfErrors {
                message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
                list: ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&list, |e| to_dafny(e.clone()))
            },
        crate::material_providers::types::error::Error::Opaque { obj } =>
            crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::Opaque {
                obj: ::dafny_runtime::Object(obj.0)
            },
    })
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error,
    >,
) -> crate::material_providers::types::error::Error {
    match ::std::borrow::Borrow::borrow(&dafny_value) {
        crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::KeyStoreException { message } =>
    crate::material_providers::types::error::Error::KeyStoreException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::DependencyTimeoutException { message } =>
    crate::material_providers::types::error::Error::DependencyTimeoutException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::InvalidMarkerException { message } =>
    crate::material_providers::types::error::Error::InvalidMarkerException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::KMSInternalException { message } =>
    crate::material_providers::types::error::Error::KmsInternalException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::InvalidArnException { message } =>
    crate::material_providers::types::error::Error::InvalidArnException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::InvalidGrantIdException { message } =>
    crate::material_providers::types::error::Error::InvalidGrantIdException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::KMSInvalidStateException { message } =>
    crate::material_providers::types::error::Error::KmsInvalidStateException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::NotFoundException { message } =>
    crate::material_providers::types::error::Error::NotFoundException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::InternalServerError { message } =>
    crate::material_providers::types::error::Error::InternalServerError {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::ResourceNotFoundException { message } =>
    crate::material_providers::types::error::Error::ResourceNotFoundException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::LimitExceededException { message } =>
    crate::material_providers::types::error::Error::LimitExceededException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::ContinuousBackupsUnavailableException { message } =>
    crate::material_providers::types::error::Error::ContinuousBackupsUnavailableException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::InvalidEndpointException { message } =>
    crate::material_providers::types::error::Error::InvalidEndpointException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::TableNotFoundException { message } =>
    crate::material_providers::types::error::Error::TableNotFoundException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::AlreadyExistsException { message } =>
    crate::material_providers::types::error::Error::AlreadyExistsException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::InvalidAliasNameException { message } =>
    crate::material_providers::types::error::Error::InvalidAliasNameException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::LimitExceededException { message } =>
    crate::material_providers::types::error::Error::LimitExceededException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::ExpiredImportTokenException { message } =>
    crate::material_providers::types::error::Error::ExpiredImportTokenException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::IncorrectKeyMaterialException { message } =>
    crate::material_providers::types::error::Error::IncorrectKeyMaterialException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::InvalidCiphertextException { message } =>
    crate::material_providers::types::error::Error::InvalidCiphertextException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::InvalidImportTokenException { message } =>
    crate::material_providers::types::error::Error::InvalidImportTokenException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::UnsupportedOperationException { message } =>
    crate::material_providers::types::error::Error::UnsupportedOperationException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::DisabledException { message } =>
    crate::material_providers::types::error::Error::DisabledException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::ExportNotFoundException { message } =>
    crate::material_providers::types::error::Error::ExportNotFoundException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::DryRunOperationException { message } =>
    crate::material_providers::types::error::Error::DryRunOperationException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::InvalidGrantTokenException { message } =>
    crate::material_providers::types::error::Error::InvalidGrantTokenException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::InvalidKeyUsageException { message } =>
    crate::material_providers::types::error::Error::InvalidKeyUsageException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::KeyUnavailableException { message } =>
    crate::material_providers::types::error::Error::KeyUnavailableException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::KMSInvalidMacException { message } =>
    crate::material_providers::types::error::Error::KmsInvalidMacException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::KMSInvalidSignatureException { message } =>
    crate::material_providers::types::error::Error::KmsInvalidSignatureException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::InvalidEncryptionMaterials { message } =>
    crate::material_providers::types::error::Error::InvalidEncryptionMaterials {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::InvalidAlgorithmSuiteInfo { message } =>
    crate::material_providers::types::error::Error::InvalidAlgorithmSuiteInfo {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::ResourceInUseException { message } =>
    crate::material_providers::types::error::Error::ResourceInUseException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::CloudHsmClusterInvalidConfigurationException { message } =>
    crate::material_providers::types::error::Error::CloudHsmClusterInvalidConfigurationException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::CustomKeyStoreInvalidStateException { message } =>
    crate::material_providers::types::error::Error::CustomKeyStoreInvalidStateException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::CustomKeyStoreNotFoundException { message } =>
    crate::material_providers::types::error::Error::CustomKeyStoreNotFoundException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::MalformedPolicyDocumentException { message } =>
    crate::material_providers::types::error::Error::MalformedPolicyDocumentException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::TagException { message } =>
    crate::material_providers::types::error::Error::TagException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::XksKeyAlreadyInUseException { message } =>
    crate::material_providers::types::error::Error::XksKeyAlreadyInUseException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::XksKeyInvalidConfigurationException { message } =>
    crate::material_providers::types::error::Error::XksKeyInvalidConfigurationException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::XksKeyNotFoundException { message } =>
    crate::material_providers::types::error::Error::XksKeyNotFoundException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::InvalidDecryptionMaterials { message } =>
    crate::material_providers::types::error::Error::InvalidDecryptionMaterials {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::ConditionalCheckFailedException { message } =>
    crate::material_providers::types::error::Error::ConditionalCheckFailedException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::ItemCollectionSizeLimitExceededException { message } =>
    crate::material_providers::types::error::Error::ItemCollectionSizeLimitExceededException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::ProvisionedThroughputExceededException { message } =>
    crate::material_providers::types::error::Error::ProvisionedThroughputExceededException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::RequestLimitExceeded { message } =>
    crate::material_providers::types::error::Error::RequestLimitExceeded {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::TransactionConflictException { message } =>
    crate::material_providers::types::error::Error::TransactionConflictException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::GlobalTableNotFoundException { message } =>
    crate::material_providers::types::error::Error::GlobalTableNotFoundException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::IndexNotFoundException { message } =>
    crate::material_providers::types::error::Error::IndexNotFoundException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::ReplicaNotFoundException { message } =>
    crate::material_providers::types::error::Error::ReplicaNotFoundException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::InvalidDecryptionMaterialsTransition { message } =>
    crate::material_providers::types::error::Error::InvalidDecryptionMaterialsTransition {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::IdempotentParameterMismatchException { message } =>
    crate::material_providers::types::error::Error::IdempotentParameterMismatchException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::TransactionCanceledException { message } =>
    crate::material_providers::types::error::Error::TransactionCanceledException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::TransactionInProgressException { message } =>
    crate::material_providers::types::error::Error::TransactionInProgressException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::IncorrectKeyException { message } =>
    crate::material_providers::types::error::Error::IncorrectKeyException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::ImportNotFoundException { message } =>
    crate::material_providers::types::error::Error::ImportNotFoundException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::ConflictException { message } =>
    crate::material_providers::types::error::Error::ConflictException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::ExportConflictException { message } =>
    crate::material_providers::types::error::Error::ExportConflictException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::InvalidExportTimeException { message } =>
    crate::material_providers::types::error::Error::InvalidExportTimeException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::PointInTimeRecoveryUnavailableException { message } =>
    crate::material_providers::types::error::Error::PointInTimeRecoveryUnavailableException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::BackupInUseException { message } =>
    crate::material_providers::types::error::Error::BackupInUseException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::TableInUseException { message } =>
    crate::material_providers::types::error::Error::TableInUseException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::BackupNotFoundException { message } =>
    crate::material_providers::types::error::Error::BackupNotFoundException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::TableAlreadyExistsException { message } =>
    crate::material_providers::types::error::Error::TableAlreadyExistsException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::InvalidRestoreTimeException { message } =>
    crate::material_providers::types::error::Error::InvalidRestoreTimeException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::GlobalTableAlreadyExistsException { message } =>
    crate::material_providers::types::error::Error::GlobalTableAlreadyExistsException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::CustomKeyStoreHasCMKsException { message } =>
    crate::material_providers::types::error::Error::CustomKeyStoreHasCmKsException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::CloudHsmClusterNotActiveException { message } =>
    crate::material_providers::types::error::Error::CloudHsmClusterNotActiveException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::CloudHsmClusterNotFoundException { message } =>
    crate::material_providers::types::error::Error::CloudHsmClusterNotFoundException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::CloudHsmClusterNotRelatedException { message } =>
    crate::material_providers::types::error::Error::CloudHsmClusterNotRelatedException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::CustomKeyStoreNameInUseException { message } =>
    crate::material_providers::types::error::Error::CustomKeyStoreNameInUseException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::XksProxyIncorrectAuthenticationCredentialException { message } =>
    crate::material_providers::types::error::Error::XksProxyIncorrectAuthenticationCredentialException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::XksProxyInvalidConfigurationException { message } =>
    crate::material_providers::types::error::Error::XksProxyInvalidConfigurationException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::XksProxyInvalidResponseException { message } =>
    crate::material_providers::types::error::Error::XksProxyInvalidResponseException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::XksProxyUriEndpointInUseException { message } =>
    crate::material_providers::types::error::Error::XksProxyUriEndpointInUseException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::XksProxyUriInUseException { message } =>
    crate::material_providers::types::error::Error::XksProxyUriInUseException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::XksProxyUriUnreachableException { message } =>
    crate::material_providers::types::error::Error::XksProxyUriUnreachableException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::XksProxyVpcEndpointServiceInUseException { message } =>
    crate::material_providers::types::error::Error::XksProxyVpcEndpointServiceInUseException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::XksProxyVpcEndpointServiceInvalidConfigurationException { message } =>
    crate::material_providers::types::error::Error::XksProxyVpcEndpointServiceInvalidConfigurationException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::XksProxyVpcEndpointServiceNotFoundException { message } =>
    crate::material_providers::types::error::Error::XksProxyVpcEndpointServiceNotFoundException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::InvalidAlgorithmSuiteInfoOnEncrypt { message } =>
    crate::material_providers::types::error::Error::InvalidAlgorithmSuiteInfoOnEncrypt {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::ReplicaAlreadyExistsException { message } =>
    crate::material_providers::types::error::Error::ReplicaAlreadyExistsException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::CloudHsmClusterInUseException { message } =>
    crate::material_providers::types::error::Error::CloudHsmClusterInUseException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::IncorrectTrustAnchorException { message } =>
    crate::material_providers::types::error::Error::IncorrectTrustAnchorException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::InvalidAlgorithmSuiteInfoOnDecrypt { message } =>
    crate::material_providers::types::error::Error::InvalidAlgorithmSuiteInfoOnDecrypt {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::InvalidEncryptionMaterialsTransition { message } =>
    crate::material_providers::types::error::Error::InvalidEncryptionMaterialsTransition {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::DuplicateItemException { message } =>
    crate::material_providers::types::error::Error::DuplicateItemException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::ImportConflictException { message } =>
    crate::material_providers::types::error::Error::ImportConflictException {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
        crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::CollectionOfErrors { list, message } =>
            crate::material_providers::types::error::Error::CollectionOfErrors {
                message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
                list: ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(&list, |e| from_dafny(e.clone()))
            },
        crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error::Opaque { obj } =>
            crate::material_providers::types::error::Error::Opaque {
                obj: obj.clone()
            },
    }
}
