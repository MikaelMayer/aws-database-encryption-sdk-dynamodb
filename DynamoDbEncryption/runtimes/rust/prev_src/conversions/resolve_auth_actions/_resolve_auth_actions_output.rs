// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::resolve_auth_actions::ResolveAuthActionsOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ResolveAuthActionsOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ResolveAuthActionsOutput::ResolveAuthActionsOutput {
        cryptoActions: ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&value.crypto_actions.clone().unwrap(),
    |e| structured_encryption::conversions::crypto_item::to_dafny(e.clone())
,
)
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ResolveAuthActionsOutput,
    >,
) -> crate::operation::resolve_auth_actions::ResolveAuthActionsOutput {
    crate::operation::resolve_auth_actions::ResolveAuthActionsOutput::builder()
        .set_crypto_actions(Some( ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(dafny_value.cryptoActions(),
    |e| structured_encryption::conversions::crypto_item::from_dafny(e.clone())
,
)
 ))
        .build()
        .unwrap()
}
