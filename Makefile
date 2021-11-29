test:
	g++ -std=c++17 -o interval-cover-test interval-cover-test.cc
	./interval-cover-test

.PHONY: test
