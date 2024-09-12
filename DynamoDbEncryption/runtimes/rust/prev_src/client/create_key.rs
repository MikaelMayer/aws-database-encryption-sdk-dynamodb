// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`CreateKey`](crate::operation::create_key::builders::CreateKeyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`bypass_policy_lockout_safety_check(impl Into<Option<::std::primitive::bool>>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::bypass_policy_lockout_safety_check) / [`set_bypass_policy_lockout_safety_check(Option<::std::primitive::bool>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::set_bypass_policy_lockout_safety_check): (undocumented)<br>
    ///   - [`custom_key_store_id(impl Into<Option<::std::string::String>>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::custom_key_store_id) / [`set_custom_key_store_id(Option<::std::string::String>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::set_custom_key_store_id): (undocumented)<br>
    ///   - [`customer_master_key_spec(impl Into<Option<kms::types::CustomerMasterKeySpec>>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::customer_master_key_spec) / [`set_customer_master_key_spec(Option<kms::types::CustomerMasterKeySpec>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::set_customer_master_key_spec): (undocumented)<br>
    ///   - [`description(impl Into<Option<::std::string::String>>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::description) / [`set_description(Option<::std::string::String>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::set_description): (undocumented)<br>
    ///   - [`key_spec(impl Into<Option<kms::types::KeySpec>>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::key_spec) / [`set_key_spec(Option<kms::types::KeySpec>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::set_key_spec): (undocumented)<br>
    ///   - [`key_usage(impl Into<Option<kms::types::KeyUsageType>>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::key_usage) / [`set_key_usage(Option<kms::types::KeyUsageType>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::set_key_usage): (undocumented)<br>
    ///   - [`multi_region(impl Into<Option<::std::primitive::bool>>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::multi_region) / [`set_multi_region(Option<::std::primitive::bool>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::set_multi_region): (undocumented)<br>
    ///   - [`origin(impl Into<Option<kms::types::OriginType>>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::origin) / [`set_origin(Option<kms::types::OriginType>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::set_origin): (undocumented)<br>
    ///   - [`policy(impl Into<Option<::std::string::String>>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::policy) / [`set_policy(Option<::std::string::String>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::set_policy): (undocumented)<br>
    ///   - [`tags(impl Into<Option<::std::vec::Vec<kms::types::Tag>>>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::tags) / [`set_tags(Option<::std::vec::Vec<kms::types::Tag>>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::set_tags): (undocumented)<br>
    ///   - [`xks_key_id(impl Into<Option<::std::string::String>>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::xks_key_id) / [`set_xks_key_id(Option<::std::string::String>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::set_xks_key_id): (undocumented)<br>
    /// - On success, responds with [`CreateKeyResponse`](crate::operation::create_key::CreateKeyResponse) with field(s):
    ///   - [`key_metadata(Option<kms::types::KeyMetadata>)`](crate::operation::create_key::CreateKeyResponse::key_metadata): (undocumented)
    /// - On failure, responds with [`SdkError<CreateKeyError>`](crate::operation::create_key::CreateKeyError)
    pub fn create_key(&self) -> crate::operation::create_key::builders::CreateKeyFluentBuilder {
        crate::operation::create_key::builders::CreateKeyFluentBuilder::new(self.clone())
    }
}
