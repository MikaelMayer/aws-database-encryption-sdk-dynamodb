// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::describe_key::DescribeKeyResponse,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DescribeKeyResponse,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DescribeKeyResponse::DescribeKeyResponse {
        KeyMetadata: ::std::rc::Rc::new(match &value.key_metadata {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: kms::conversions::key_metadata::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DescribeKeyResponse,
    >,
) -> crate::operation::describe_key::DescribeKeyResponse {
    crate::operation::describe_key::DescribeKeyResponse::builder()
        .set_key_metadata(match (*dafny_value.KeyMetadata()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(kms::conversions::key_metadata::from_dafny(value.clone())),
    _ => None,
}
)
        .build()
        .unwrap()
}
