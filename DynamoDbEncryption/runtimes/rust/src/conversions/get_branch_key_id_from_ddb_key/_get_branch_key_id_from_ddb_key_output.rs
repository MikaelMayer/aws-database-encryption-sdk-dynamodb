// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::get_branch_key_id_from_ddb_key::GetBranchKeyIdFromDdbKeyOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GetBranchKeyIdFromDdbKeyOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GetBranchKeyIdFromDdbKeyOutput::GetBranchKeyIdFromDdbKeyOutput {
        branchKeyId: crate::standard_library_conversions::ostring_to_dafny(&value.branch_key_id) .Extract(),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GetBranchKeyIdFromDdbKeyOutput,
    >,
) -> crate::operation::get_branch_key_id_from_ddb_key::GetBranchKeyIdFromDdbKeyOutput {
    crate::operation::get_branch_key_id_from_ddb_key::GetBranchKeyIdFromDdbKeyOutput::builder()
        .set_branch_key_id(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.branchKeyId()) ))
        .build()
        .unwrap()
}
