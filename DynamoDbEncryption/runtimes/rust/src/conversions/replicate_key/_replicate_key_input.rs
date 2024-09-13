// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::replicate_key::ReplicateKeyRequest,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ReplicateKeyRequest,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ReplicateKeyRequest::ReplicateKeyRequest {
        KeyId: crate::standard_library_conversions::ostring_to_dafny(&value.key_id) .Extract(),
 ReplicaRegion: crate::standard_library_conversions::ostring_to_dafny(&value.replica_region) .Extract(),
 Policy: crate::standard_library_conversions::ostring_to_dafny(&value.policy),
 BypassPolicyLockoutSafetyCheck: crate::standard_library_conversions::obool_to_dafny(&value.bypass_policy_lockout_safety_check),
 Description: crate::standard_library_conversions::ostring_to_dafny(&value.description),
 Tags: ::std::rc::Rc::new(match &value.tags {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| kms::conversions::tag::to_dafny(e.clone())
,
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ReplicateKeyRequest,
    >,
) -> crate::operation::replicate_key::ReplicateKeyRequest {
    crate::operation::replicate_key::ReplicateKeyRequest::builder()
        .set_key_id(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.KeyId()) ))
 .set_replica_region(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.ReplicaRegion()) ))
 .set_policy(crate::standard_library_conversions::ostring_from_dafny(dafny_value.Policy().clone()))
 .set_bypass_policy_lockout_safety_check(crate::standard_library_conversions::obool_from_dafny(dafny_value.BypassPolicyLockoutSafetyCheck().clone()))
 .set_description(crate::standard_library_conversions::ostring_from_dafny(dafny_value.Description().clone()))
 .set_tags(match (*dafny_value.Tags()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e| kms::conversions::tag::from_dafny(e.clone())
,
            )
        ),
    _ => None
}
)
        .build()
        .unwrap()
}