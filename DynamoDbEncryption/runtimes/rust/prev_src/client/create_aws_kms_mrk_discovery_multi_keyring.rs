// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`CreateAwsKmsMrkDiscoveryMultiKeyring`](crate::operation::create_aws_kms_mrk_discovery_multi_keyring::builders::CreateAwsKmsMrkDiscoveryMultiKeyringFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`client_supplier(impl Into<Option<material_providers::types::client_supplier::ClientSupplierRef>>)`](crate::operation::create_aws_kms_mrk_discovery_multi_keyring::builders::CreateAwsKmsMrkDiscoveryMultiKeyringFluentBuilder::client_supplier) / [`set_client_supplier(Option<material_providers::types::client_supplier::ClientSupplierRef>)`](crate::operation::create_aws_kms_mrk_discovery_multi_keyring::builders::CreateAwsKmsMrkDiscoveryMultiKeyringFluentBuilder::set_client_supplier): (undocumented)<br>
    ///   - [`discovery_filter(impl Into<Option<material_providers::types::DiscoveryFilter>>)`](crate::operation::create_aws_kms_mrk_discovery_multi_keyring::builders::CreateAwsKmsMrkDiscoveryMultiKeyringFluentBuilder::discovery_filter) / [`set_discovery_filter(Option<material_providers::types::DiscoveryFilter>)`](crate::operation::create_aws_kms_mrk_discovery_multi_keyring::builders::CreateAwsKmsMrkDiscoveryMultiKeyringFluentBuilder::set_discovery_filter): (undocumented)<br>
    ///   - [`grant_tokens(impl Into<Option<::std::vec::Vec<::std::string::String>>>)`](crate::operation::create_aws_kms_mrk_discovery_multi_keyring::builders::CreateAwsKmsMrkDiscoveryMultiKeyringFluentBuilder::grant_tokens) / [`set_grant_tokens(Option<::std::vec::Vec<::std::string::String>>)`](crate::operation::create_aws_kms_mrk_discovery_multi_keyring::builders::CreateAwsKmsMrkDiscoveryMultiKeyringFluentBuilder::set_grant_tokens): (undocumented)<br>
    ///   - [`regions(impl Into<Option<::std::vec::Vec<::std::string::String>>>)`](crate::operation::create_aws_kms_mrk_discovery_multi_keyring::builders::CreateAwsKmsMrkDiscoveryMultiKeyringFluentBuilder::regions) / [`set_regions(Option<::std::vec::Vec<::std::string::String>>)`](crate::operation::create_aws_kms_mrk_discovery_multi_keyring::builders::CreateAwsKmsMrkDiscoveryMultiKeyringFluentBuilder::set_regions): (undocumented)<br>
    /// - On success, responds with [`CreateKeyringOutput`](crate::operation::create_aws_kms_mrk_discovery_multi_keyring::CreateKeyringOutput) with field(s):
    ///   - [`keyring(Option<material_providers::types::keyring::KeyringRef>)`](crate::operation::create_aws_kms_mrk_discovery_multi_keyring::CreateKeyringOutput::keyring): (undocumented)
    /// - On failure, responds with [`SdkError<CreateAwsKmsMrkDiscoveryMultiKeyringError>`](crate::operation::create_aws_kms_mrk_discovery_multi_keyring::CreateAwsKmsMrkDiscoveryMultiKeyringError)
    pub fn create_aws_kms_mrk_discovery_multi_keyring(&self) -> crate::operation::create_aws_kms_mrk_discovery_multi_keyring::builders::CreateAwsKmsMrkDiscoveryMultiKeyringFluentBuilder {
        crate::operation::create_aws_kms_mrk_discovery_multi_keyring::builders::CreateAwsKmsMrkDiscoveryMultiKeyringFluentBuilder::new(self.clone())
    }
}
