WARNINGS=-Wall -Wextra -pedantic

gamerank: gamerank.c
	gcc $(WARNINGS) -o $@ $<

addingwords: addingwords.cc
	g++ -std=c++17 $(WARNINGS) -o $@ $<

imperfectgps: imperfectgps.c
	gcc -std=c99 $(WARNINGS) -o $@ $< -lm

quantum: quantum.cc
	g++ -std=c++17 $(WARNINGS) -o $@ $<

test-interval-cover: interval-cover-test.cc
	g++ -std=c++17 $(WARNINGS) -o interval-cover-test $<
	./interval-cover-test

clean:
	rm gamerank addingwords imperfectgps quantum interval-cover-test tsp

.PHONY: test all
