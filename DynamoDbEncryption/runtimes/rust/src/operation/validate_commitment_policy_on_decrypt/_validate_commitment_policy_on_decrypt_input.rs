// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ValidateCommitmentPolicyOnDecryptInput {
    #[allow(missing_docs)] // documentation missing in model
pub algorithm: ::std::option::Option<material_providers::types::AlgorithmSuiteId>,
#[allow(missing_docs)] // documentation missing in model
pub commitment_policy: ::std::option::Option<material_providers::types::CommitmentPolicy>,
}
impl ValidateCommitmentPolicyOnDecryptInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn algorithm(&self) -> &::std::option::Option<material_providers::types::AlgorithmSuiteId> {
    &self.algorithm
}
#[allow(missing_docs)] // documentation missing in model
pub fn commitment_policy(&self) -> &::std::option::Option<material_providers::types::CommitmentPolicy> {
    &self.commitment_policy
}
}
impl ValidateCommitmentPolicyOnDecryptInput {
    /// Creates a new builder-style object to manufacture [`ValidateCommitmentPolicyOnDecryptInput`](crate::operation::validate_commitment_policy_on_decrypt::builders::ValidateCommitmentPolicyOnDecryptInput).
    pub fn builder() -> crate::operation::validate_commitment_policy_on_decrypt::builders::ValidateCommitmentPolicyOnDecryptInputBuilder {
        crate::operation::validate_commitment_policy_on_decrypt::builders::ValidateCommitmentPolicyOnDecryptInputBuilder::default()
    }
}

/// A builder for [`ValidateCommitmentPolicyOnDecryptInput`](crate::operation::operation::ValidateCommitmentPolicyOnDecryptInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ValidateCommitmentPolicyOnDecryptInputBuilder {
    pub(crate) algorithm: ::std::option::Option<material_providers::types::AlgorithmSuiteId>,
pub(crate) commitment_policy: ::std::option::Option<material_providers::types::CommitmentPolicy>,
}
impl ValidateCommitmentPolicyOnDecryptInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn algorithm(mut self, input: impl ::std::convert::Into<material_providers::types::AlgorithmSuiteId>) -> Self {
    self.algorithm = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_algorithm(mut self, input: ::std::option::Option<material_providers::types::AlgorithmSuiteId>) -> Self {
    self.algorithm = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_algorithm(&self) -> &::std::option::Option<material_providers::types::AlgorithmSuiteId> {
    &self.algorithm
}
#[allow(missing_docs)] // documentation missing in model
pub fn commitment_policy(mut self, input: impl ::std::convert::Into<material_providers::types::CommitmentPolicy>) -> Self {
    self.commitment_policy = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_commitment_policy(mut self, input: ::std::option::Option<material_providers::types::CommitmentPolicy>) -> Self {
    self.commitment_policy = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_commitment_policy(&self) -> &::std::option::Option<material_providers::types::CommitmentPolicy> {
    &self.commitment_policy
}
    /// Consumes the builder and constructs a [`ValidateCommitmentPolicyOnDecryptInput`](crate::operation::operation::ValidateCommitmentPolicyOnDecryptInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::validate_commitment_policy_on_decrypt::ValidateCommitmentPolicyOnDecryptInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::validate_commitment_policy_on_decrypt::ValidateCommitmentPolicyOnDecryptInput {
            algorithm: self.algorithm,
commitment_policy: self.commitment_policy,
        })
    }
}
