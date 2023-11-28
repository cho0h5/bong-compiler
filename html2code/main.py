#! /usr/bin/python3

# https://jsmachines.sourceforge.net/machines/slr.html의 parsing table의
# html을 parsing하여 rust 코드를 출력하는 프로그램입니다.

from bs4 import BeautifulSoup as bs

label = (
    "dummy"
    ,"Int","Void","Lbracket","IntLit","Rbracket","Star","Lbrace","Rbrace","Semicolon","Identifier"
    ,"Lparen","Rparen","Comma","StringLit","LogOp","RelOp","AddOp","AddMinus","MulOp","And","UnaryOp","AssignOp"
    ,"If","While","Return","Break","Continue"
    ,"EOL"
    ,"PROGRAM_","PROGRAM","TYPE","ARRAY_TYPE","POINTER_TYPE","BLOCK","STATEMENT_LIST","VAR_DECL","FUNCTION_DECL","PARAMETERS"
    ,"PARAMETER_LIST","PARAMETER_DECL","OPERAND","PRIMARY_EXPR","INDEX","ARGUMENTS","EXPRESSION_LIST","EXPRESSION","LOGICAL_EXPR","RELATIONAL_EXPR"
    ,"ADDITIVE_EXPR","MULTIPLICATIVE_EXPR","UNARY_EXPR","STATEMENT","ASSIGNMENT","IF_STMT","WHILE_STMT","RETURN_STMT","BREAK_STMT","CONTINUE_STMT"
)

t_LogOp = (
    "Or",
    "And",
)

t_RelOp = {
    "Equal",
    "NotEqual",
    "Less",
    "LessEqual",
    "Greater",
    "GreaterEqual",
}

t_AddOp = {
    "BitwiseOr",
    "BitwiseAnd",
}

t_MulOp = {
    "Div",
    "Mod",
    "LeftShift",
    "RightShift",
}

t_UnaryOp = {
    "Not",
    "BitwiseNot",
}

t_AddMinus = {
    "Add",
    "Minus",
}

i = 0

def parsing(line):
    print("    // for state", i)
    print("    let mut hashmap = HashMap::new();")
    j = 0
    for l in line.find_all('td'):
        content = l.get_text()
        if len(content.strip()) != 0 and j != 0:
            print_line(content, j)

        j += 1
    print("    table.push(hashmap);")

def print_line(content, j):
    if label[j] == "LogOp":
        print_mult(label[j], "LogicalOperator", t_LogOp, j, content)
    elif label[j] == "RelOp":
        print_mult(label[j], "RelativeOperator", t_RelOp, j, content)
    elif label[j] == "AddOp":
        print_mult(label[j], "AdditiveOperator", t_AddOp, j, content)
    elif label[j] == "AddMinus":
        print_mult(label[j], "AddMinusOperator", t_AddMinus, j, content)
    elif label[j] == "MulOp":
        print_mult(label[j], "MultiplicativeOperator", t_MulOp, j, content)
    elif label[j] == "UnaryOp":
        print_mult(label[j], "UnaryOperator", t_UnaryOp, j, content)
    else:
        print("    hashmap.insert({}, ".format(label[j]), end='')
        print_rear(content)

def print_mult(labelname, realname, tup, j, content):
    for e in tup:
        print("    hashmap.insert({}({}::{}), ".format(labelname, realname, e), end='')
        print_rear(content)

def print_rear(content):
    if content[0] == 'r':
        print("Reduce({}));".format(content[1:]))
    elif content[0] == 's':
        print("Shift({}));".format(content[1:]))
    else:
        print("Goto({}));".format(content))

f = open("table.html", 'r')

while True:
    line = f.readline()
    if not line: break

    print()
    soup = bs(line, 'html.parser').select_one('tr')
    parsing(soup)
    i += 1

f.close()
