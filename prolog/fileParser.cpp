#include "interpreter.h"
#include <stdexcept>

struct Token {
    enum Type {
        SYMBOL,
        RULE_DELIMITER,
        DOT,
        COMMA,
        SEMI_COLON,
        PARAN_OPEN,
        PARAN_CLOSE
    } type;
    u32 id;
};

void Interpreter::readFile(const char* name) {
    std::vector<Token> tokens = {};

    u32 lastId = 0;
    for (const char* c = name; c != 0; c++) {
        switch (*c) {
            case ':':
                if (*(c + 1) == '-') {
                    c++; // greed
                    tokens.push_back({.type = Token::RULE_DELIMITER, .id = lastId});
                    lastId++;
                } else throw std::runtime_error("Invalid Syntax");
                break;
            case '.':
                tokens.push_back({.type = Token::DOT, .id = lastId});
                lastId++;
                break;
            case ';':
                tokens.push_back({.type = Token::SEMI_COLON, .id = lastId});
                lastId++;
                break;
            case '(':
                tokens.push_back({.type = Token::PARAN_OPEN, .id = lastId});
                lastId++;
                break;
            case ')':
                tokens.push_back({.type = Token::PARAN_CLOSE, .id = lastId});
                lastId++;
                break;
            default:
                if (
                    *c <= 'a' || *c >= 'z' ||
                    *c <= 'A' || *c >= 'Z' ||
                    *c <= '0' || *c >= '9'
                ) {
                    throw std::runtime_error("Invalid Syntax");
                }
                break;
        }
    }
}