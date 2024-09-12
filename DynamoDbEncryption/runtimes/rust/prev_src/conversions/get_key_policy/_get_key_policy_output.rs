// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::get_key_policy::GetKeyPolicyResponse,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GetKeyPolicyResponse,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GetKeyPolicyResponse::GetKeyPolicyResponse {
        Policy: crate::standard_library_conversions::ostring_to_dafny(&value.policy),
 PolicyName: crate::standard_library_conversions::ostring_to_dafny(&value.policy_name),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GetKeyPolicyResponse,
    >,
) -> crate::operation::get_key_policy::GetKeyPolicyResponse {
    crate::operation::get_key_policy::GetKeyPolicyResponse::builder()
        .set_policy(crate::standard_library_conversions::ostring_from_dafny(dafny_value.Policy().clone()))
 .set_policy_name(crate::standard_library_conversions::ostring_from_dafny(dafny_value.PolicyName().clone()))
        .build()
        .unwrap()
}
