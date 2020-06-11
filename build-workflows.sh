#!/bin/bash

function create_workflow {
  OUTPUT_FILE=$1

  mkdir -p build

  rm -v -rf release
  rm -v -rf $OUTPUT_FILE.alfredworkflow
  rm -v -rf info.plist

  mkdir -p release{/,/target/release}
  cargo build --release
  cp $OUTPUT_FILE.plist info.plist

  # zip it
  zip -9 -r $OUTPUT_FILE.alfredworkflow icon.png info.plist target/release/github
  mv $OUTPUT_FILE.alfredworkflow build/$OUTPUT_FILE.alfredworkflow
  rm -v -rf info.plist
}

create_workflow my-github-search
create_workflow github-search