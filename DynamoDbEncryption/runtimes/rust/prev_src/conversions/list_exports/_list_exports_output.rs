// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::list_exports::ListExportsOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ListExportsOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ListExportsOutput::ListExportsOutput {
        ExportSummaries: ::std::rc::Rc::new(match &value.export_summaries {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| dynamodb::conversions::export_summary::to_dafny(e.clone())
,
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
 NextToken: crate::standard_library_conversions::ostring_to_dafny(&value.next_token),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ListExportsOutput,
    >,
) -> crate::operation::list_exports::ListExportsOutput {
    crate::operation::list_exports::ListExportsOutput::builder()
        .set_export_summaries(match (*dafny_value.ExportSummaries()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e| dynamodb::conversions::export_summary::from_dafny(e.clone())
,
            )
        ),
    _ => None
}
)
 .set_next_token(crate::standard_library_conversions::ostring_from_dafny(dafny_value.NextToken().clone()))
        .build()
        .unwrap()
}
