// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`ValidAlgorithmSuiteInfo`](crate::operation::valid_algorithm_suite_info::builders::ValidAlgorithmSuiteInfoFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`binary_id(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::valid_algorithm_suite_info::builders::ValidAlgorithmSuiteInfoFluentBuilder::binary_id) / [`set_binary_id(Option<::aws_smithy_types::Blob>)`](crate::operation::valid_algorithm_suite_info::builders::ValidAlgorithmSuiteInfoFluentBuilder::set_binary_id): (undocumented)<br>
    ///   - [`commitment(impl Into<Option<material_providers::types::DerivationAlgorithm>>)`](crate::operation::valid_algorithm_suite_info::builders::ValidAlgorithmSuiteInfoFluentBuilder::commitment) / [`set_commitment(Option<material_providers::types::DerivationAlgorithm>)`](crate::operation::valid_algorithm_suite_info::builders::ValidAlgorithmSuiteInfoFluentBuilder::set_commitment): (undocumented)<br>
    ///   - [`edk_wrapping(impl Into<Option<material_providers::types::EdkWrappingAlgorithm>>)`](crate::operation::valid_algorithm_suite_info::builders::ValidAlgorithmSuiteInfoFluentBuilder::edk_wrapping) / [`set_edk_wrapping(Option<material_providers::types::EdkWrappingAlgorithm>)`](crate::operation::valid_algorithm_suite_info::builders::ValidAlgorithmSuiteInfoFluentBuilder::set_edk_wrapping): (undocumented)<br>
    ///   - [`encrypt(impl Into<Option<material_providers::types::Encrypt>>)`](crate::operation::valid_algorithm_suite_info::builders::ValidAlgorithmSuiteInfoFluentBuilder::encrypt) / [`set_encrypt(Option<material_providers::types::Encrypt>)`](crate::operation::valid_algorithm_suite_info::builders::ValidAlgorithmSuiteInfoFluentBuilder::set_encrypt): (undocumented)<br>
    ///   - [`id(impl Into<Option<material_providers::types::AlgorithmSuiteId>>)`](crate::operation::valid_algorithm_suite_info::builders::ValidAlgorithmSuiteInfoFluentBuilder::id) / [`set_id(Option<material_providers::types::AlgorithmSuiteId>)`](crate::operation::valid_algorithm_suite_info::builders::ValidAlgorithmSuiteInfoFluentBuilder::set_id): (undocumented)<br>
    ///   - [`kdf(impl Into<Option<material_providers::types::DerivationAlgorithm>>)`](crate::operation::valid_algorithm_suite_info::builders::ValidAlgorithmSuiteInfoFluentBuilder::kdf) / [`set_kdf(Option<material_providers::types::DerivationAlgorithm>)`](crate::operation::valid_algorithm_suite_info::builders::ValidAlgorithmSuiteInfoFluentBuilder::set_kdf): (undocumented)<br>
    ///   - [`message_version(impl Into<Option<::std::primitive::i32>>)`](crate::operation::valid_algorithm_suite_info::builders::ValidAlgorithmSuiteInfoFluentBuilder::message_version) / [`set_message_version(Option<::std::primitive::i32>)`](crate::operation::valid_algorithm_suite_info::builders::ValidAlgorithmSuiteInfoFluentBuilder::set_message_version): (undocumented)<br>
    ///   - [`signature(impl Into<Option<material_providers::types::SignatureAlgorithm>>)`](crate::operation::valid_algorithm_suite_info::builders::ValidAlgorithmSuiteInfoFluentBuilder::signature) / [`set_signature(Option<material_providers::types::SignatureAlgorithm>)`](crate::operation::valid_algorithm_suite_info::builders::ValidAlgorithmSuiteInfoFluentBuilder::set_signature): (undocumented)<br>
    ///   - [`symmetric_signature(impl Into<Option<material_providers::types::SymmetricSignatureAlgorithm>>)`](crate::operation::valid_algorithm_suite_info::builders::ValidAlgorithmSuiteInfoFluentBuilder::symmetric_signature) / [`set_symmetric_signature(Option<material_providers::types::SymmetricSignatureAlgorithm>)`](crate::operation::valid_algorithm_suite_info::builders::ValidAlgorithmSuiteInfoFluentBuilder::set_symmetric_signature): (undocumented)<br>
    /// - On success, responds with [`Unit`](crate::operation::valid_algorithm_suite_info::Unit) with field(s):

    /// - On failure, responds with [`SdkError<ValidAlgorithmSuiteInfoError>`](crate::operation::valid_algorithm_suite_info::ValidAlgorithmSuiteInfoError)
    pub fn valid_algorithm_suite_info(&self) -> crate::operation::valid_algorithm_suite_info::builders::ValidAlgorithmSuiteInfoFluentBuilder {
        crate::operation::valid_algorithm_suite_info::builders::ValidAlgorithmSuiteInfoFluentBuilder::new(self.clone())
    }
}
