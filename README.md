testdb should exist, 
testdb_2 shouldn't

cargo test  should show success (connection to testdb_2 is not available - we got 503)

In fact ROCKET_FEATURES=test cargo test will give the expected result