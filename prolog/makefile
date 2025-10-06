CXX:=clang
CXXFLAGS:=-std=c++17 -Wall -Wextra -Werror -g

%.o: %.cpp
	$(CXX) $(CXXFLAGS) -c $< -o $@

OBJECTS=$(patsubst %.cpp, %.o, $(wildcard *.cpp))

prolog.exe: $(OBJECTS)
	$(CXX) $^ -o $@ -g

debug: prolog.exe