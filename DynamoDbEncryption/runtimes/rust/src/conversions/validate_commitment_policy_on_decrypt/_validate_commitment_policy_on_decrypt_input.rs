// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::validate_commitment_policy_on_decrypt::ValidateCommitmentPolicyOnDecryptInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ValidateCommitmentPolicyOnDecryptInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ValidateCommitmentPolicyOnDecryptInput::ValidateCommitmentPolicyOnDecryptInput {
        algorithm: material_providers::conversions::algorithm_suite_id::to_dafny(&value.algorithm.clone().unwrap())
,
 commitmentPolicy: material_providers::conversions::commitment_policy::to_dafny(&value.commitment_policy.clone().unwrap())
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ValidateCommitmentPolicyOnDecryptInput,
    >,
) -> crate::operation::validate_commitment_policy_on_decrypt::ValidateCommitmentPolicyOnDecryptInput {
    crate::operation::validate_commitment_policy_on_decrypt::ValidateCommitmentPolicyOnDecryptInput::builder()
        .set_algorithm(Some( material_providers::conversions::algorithm_suite_id::from_dafny(dafny_value.algorithm().clone())
 ))
 .set_commitment_policy(Some( material_providers::conversions::commitment_policy::from_dafny(dafny_value.commitmentPolicy().clone())
 ))
        .build()
        .unwrap()
}
