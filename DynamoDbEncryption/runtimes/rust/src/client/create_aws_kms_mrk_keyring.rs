// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`CreateAwsKmsMrkKeyring`](crate::operation::create_aws_kms_mrk_keyring::builders::CreateAwsKmsMrkKeyringFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`grant_tokens(impl Into<Option<::std::vec::Vec<::std::string::String>>>)`](crate::operation::create_aws_kms_mrk_keyring::builders::CreateAwsKmsMrkKeyringFluentBuilder::grant_tokens) / [`set_grant_tokens(Option<::std::vec::Vec<::std::string::String>>)`](crate::operation::create_aws_kms_mrk_keyring::builders::CreateAwsKmsMrkKeyringFluentBuilder::set_grant_tokens): (undocumented)<br>
    ///   - [`kms_client(impl Into<Option<kms::client::Client>>)`](crate::operation::create_aws_kms_mrk_keyring::builders::CreateAwsKmsMrkKeyringFluentBuilder::kms_client) / [`set_kms_client(Option<kms::client::Client>)`](crate::operation::create_aws_kms_mrk_keyring::builders::CreateAwsKmsMrkKeyringFluentBuilder::set_kms_client): (undocumented)<br>
    ///   - [`kms_key_id(impl Into<Option<::std::string::String>>)`](crate::operation::create_aws_kms_mrk_keyring::builders::CreateAwsKmsMrkKeyringFluentBuilder::kms_key_id) / [`set_kms_key_id(Option<::std::string::String>)`](crate::operation::create_aws_kms_mrk_keyring::builders::CreateAwsKmsMrkKeyringFluentBuilder::set_kms_key_id): (undocumented)<br>
    /// - On success, responds with [`CreateKeyringOutput`](crate::operation::create_aws_kms_mrk_keyring::CreateKeyringOutput) with field(s):
    ///   - [`keyring(Option<material_providers::types::keyring::KeyringRef>)`](crate::operation::create_aws_kms_mrk_keyring::CreateKeyringOutput::keyring): (undocumented)
    /// - On failure, responds with [`SdkError<CreateAwsKmsMrkKeyringError>`](crate::operation::create_aws_kms_mrk_keyring::CreateAwsKmsMrkKeyringError)
    pub fn create_aws_kms_mrk_keyring(&self) -> crate::operation::create_aws_kms_mrk_keyring::builders::CreateAwsKmsMrkKeyringFluentBuilder {
        crate::operation::create_aws_kms_mrk_keyring::builders::CreateAwsKmsMrkKeyringFluentBuilder::new(self.clone())
    }
}
