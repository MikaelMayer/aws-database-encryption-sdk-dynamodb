// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::resolve_auth_actions::ResolveAuthActionsInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ResolveAuthActionsInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ResolveAuthActionsInput::ResolveAuthActionsInput {
        tableName: crate::standard_library_conversions::ostring_to_dafny(&value.table_name) .Extract(),
 authActions: ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&value.auth_actions.clone().unwrap(),
    |e| structured_encryption::conversions::auth_item::to_dafny(e.clone())
,
)
,
 headerBytes: crate::standard_library_conversions::oblob_to_dafny(&value.header_bytes).Extract(),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ResolveAuthActionsInput,
    >,
) -> crate::operation::resolve_auth_actions::ResolveAuthActionsInput {
    crate::operation::resolve_auth_actions::ResolveAuthActionsInput::builder()
        .set_table_name(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.tableName()) ))
 .set_auth_actions(Some( ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(dafny_value.authActions(),
    |e| structured_encryption::conversions::auth_item::from_dafny(e.clone())
,
)
 ))
 .set_header_bytes(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.headerBytes().clone())))
        .build()
        .unwrap()
}
