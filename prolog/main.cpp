#include "interpreter.h"
#include <iostream>

bool Interpreter::checkFact(Fact* fact, u32 symbolCount, Symbol* symbols) {
    if (fact->symbolCount != symbolCount) return false;
    for (u32 i = 0; i < symbolCount; i++) {
        if (fact->symbols[i] != symbols[i]) return false;
    }

    return true;
};

bool Interpreter::checkRule(Rule* rule, u32 symbolCount, Symbol* symbols) {
    if (symbolCount != rule->variableCount) return false;
    Symbol _symbols[MAX_SYMBOLS_PER_FACT];
    for (u32 i = 0; i < rule->checks.size(); i++) {
        for (u32 j = 0; j < rule->variableCount; j++) {
            _symbols[j] = symbols[rule->checks[i].variables[j]];
        }
        
        auto facts = getFactsByRelation(rule->checks[i].rel);
        for (Fact& fact : facts) {
            if (checkFact(&fact, rule->variableCount, _symbols)) {
                break;
            }
            return false;
        }
        auto rules = getRulesByRelation(rule->checks[i].rel);
        for (Rule& _rule : rules) {
            if (checkRule(&_rule, _rule.variableCount, _symbols)) {
                break;
            }
            return false;
        }
    }
    return true;
}

void Interpreter::readFile(const char* path) {

}

void Interpreter::parseQuery(const std::string& query) {
    
}

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

        intptr.parseQuery(query);
    }

    return 0;
}