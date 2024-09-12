// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`ReplicateKey`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`bypass_policy_lockout_safety_check(impl Into<Option<::std::primitive::bool>>)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::bypass_policy_lockout_safety_check) / [`set_bypass_policy_lockout_safety_check(Option<::std::primitive::bool>)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::set_bypass_policy_lockout_safety_check): (undocumented)<br>
    ///   - [`description(impl Into<Option<::std::string::String>>)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::description) / [`set_description(Option<::std::string::String>)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::set_description): (undocumented)<br>
    ///   - [`key_id(impl Into<Option<::std::string::String>>)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::key_id) / [`set_key_id(Option<::std::string::String>)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::set_key_id): (undocumented)<br>
    ///   - [`policy(impl Into<Option<::std::string::String>>)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::policy) / [`set_policy(Option<::std::string::String>)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::set_policy): (undocumented)<br>
    ///   - [`replica_region(impl Into<Option<::std::string::String>>)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::replica_region) / [`set_replica_region(Option<::std::string::String>)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::set_replica_region): (undocumented)<br>
    ///   - [`tags(impl Into<Option<::std::vec::Vec<kms::types::Tag>>>)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::tags) / [`set_tags(Option<::std::vec::Vec<kms::types::Tag>>)`](crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::set_tags): (undocumented)<br>
    /// - On success, responds with [`ReplicateKeyResponse`](crate::operation::replicate_key::ReplicateKeyResponse) with field(s):
    ///   - [`replica_key_metadata(Option<kms::types::KeyMetadata>)`](crate::operation::replicate_key::ReplicateKeyResponse::replica_key_metadata): (undocumented)
    ///   - [`replica_policy(Option<::std::string::String>)`](crate::operation::replicate_key::ReplicateKeyResponse::replica_policy): (undocumented)
    ///   - [`replica_tags(Option<::std::vec::Vec<kms::types::Tag>>)`](crate::operation::replicate_key::ReplicateKeyResponse::replica_tags): (undocumented)
    /// - On failure, responds with [`SdkError<ReplicateKeyError>`](crate::operation::replicate_key::ReplicateKeyError)
    pub fn replicate_key(&self) -> crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder {
        crate::operation::replicate_key::builders::ReplicateKeyFluentBuilder::new(self.clone())
    }
}
