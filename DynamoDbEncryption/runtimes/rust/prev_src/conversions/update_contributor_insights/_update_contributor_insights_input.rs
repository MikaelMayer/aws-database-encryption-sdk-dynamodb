// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::update_contributor_insights::UpdateContributorInsightsInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::UpdateContributorInsightsInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::UpdateContributorInsightsInput::UpdateContributorInsightsInput {
        TableName: crate::standard_library_conversions::ostring_to_dafny(&value.table_name) .Extract(),
 IndexName: crate::standard_library_conversions::ostring_to_dafny(&value.index_name),
 ContributorInsightsAction: dynamodb::conversions::contributor_insights_action::to_dafny(value.contributor_insights_action.clone().unwrap()),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::UpdateContributorInsightsInput,
    >,
) -> crate::operation::update_contributor_insights::UpdateContributorInsightsInput {
    crate::operation::update_contributor_insights::UpdateContributorInsightsInput::builder()
        .set_table_name(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.TableName()) ))
 .set_index_name(crate::standard_library_conversions::ostring_from_dafny(dafny_value.IndexName().clone()))
 .set_contributor_insights_action(Some( dynamodb::conversions::contributor_insights_action::from_dafny(dafny_value.ContributorInsightsAction()) ))
        .build()
        .unwrap()
}
