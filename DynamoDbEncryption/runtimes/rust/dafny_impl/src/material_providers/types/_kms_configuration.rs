// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub enum KmsConfiguration {
    #[allow(missing_docs)] // documentation missing in model
KmsKeyArn(::std::string::String),
#[allow(missing_docs)] // documentation missing in model
KmsMrKeyArn(::std::string::String),
#[allow(missing_docs)] // documentation missing in model
Discovery(crate::material_providers::types::Discovery),
#[allow(missing_docs)] // documentation missing in model
MrDiscovery(crate::material_providers::types::MrDiscovery),
    /// The `Unknown` variant represents cases where new union variant was received. Consider upgrading the SDK to the latest available version.
    /// An unknown enum variant
    ///
    /// _Note: If you encounter this error, consider upgrading your SDK to the latest version._
    /// The `Unknown` variant represents cases where the server sent a value that wasn't recognized
    /// by the client. This can happen when the server adds new functionality, but the client has not been updated.
    /// To investigate this, consider turning on debug logging to print the raw HTTP response.
    #[non_exhaustive]
    Unknown,
}
impl KmsConfiguration {
    /// Tries to convert the enum instance into [`KmsKeyArn`](crate::material_providers::types::KmsConfiguration::KmsKeyArn), extracting the inner [`::std::string::String`](::std::string::String).
/// Returns `Err(&Self)` if it can't be converted.
pub fn as_kms_key_arn(&self) -> ::std::result::Result<&::std::string::String, &Self> {
    if let crate::material_providers::types::KmsConfiguration::KmsKeyArn(val) = &self {
        ::std::result::Result::Ok(val)
    } else {
        ::std::result::Result::Err(self)
    }
}
/// Tries to convert the enum instance into [`KmsMrKeyArn`](crate::material_providers::types::KmsConfiguration::KmsMrKeyArn), extracting the inner [`::std::string::String`](::std::string::String).
/// Returns `Err(&Self)` if it can't be converted.
pub fn as_kms_mr_key_arn(&self) -> ::std::result::Result<&::std::string::String, &Self> {
    if let crate::material_providers::types::KmsConfiguration::KmsMrKeyArn(val) = &self {
        ::std::result::Result::Ok(val)
    } else {
        ::std::result::Result::Err(self)
    }
}
/// Tries to convert the enum instance into [`Discovery`](crate::material_providers::types::KmsConfiguration::Discovery), extracting the inner [`crate::types::Discovery`](crate::types::Discovery).
/// Returns `Err(&Self)` if it can't be converted.
pub fn as_discovery(&self) -> ::std::result::Result<&crate::material_providers::types::Discovery, &Self> {
    if let crate::material_providers::types::KmsConfiguration::Discovery(val) = &self {
        ::std::result::Result::Ok(val)
    } else {
        ::std::result::Result::Err(self)
    }
}
/// Tries to convert the enum instance into [`MrDiscovery`](crate::material_providers::types::KmsConfiguration::MrDiscovery), extracting the inner [`crate::types::MrDiscovery`](crate::types::MrDiscovery).
/// Returns `Err(&Self)` if it can't be converted.
pub fn as_mr_discovery(&self) -> ::std::result::Result<&crate::material_providers::types::MrDiscovery, &Self> {
    if let crate::material_providers::types::KmsConfiguration::MrDiscovery(val) = &self {
        ::std::result::Result::Ok(val)
    } else {
        ::std::result::Result::Err(self)
    }
}
    /// Returns true if this is a [`KmsKeyArn`](crate::material_providers::types::KmsConfiguration::KmsKeyArn).
pub fn is_kms_key_arn(&self) -> ::std::primitive::bool {
    self.as_kms_key_arn().is_ok()
}
/// Returns true if this is a [`KmsMrKeyArn`](crate::material_providers::types::KmsConfiguration::KmsMrKeyArn).
pub fn is_kms_mr_key_arn(&self) -> ::std::primitive::bool {
    self.as_kms_mr_key_arn().is_ok()
}
/// Returns true if this is a [`Discovery`](crate::material_providers::types::KmsConfiguration::Discovery).
pub fn is_discovery(&self) -> ::std::primitive::bool {
    self.as_discovery().is_ok()
}
/// Returns true if this is a [`MrDiscovery`](crate::material_providers::types::KmsConfiguration::MrDiscovery).
pub fn is_mr_discovery(&self) -> ::std::primitive::bool {
    self.as_mr_discovery().is_ok()
}
    /// Returns true if the enum instance is the `Unknown` variant.
    pub fn is_unknown(&self) -> ::std::primitive::bool {
        matches!(self, Self::Unknown)
    }
}
