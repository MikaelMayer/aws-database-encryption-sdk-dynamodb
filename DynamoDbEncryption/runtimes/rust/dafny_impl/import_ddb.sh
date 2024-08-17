#!/bin/bash

cd $( dirname -- "${BASH_SOURCE[0]}" )

rm -rf src/ddb
cp -r /Users/ajewell/gzg/smithy-dafny/TestModels/aws-sdks/ddb-lite/runtimes/rust/src src/ddb
rm -f src/ddb/implementation_from_dafny.rs
find src/ddb -name '*.rs' | fgrep -v ImplementationFromDafny | xargs perl -i -pe 's/crate::_Wrappers_Compile/crate::Wrappers/'
find src/ddb -name '*.rs' | fgrep -v ImplementationFromDafny | xargs perl -i -pe 's/crate::r#_Wrappers_Compile/crate::Wrappers/'
find src/ddb -name '*.rs' | fgrep -v ImplementationFromDafny | xargs perl -i -pe 's/crate::conversions/crate::ddb::conversions/'
find src/ddb -name '*.rs' | fgrep -v ImplementationFromDafny | xargs perl -i -pe 's/crate::standard_library_conversions/crate::ddb::standard_library_conversions/'
cat extra_ddb.txt >> src/ddb/client.rs
