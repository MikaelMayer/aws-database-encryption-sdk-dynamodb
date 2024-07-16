// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![allow(warnings, unconditional_panic)]
#![allow(nonstandard_style)]
use crate::*;

impl crate::software::amazon::cryptography::services::kms::internaldafny::_default {
    pub fn KMSClient() -> ::std::rc::Rc<Wrappers::Result<::dafny_runtime::Object<dyn crate::software::amazon::cryptography::services::kms::internaldafny::types::IKMSClient>, ::std::rc::Rc<crate::software::amazon::cryptography::services::kms::internaldafny::types::Error>>>{
        todo!("software::amazon::cryptography::services::kms::internaldafny::KMSClient not implemented");
    }
    pub fn KMSClientForRegion(_region: &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>) -> ::std::rc::Rc<Wrappers::Result<::dafny_runtime::Object<dyn crate::software::amazon::cryptography::services::kms::internaldafny::types::IKMSClient>, ::std::rc::Rc<crate::software::amazon::cryptography::services::kms::internaldafny::types::Error>>>{
        todo!("software::amazon::cryptography::services::kms::internaldafny::KMSClientForRegion not implemented");
    }
    pub fn RegionMatch(
        _kmsClient: &::dafny_runtime::Object<dyn crate::software::amazon::cryptography::services::kms::internaldafny::types::IKMSClient>,
        _region: &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
    ) -> ::std::rc::Rc<Wrappers::Option<bool>> {
        todo!("software::amazon::cryptography::services::kms::internaldafny::RegionMatch not implemented");
    }
}
