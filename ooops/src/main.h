#include <iostream>
#include <stdexcept>
#include <string>
#include <thread>
#include <chrono>

class LineareFunktion {
    float m;
    float n;

    public:
    LineareFunktion(float m, float n): m(m), n(n) {
        if(m==0)throw std::runtime_error("nene");
    }

    float eval(float x) {
        return (m * x) + n;
    }

    float nullstelle() {
        return -n/m;
    }
};

namespace Color {
    inline const char* red() { return "\e[31m"; }
    inline const char* green() { return "\e[32m"; }
    inline const char* none() { return "\e[0m"; }
};