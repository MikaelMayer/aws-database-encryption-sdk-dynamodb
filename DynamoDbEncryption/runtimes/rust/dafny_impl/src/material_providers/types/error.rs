// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[derive(::std::clone::Clone, ::std::fmt::Debug, ::std::cmp::PartialEq)]
pub enum Error {
    KeyStoreException {
    message: ::std::string::String,
},

DependencyTimeoutException {
    message: ::std::string::String,
},

InvalidMarkerException {
    message: ::std::string::String,
},

KmsInternalException {
    message: ::std::string::String,
},

InvalidArnException {
    message: ::std::string::String,
},

InvalidGrantIdException {
    message: ::std::string::String,
},

KmsInvalidStateException {
    message: ::std::string::String,
},

NotFoundException {
    message: ::std::string::String,
},

InternalServerError {
    message: ::std::string::String,
},

ResourceNotFoundException {
    message: ::std::string::String,
},

LimitExceededException {
    message: ::std::string::String,
},

ContinuousBackupsUnavailableException {
    message: ::std::string::String,
},

InvalidEndpointException {
    message: ::std::string::String,
},

TableNotFoundException {
    message: ::std::string::String,
},

AlreadyExistsException {
    message: ::std::string::String,
},

InvalidAliasNameException {
    message: ::std::string::String,
},

LimitExceededException {
    message: ::std::string::String,
},

ExpiredImportTokenException {
    message: ::std::string::String,
},

IncorrectKeyMaterialException {
    message: ::std::string::String,
},

InvalidCiphertextException {
    message: ::std::string::String,
},

InvalidImportTokenException {
    message: ::std::string::String,
},

UnsupportedOperationException {
    message: ::std::string::String,
},

DisabledException {
    message: ::std::string::String,
},

ExportNotFoundException {
    message: ::std::string::String,
},

DryRunOperationException {
    message: ::std::string::String,
},

InvalidGrantTokenException {
    message: ::std::string::String,
},

InvalidKeyUsageException {
    message: ::std::string::String,
},

KeyUnavailableException {
    message: ::std::string::String,
},

KmsInvalidMacException {
    message: ::std::string::String,
},

KmsInvalidSignatureException {
    message: ::std::string::String,
},

InvalidEncryptionMaterials {
    message: ::std::string::String,
},

InvalidAlgorithmSuiteInfo {
    message: ::std::string::String,
},

ResourceInUseException {
    message: ::std::string::String,
},

CloudHsmClusterInvalidConfigurationException {
    message: ::std::string::String,
},

CustomKeyStoreInvalidStateException {
    message: ::std::string::String,
},

CustomKeyStoreNotFoundException {
    message: ::std::string::String,
},

MalformedPolicyDocumentException {
    message: ::std::string::String,
},

TagException {
    message: ::std::string::String,
},

XksKeyAlreadyInUseException {
    message: ::std::string::String,
},

XksKeyInvalidConfigurationException {
    message: ::std::string::String,
},

XksKeyNotFoundException {
    message: ::std::string::String,
},

InvalidDecryptionMaterials {
    message: ::std::string::String,
},

ConditionalCheckFailedException {
    message: ::std::string::String,
},

ItemCollectionSizeLimitExceededException {
    message: ::std::string::String,
},

ProvisionedThroughputExceededException {
    message: ::std::string::String,
},

RequestLimitExceeded {
    message: ::std::string::String,
},

TransactionConflictException {
    message: ::std::string::String,
},

GlobalTableNotFoundException {
    message: ::std::string::String,
},

IndexNotFoundException {
    message: ::std::string::String,
},

ReplicaNotFoundException {
    message: ::std::string::String,
},

InvalidDecryptionMaterialsTransition {
    message: ::std::string::String,
},

IdempotentParameterMismatchException {
    message: ::std::string::String,
},

TransactionCanceledException {
    message: ::std::string::String,
},

TransactionInProgressException {
    message: ::std::string::String,
},

IncorrectKeyException {
    message: ::std::string::String,
},

ImportNotFoundException {
    message: ::std::string::String,
},

ConflictException {
    message: ::std::string::String,
},

ExportConflictException {
    message: ::std::string::String,
},

InvalidExportTimeException {
    message: ::std::string::String,
},

PointInTimeRecoveryUnavailableException {
    message: ::std::string::String,
},

BackupInUseException {
    message: ::std::string::String,
},

TableInUseException {
    message: ::std::string::String,
},

BackupNotFoundException {
    message: ::std::string::String,
},

TableAlreadyExistsException {
    message: ::std::string::String,
},

InvalidRestoreTimeException {
    message: ::std::string::String,
},

GlobalTableAlreadyExistsException {
    message: ::std::string::String,
},

CustomKeyStoreHasCmKsException {
    message: ::std::string::String,
},

CloudHsmClusterNotActiveException {
    message: ::std::string::String,
},

CloudHsmClusterNotFoundException {
    message: ::std::string::String,
},

CloudHsmClusterNotRelatedException {
    message: ::std::string::String,
},

CustomKeyStoreNameInUseException {
    message: ::std::string::String,
},

XksProxyIncorrectAuthenticationCredentialException {
    message: ::std::string::String,
},

XksProxyInvalidConfigurationException {
    message: ::std::string::String,
},

XksProxyInvalidResponseException {
    message: ::std::string::String,
},

XksProxyUriEndpointInUseException {
    message: ::std::string::String,
},

XksProxyUriInUseException {
    message: ::std::string::String,
},

XksProxyUriUnreachableException {
    message: ::std::string::String,
},

XksProxyVpcEndpointServiceInUseException {
    message: ::std::string::String,
},

XksProxyVpcEndpointServiceInvalidConfigurationException {
    message: ::std::string::String,
},

XksProxyVpcEndpointServiceNotFoundException {
    message: ::std::string::String,
},

InvalidAlgorithmSuiteInfoOnEncrypt {
    message: ::std::string::String,
},

ReplicaAlreadyExistsException {
    message: ::std::string::String,
},

CloudHsmClusterInUseException {
    message: ::std::string::String,
},

IncorrectTrustAnchorException {
    message: ::std::string::String,
},

InvalidAlgorithmSuiteInfoOnDecrypt {
    message: ::std::string::String,
},

InvalidEncryptionMaterialsTransition {
    message: ::std::string::String,
},

DuplicateItemException {
    message: ::std::string::String,
},

ImportConflictException {
    message: ::std::string::String,
},
    CollectionOfErrors {
        list: ::std::vec::Vec<Error>,
        message: ::std::string::String,
    },
    Opaque {
        obj: ::dafny_runtime::Object<dyn ::std::any::Any>,
    },
}

impl ::std::cmp::Eq for Error {}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ::std::error::Error for Error {}
