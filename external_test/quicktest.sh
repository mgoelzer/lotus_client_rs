#!/bin/sh

./target/debug/indexer --max=9 > TMPFILE && md5 TMPFILE | diff - ./external_test/expected-quicktest.data > /dev/null
if [ $? -eq 0 ] ; then
	echo OK
	rm TMPFILE
else
	echo ERROR
fi
