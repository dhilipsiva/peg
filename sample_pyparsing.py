from pyparsing import alphanums, numeric

indent = ~numeric + alphanums
indent_list = indent + (" " + indent)
