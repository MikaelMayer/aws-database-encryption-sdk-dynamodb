#!/bin/bash

cd $( dirname -- "${BASH_SOURCE[0]}" )

rm -rf src/kms
cp -r /Users/ajewell/gzg/smithy-dafny/TestModels/aws-sdks/kms-lite/runtimes/rust/src src/kms
rm -f src/kms/implementation_from_dafny.rs
find src/kms -name '*.rs' | fgrep -v ImplementationFromDafny | xargs perl -i -pe 's/crate::_Wrappers_Compile/crate::Wrappers/'
find src/kms -name '*.rs' | fgrep -v ImplementationFromDafny | xargs perl -i -pe 's/crate::r#_Wrappers_Compile/crate::Wrappers/'
find src/kms -name '*.rs' | fgrep -v ImplementationFromDafny | xargs perl -i -pe 's/crate::conversions/crate::kms::conversions/'
find src/kms -name '*.rs' | fgrep -v ImplementationFromDafny | xargs perl -i -pe 's/crate::standard_library_conversions/crate::kms::standard_library_conversions/'
cat extra_kms.txt >> src/kms/client.rs
