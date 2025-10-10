#include "interpreter.h"
#include <iostream>


int main(void) {
    Interpreter intptr = {};

    std::cout << "File: " << std::flush;

    std::string path;
    std::cin >> path;

    intptr.readFile(path.c_str());

    while (true) {
        std::cout << "?- " << std::flush;

        std::string query;
        std::cin >> query;

        intptr.parseQuery(query.c_str());
    }

    return 0;
}