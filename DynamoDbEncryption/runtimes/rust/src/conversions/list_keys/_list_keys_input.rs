// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::list_keys::ListKeysRequest,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ListKeysRequest,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ListKeysRequest::ListKeysRequest {
        Limit: crate::standard_library_conversions::oint_to_dafny(value.limit),
 Marker: crate::standard_library_conversions::ostring_to_dafny(&value.marker),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ListKeysRequest,
    >,
) -> crate::operation::list_keys::ListKeysRequest {
    crate::operation::list_keys::ListKeysRequest::builder()
        .set_limit(crate::standard_library_conversions::oint_from_dafny(dafny_value.Limit().clone()))
 .set_marker(crate::standard_library_conversions::ostring_from_dafny(dafny_value.Marker().clone()))
        .build()
        .unwrap()
}
