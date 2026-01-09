OBJECTS=$(patsubst src/%.cpp,build/%.o,$(wildcard src/*.cpp))

build/%.o: src/%.cpp $(wildcard src/*.h)
	clang++ $< -o $@ -std=c++17 -g -c

bin/debug: $(OBJECTS)
	clang++ $(OBJECTS) -o bin/debug -g

setup:
	mkdir -p bin
	mkdir -p build

debug: bin/debug