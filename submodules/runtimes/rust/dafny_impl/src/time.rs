// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![allow(warnings, unconditional_panic)]
#![allow(nonstandard_style)]

pub mod Time {
    use crate::*;
    use std::time::SystemTime;

    pub struct _default {}
    impl _default {
        pub fn CurrentRelativeTime() -> i64 {
            match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
                Ok(n) => n.as_secs() as i64,
                Err(_) => 0,
            }
        }

        pub fn GetCurrentTimeStamp() -> ::std::rc::Rc<
            Wrappers::Result<
                ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
            >,
        > {
            let now_utc = chrono::Utc::now();
            let formatted = format!("{}", now_utc.format("%Y-%m-%dT%H:%M:%S:%.fZ"));
            ::std::rc::Rc::new(
        Wrappers::Result::Success{value :
          dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&formatted)
        }
      )
        }
    }
}
