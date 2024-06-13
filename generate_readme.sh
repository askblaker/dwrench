#!/bin/bash
FILE=README.md

cat > $FILE  <<- EOM
# dwrench  
A cli tool to do various data related tasks.

Status: Work barely in progress

## Usage
### help
\`\`\`
EOM

cargo run -- --help >> $FILE

echo '```' >> $FILE

echo '' >> $FILE

echo '### csvconvert' >> $FILE

echo '```' >> $FILE

cargo run csvconvert --help >> $FILE

echo '```' >> $FILE