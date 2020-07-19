#!/bin/sh

RUST_LOG="info" ./target/debug/indexer --max=10 2>&1 | grep -v "check_endpoint_connection" > TMPFILE
diff TMPFILE ./external_test/expected-quicktest.data > /dev/null
if [ $? -eq 0 ] ; then
	echo OK
	rm TMPFILE
else
	echo ERROR
fi
