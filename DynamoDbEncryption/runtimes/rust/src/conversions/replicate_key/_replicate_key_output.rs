// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::replicate_key::ReplicateKeyResponse,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ReplicateKeyResponse,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ReplicateKeyResponse::ReplicateKeyResponse {
        ReplicaKeyMetadata: ::std::rc::Rc::new(match &value.replica_key_metadata {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: kms::conversions::key_metadata::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 ReplicaPolicy: crate::standard_library_conversions::ostring_to_dafny(&value.replica_policy),
 ReplicaTags: ::std::rc::Rc::new(match &value.replica_tags {
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
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ReplicateKeyResponse,
    >,
) -> crate::operation::replicate_key::ReplicateKeyResponse {
    crate::operation::replicate_key::ReplicateKeyResponse::builder()
        .set_replica_key_metadata(match (*dafny_value.ReplicaKeyMetadata()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(kms::conversions::key_metadata::from_dafny(value.clone())),
    _ => None,
}
)
 .set_replica_policy(crate::standard_library_conversions::ostring_from_dafny(dafny_value.ReplicaPolicy().clone()))
 .set_replica_tags(match (*dafny_value.ReplicaTags()).as_ref() {
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
