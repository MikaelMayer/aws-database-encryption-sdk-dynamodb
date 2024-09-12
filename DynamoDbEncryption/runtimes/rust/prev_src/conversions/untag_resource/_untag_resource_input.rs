// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::untag_resource::UntagResourceRequest,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::UntagResourceRequest,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::UntagResourceRequest::UntagResourceRequest {
        KeyId: crate::standard_library_conversions::ostring_to_dafny(&value.key_id) .Extract(),
 TagKeys: ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&value.tag_keys.clone().unwrap(),
    |e| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&e),
)
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::UntagResourceRequest,
    >,
) -> crate::operation::untag_resource::UntagResourceRequest {
    crate::operation::untag_resource::UntagResourceRequest::builder()
        .set_key_id(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.KeyId()) ))
 .set_tag_keys(Some( ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(dafny_value.TagKeys(),
    |e| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(e),
)
 ))
        .build()
        .unwrap()
}
