from parsimonious.grammar import Grammar

grammar = open("sample.pest", "r").read()
print(grammar)
grammar = Grammar(grammar)
print(grammar)
