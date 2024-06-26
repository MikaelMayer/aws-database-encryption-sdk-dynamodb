// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![allow(warnings, unconditional_panic)]
#![allow(nonstandard_style)]
use crate::ImplementationFromDafny::*;
use std::time::SystemTime;

impl Time::_default {
  pub fn CurrentRelativeTime() -> u64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
      Ok(n) => n.as_secs(),
      Err(_) => 0
    }
  }

  pub fn GetCurrentTimeStamp() -> ::std::rc::Rc<Wrappers::Result<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>> {
    todo!("Time::GetCurrentTimeStamp not implemented");
    /*
      let now_utc = chrono::Utc::now();
      let formatted = format!("{}", date_time.format("%Y-%m-%dT%H:%M:%S:%.fZ"));
      return Result<icharseq, icharseq>.create_Success(charseq.FromString(timestamp));
    */
  }
}
