// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::get_client::GetClientOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GetClientOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GetClientOutput::GetClientOutput {
        client: kms::conversions::client::to_dafny(&value.client.clone().unwrap())
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GetClientOutput,
    >,
) -> crate::operation::get_client::GetClientOutput {
    crate::operation::get_client::GetClientOutput::builder()
        .set_client(Some( kms::conversions::client::from_dafny(dafny_value.client().clone())
 ))
        .build()
        .unwrap()
}
