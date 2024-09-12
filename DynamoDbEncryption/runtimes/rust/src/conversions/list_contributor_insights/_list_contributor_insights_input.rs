// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::list_contributor_insights::ListContributorInsightsInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ListContributorInsightsInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ListContributorInsightsInput::ListContributorInsightsInput {
        TableName: crate::standard_library_conversions::ostring_to_dafny(&value.table_name),
 NextToken: crate::standard_library_conversions::ostring_to_dafny(&value.next_token),
 MaxResults: crate::standard_library_conversions::oint_to_dafny(value.max_results),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ListContributorInsightsInput,
    >,
) -> crate::operation::list_contributor_insights::ListContributorInsightsInput {
    crate::operation::list_contributor_insights::ListContributorInsightsInput::builder()
        .set_table_name(crate::standard_library_conversions::ostring_from_dafny(dafny_value.TableName().clone()))
 .set_next_token(crate::standard_library_conversions::ostring_from_dafny(dafny_value.NextToken().clone()))
 .set_max_results(crate::standard_library_conversions::oint_from_dafny(dafny_value.MaxResults().clone()))
        .build()
        .unwrap()
}
