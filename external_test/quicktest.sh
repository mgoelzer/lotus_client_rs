#!/bin/sh

./target/debug/indexer --max=9 > TMPFILE && diff TMPFILE ./external_test/expected-quicktest.data > /dev/null
if [ $? -eq 0 ] ; then
	echo OK
	rm TMPFILE
else
	echo ERROR
fi
