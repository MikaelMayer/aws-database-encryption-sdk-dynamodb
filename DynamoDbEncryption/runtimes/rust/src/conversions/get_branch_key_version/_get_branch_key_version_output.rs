// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::get_branch_key_version::GetBranchKeyVersionOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GetBranchKeyVersionOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GetBranchKeyVersionOutput::GetBranchKeyVersionOutput {
        branchKeyMaterials: key_store::conversions::branch_key_materials::to_dafny(&value.branch_key_materials.clone().unwrap())
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GetBranchKeyVersionOutput,
    >,
) -> crate::operation::get_branch_key_version::GetBranchKeyVersionOutput {
    crate::operation::get_branch_key_version::GetBranchKeyVersionOutput::builder()
        .set_branch_key_materials(Some( key_store::conversions::branch_key_materials::from_dafny(dafny_value.branchKeyMaterials().clone())
 ))
        .build()
        .unwrap()
}
