// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::create_aws_kms_hierarchical_keyring::CreateAwsKmsHierarchicalKeyringInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateAwsKmsHierarchicalKeyringInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateAwsKmsHierarchicalKeyringInput::CreateAwsKmsHierarchicalKeyringInput {
        branchKeyId: crate::standard_library_conversions::ostring_to_dafny(&value.branch_key_id),
 branchKeyIdSupplier: ::std::rc::Rc::new(match &value.branch_key_id_supplier {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: material_providers::conversions::branch_key_id_supplier::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 keyStore: key_store::conversions::client::to_dafny(&value.key_store.clone().unwrap())
,
 ttlSeconds: crate::standard_library_conversions::olong_to_dafny(&value.ttl_seconds),
 cache: ::std::rc::Rc::new(match &value.cache {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: material_providers::conversions::cache_type::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateAwsKmsHierarchicalKeyringInput,
    >,
) -> crate::operation::create_aws_kms_hierarchical_keyring::CreateAwsKmsHierarchicalKeyringInput {
    crate::operation::create_aws_kms_hierarchical_keyring::CreateAwsKmsHierarchicalKeyringInput::builder()
        .set_branch_key_id(crate::standard_library_conversions::ostring_from_dafny(dafny_value.branchKeyId().clone()))
 .set_branch_key_id_supplier(match (*dafny_value.branchKeyIdSupplier()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(material_providers::conversions::branch_key_id_supplier::from_dafny(value.clone())),
    _ => None,
}
)
 .set_key_store(Some( key_store::conversions::client::from_dafny(dafny_value.keyStore().clone())
 ))
 .set_ttl_seconds(Some( dafny_value.ttlSeconds() .clone() ))
 .set_cache(match (*dafny_value.cache()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(material_providers::conversions::cache_type::from_dafny(value.clone())),
    _ => None,
}
)
        .build()
        .unwrap()
}
