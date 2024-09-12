// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::list_grants::ListGrantsRequest,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ListGrantsRequest,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ListGrantsRequest::ListGrantsRequest {
        Limit: crate::standard_library_conversions::oint_to_dafny(value.limit),
 Marker: crate::standard_library_conversions::ostring_to_dafny(&value.marker),
 KeyId: crate::standard_library_conversions::ostring_to_dafny(&value.key_id) .Extract(),
 GrantId: crate::standard_library_conversions::ostring_to_dafny(&value.grant_id),
 GranteePrincipal: crate::standard_library_conversions::ostring_to_dafny(&value.grantee_principal),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ListGrantsRequest,
    >,
) -> crate::operation::list_grants::ListGrantsRequest {
    crate::operation::list_grants::ListGrantsRequest::builder()
        .set_limit(crate::standard_library_conversions::oint_from_dafny(dafny_value.Limit().clone()))
 .set_marker(crate::standard_library_conversions::ostring_from_dafny(dafny_value.Marker().clone()))
 .set_key_id(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.KeyId()) ))
 .set_grant_id(crate::standard_library_conversions::ostring_from_dafny(dafny_value.GrantId().clone()))
 .set_grantee_principal(crate::standard_library_conversions::ostring_from_dafny(dafny_value.GranteePrincipal().clone()))
        .build()
        .unwrap()
}
