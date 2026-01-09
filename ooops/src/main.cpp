#include "main.h"

int main(void) {
    std::cout << Color::red() << "X";
    std::this_thread::sleep_for(std::chrono::seconds(1));
}