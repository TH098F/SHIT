#pragma once
#include "defines.h"

#include <exception>
#include <stdexcept>
#include <vector>
#include <string>

#define MAX_SYMBOLS_PER_FACT 16

struct Symbol {
    u32 id;

    enum {
        ANY
    };

    constexpr operator u32() { return id; }
};

constexpr bool operator==(Symbol a, Symbol b) { return a.id == b.id; }
constexpr bool operator!=(Symbol a, Symbol b) { return a.id != b.id; }


struct Fact {
    Symbol name;
    u32 symbolCount;
    Symbol symbols[MAX_SYMBOLS_PER_FACT];

    bool check(u32 symbolCount, Symbol* symbols);
};

enum Operator {
    AND,
    OR
};

union Expression {
    u8 type : 1;
    struct {
        Expression* left;
        Expression* right;
        Operator op;
    } a;
    struct {
        Symbol name;
        u32 symbolCount;
        Symbol Symbols[MAX_SYMBOLS_PER_FACT];
    } b;

    bool check(u32 symbolCount, Symbol* symbols);
};

struct Rule {
    Symbol name;
    Expression* startingExpr;
    Expression expressions[MAX_SYMBOLS_PER_FACT];

    bool check(u32 symbolCount, Symbol* symbols);
};

struct Interpreter {
    std::vector<Fact> facts;
    std::vector<Rule> rules;

    const Fact& getFactByName(Symbol name) {
        for (const auto& fact : facts) {
            if (fact.name == name) return fact;
        }
        throw std::runtime_error("Fact Not Found");
    }

    const Rule& getRuleByName(Symbol name) {
        for (const auto& rule : rules) {
            if (rule.name == name) return rule;
        }
        throw std::runtime_error("Rule Not Found");
    }

    void readFile(const char* name);
    bool parseQuery(const char* query);
};