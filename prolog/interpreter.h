#pragma once
#include "defines.h"

#include <vector>
#include <array>

#define MAX_SYMBOLS_PER_FACT 16

struct Symbol {
    u32 id;
};

inline bool operator==(const Symbol& a, const Symbol& b) { return a.id == b.id; }
inline bool operator!=(const Symbol& a, const Symbol& b) { return a.id != b.id; }

struct Relation {
    u32 id;
};

inline bool operator==(const Relation& a, const Relation& b) { return a.id == b.id; }
inline bool operator!=(const Relation& a, const Relation& b) { return a.id != b.id; }

struct Fact {
    Relation name;
    u32 symbolCount;
    Symbol symbols[MAX_SYMBOLS_PER_FACT];
};

struct Rule {
    Relation name;
    u32 variableCount;
    struct Check {
        Relation rel;
        u32 variables[MAX_SYMBOLS_PER_FACT];
    };
    std::vector<Check> checks;
};

struct Interpreter {
    std::vector<Fact> facts;
    std::vector<Rule> rules;

    std::vector<Fact> getFactsByRelation(Relation rel) {
        std::vector<Fact> res;
        for (u32 i = 0; i < facts.size(); i++) {
            if (facts[i].name == rel) {
                res.push_back(facts[i]);
            }
        }
        return res;
    }

    std::vector<Rule> getRulesByRelation(Relation rel) {
        std::vector<Rule> res;
        for (u32 i = 0; i < rules.size(); i++) {
            if (rules[i].name == rel) {
                res.push_back(rules[i]);
            }
        }
        return res;
    }

    bool checkFact(Fact* fact, u32 symbolCount, Symbol* symbols);

    bool checkRule(Rule* rule, u32 symbolCount, Symbol* symbols);
};