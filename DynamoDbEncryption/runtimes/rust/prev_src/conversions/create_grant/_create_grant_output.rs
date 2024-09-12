// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::create_grant::CreateGrantResponse,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateGrantResponse,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateGrantResponse::CreateGrantResponse {
        GrantToken: crate::standard_library_conversions::ostring_to_dafny(&value.grant_token),
 GrantId: crate::standard_library_conversions::ostring_to_dafny(&value.grant_id),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateGrantResponse,
    >,
) -> crate::operation::create_grant::CreateGrantResponse {
    crate::operation::create_grant::CreateGrantResponse::builder()
        .set_grant_token(crate::standard_library_conversions::ostring_from_dafny(dafny_value.GrantToken().clone()))
 .set_grant_id(crate::standard_library_conversions::ostring_from_dafny(dafny_value.GrantId().clone()))
        .build()
        .unwrap()
}
