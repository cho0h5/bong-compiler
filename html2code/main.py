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

i = 0

def parsing(line):
    print("    if state == " + str(i) + " {")
    print("        match token {")
    j = 0
    for l in line.find_all('td'):
        content = l.get_text()
        if len(content.strip()) != 0 and j != 0:
            print_line(content, j)

        j += 1
    print("            _ => return None,")
    print("        };")
    print("    }")

def print_line(content, j):
    if label[j] == "LogOp" or label[j] == "RelOp" or label[j] == "AddOp" or label[j] == "MulOp" or label[j] == "UnaryOp" or label[j] == "AddMinus" or label[j] == "StringLit" or label[j] == "IntLit" or label[j] == "Identifier":
        print("            " + label[j] + "(_)" + " => ", end='')

    else:
        print("            " + label[j] + " => ", end='')
    print_rear(content)

def print_rear(content):
    if content[0] == 'r':
        print("return Some(Reduce({})),".format(content[1:]))
    elif content[0] == 's':
        print("return Some(Shift({})),".format(content[1:]))
    else:
        print("return Some(Goto({})),".format(content))

f = open("table.html", 'r')

while True:
    line = f.readline()
    if not line: break

    print()
    soup = bs(line, 'html.parser').select_one('tr')
    parsing(soup)
    i += 1

f.close()
