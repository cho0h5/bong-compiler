
    // for state 0
    let mut hashmap = HashMap::new();
    hashmap.insert(Int, Shift(4));
    hashmap.insert(Void, Shift(5));
    hashmap.insert(EOL, Reduce(2));
    hashmap.insert(PROGRAM, Goto(1));
    hashmap.insert(TYPE, Goto(3));
    hashmap.insert(ARRAY_TYPE, Goto(6));
    hashmap.insert(POINTER_TYPE, Goto(7));
    hashmap.insert(FUNCTION_DECL, Goto(2));
    table.push(hashmap);

    // for state 1
    let mut hashmap = HashMap::new();
    hashmap.insert(EOL, Goto(acc));
    table.push(hashmap);

    // for state 2
    let mut hashmap = HashMap::new();
    hashmap.insert(Int, Shift(4));
    hashmap.insert(Void, Shift(5));
    hashmap.insert(EOL, Reduce(2));
    hashmap.insert(PROGRAM, Goto(8));
    hashmap.insert(TYPE, Goto(3));
    hashmap.insert(ARRAY_TYPE, Goto(6));
    hashmap.insert(POINTER_TYPE, Goto(7));
    hashmap.insert(FUNCTION_DECL, Goto(2));
    table.push(hashmap);

    // for state 3
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Shift(10));
    hashmap.insert(Star, Shift(11));
    hashmap.insert(Identifier, Shift(9));
    table.push(hashmap);

    // for state 4
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(3));
    hashmap.insert(Star, Reduce(3));
    hashmap.insert(Identifier, Reduce(3));
    table.push(hashmap);

    // for state 5
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(4));
    hashmap.insert(Star, Reduce(4));
    hashmap.insert(Identifier, Reduce(4));
    table.push(hashmap);

    // for state 6
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(5));
    hashmap.insert(Star, Reduce(5));
    hashmap.insert(Identifier, Reduce(5));
    table.push(hashmap);

    // for state 7
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(6));
    hashmap.insert(Star, Reduce(6));
    hashmap.insert(Identifier, Reduce(6));
    table.push(hashmap);

    // for state 8
    let mut hashmap = HashMap::new();
    hashmap.insert(EOL, Reduce(1));
    table.push(hashmap);

    // for state 9
    let mut hashmap = HashMap::new();
    hashmap.insert(Lparen, Shift(13));
    hashmap.insert(PARAMETERS, Goto(12));
    table.push(hashmap);

    // for state 10
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(14));
    table.push(hashmap);

    // for state 11
    let mut hashmap = HashMap::new();
    hashmap.insert(Identifier, Reduce(8));
    table.push(hashmap);

    // for state 12
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbrace, Shift(16));
    hashmap.insert(BLOCK, Goto(15));
    table.push(hashmap);

    // for state 13
    let mut hashmap = HashMap::new();
    hashmap.insert(Int, Shift(4));
    hashmap.insert(Void, Shift(5));
    hashmap.insert(Rparen, Reduce(18));
    hashmap.insert(TYPE, Goto(19));
    hashmap.insert(ARRAY_TYPE, Goto(6));
    hashmap.insert(POINTER_TYPE, Goto(7));
    hashmap.insert(PARAMETER_LIST, Goto(17));
    hashmap.insert(PARAMETER_DECL, Goto(18));
    table.push(hashmap);

    // for state 14
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbracket, Shift(20));
    table.push(hashmap);

    // for state 15
    let mut hashmap = HashMap::new();
    hashmap.insert(Int, Reduce(14));
    hashmap.insert(Void, Reduce(14));
    hashmap.insert(EOL, Reduce(14));
    table.push(hashmap);

    // for state 16
    let mut hashmap = HashMap::new();
    hashmap.insert(Int, Shift(4));
    hashmap.insert(Void, Shift(5));
    hashmap.insert(IntLit, Shift(48));
    hashmap.insert(Star, Shift(43));
    hashmap.insert(Rbrace, Reduce(12));
    hashmap.insert(Identifier, Shift(50));
    hashmap.insert(Lparen, Shift(51));
    hashmap.insert(StringLit, Shift(49));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(45));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(45));
    hashmap.insert(And, Shift(44));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(42));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(42));
    hashmap.insert(If, Shift(35));
    hashmap.insert(While, Shift(36));
    hashmap.insert(Return, Shift(32));
    hashmap.insert(Break, Shift(33));
    hashmap.insert(Continue, Shift(34));
    hashmap.insert(TYPE, Goto(31));
    hashmap.insert(ARRAY_TYPE, Goto(6));
    hashmap.insert(POINTER_TYPE, Goto(7));
    hashmap.insert(STATEMENT_LIST, Goto(21));
    hashmap.insert(VAR_DECL, Goto(24));
    hashmap.insert(OPERAND, Goto(47));
    hashmap.insert(PRIMARY_EXPR, Goto(46));
    hashmap.insert(EXPRESSION, Goto(30));
    hashmap.insert(LOGICAL_EXPR, Goto(37));
    hashmap.insert(RELATIONAL_EXPR, Goto(38));
    hashmap.insert(ADDITIVE_EXPR, Goto(39));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(40));
    hashmap.insert(UNARY_EXPR, Goto(41));
    hashmap.insert(STATEMENT, Goto(22));
    hashmap.insert(ASSIGNMENT, Goto(23));
    hashmap.insert(IF_STMT, Goto(28));
    hashmap.insert(WHILE_STMT, Goto(29));
    hashmap.insert(RETURN_STMT, Goto(25));
    hashmap.insert(BREAK_STMT, Goto(26));
    hashmap.insert(CONTINUE_STMT, Goto(27));
    table.push(hashmap);

    // for state 17
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Shift(52));
    table.push(hashmap);

    // for state 18
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Reduce(17));
    hashmap.insert(Comma, Shift(53));
    table.push(hashmap);

    // for state 19
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Shift(10));
    hashmap.insert(Star, Shift(11));
    hashmap.insert(Identifier, Shift(54));
    table.push(hashmap);

    // for state 20
    let mut hashmap = HashMap::new();
    hashmap.insert(Identifier, Reduce(7));
    table.push(hashmap);

    // for state 21
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Shift(55));
    table.push(hashmap);

    // for state 22
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(11));
    hashmap.insert(Semicolon, Shift(56));
    table.push(hashmap);

    // for state 23
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(49));
    hashmap.insert(Semicolon, Reduce(49));
    table.push(hashmap);

    // for state 24
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(50));
    hashmap.insert(Semicolon, Reduce(50));
    table.push(hashmap);

    // for state 25
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(51));
    hashmap.insert(Semicolon, Reduce(51));
    table.push(hashmap);

    // for state 26
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(52));
    hashmap.insert(Semicolon, Reduce(52));
    table.push(hashmap);

    // for state 27
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(53));
    hashmap.insert(Semicolon, Reduce(53));
    table.push(hashmap);

    // for state 28
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(54));
    hashmap.insert(Semicolon, Reduce(54));
    table.push(hashmap);

    // for state 29
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(55));
    hashmap.insert(Semicolon, Reduce(55));
    table.push(hashmap);

    // for state 30
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(56));
    hashmap.insert(Semicolon, Reduce(56));
    hashmap.insert(AssignOp, Shift(57));
    table.push(hashmap);

    // for state 31
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Shift(10));
    hashmap.insert(Star, Shift(11));
    hashmap.insert(Identifier, Shift(58));
    table.push(hashmap);

    // for state 32
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(71));
    hashmap.insert(Star, Shift(66));
    hashmap.insert(Identifier, Shift(73));
    hashmap.insert(Lparen, Shift(74));
    hashmap.insert(StringLit, Shift(72));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(68));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(68));
    hashmap.insert(And, Shift(67));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(65));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(65));
    hashmap.insert(OPERAND, Goto(70));
    hashmap.insert(PRIMARY_EXPR, Goto(69));
    hashmap.insert(EXPRESSION, Goto(59));
    hashmap.insert(LOGICAL_EXPR, Goto(60));
    hashmap.insert(RELATIONAL_EXPR, Goto(61));
    hashmap.insert(ADDITIVE_EXPR, Goto(62));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(63));
    hashmap.insert(UNARY_EXPR, Goto(64));
    table.push(hashmap);

    // for state 33
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(61));
    hashmap.insert(Semicolon, Reduce(61));
    table.push(hashmap);

    // for state 34
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(62));
    hashmap.insert(Semicolon, Reduce(62));
    table.push(hashmap);

    // for state 35
    let mut hashmap = HashMap::new();
    hashmap.insert(Lparen, Shift(75));
    table.push(hashmap);

    // for state 36
    let mut hashmap = HashMap::new();
    hashmap.insert(Lparen, Shift(76));
    table.push(hashmap);

    // for state 37
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(32));
    hashmap.insert(Semicolon, Reduce(32));
    hashmap.insert(LogOp(LogicalOperator::Or), Shift(77));
    hashmap.insert(LogOp(LogicalOperator::And), Shift(77));
    hashmap.insert(AssignOp, Reduce(32));
    table.push(hashmap);

    // for state 38
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(34));
    hashmap.insert(Semicolon, Reduce(34));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(34));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(34));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Shift(78));
    hashmap.insert(RelOp(RelativeOperator::Greater), Shift(78));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Shift(78));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Shift(78));
    hashmap.insert(RelOp(RelativeOperator::Equal), Shift(78));
    hashmap.insert(RelOp(RelativeOperator::Less), Shift(78));
    hashmap.insert(AssignOp, Reduce(34));
    table.push(hashmap);

    // for state 39
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(36));
    hashmap.insert(Semicolon, Reduce(36));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(36));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(36));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(36));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(36));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(36));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(36));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(36));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(36));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Shift(79));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Shift(79));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(80));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(80));
    hashmap.insert(AssignOp, Reduce(36));
    table.push(hashmap);

    // for state 40
    let mut hashmap = HashMap::new();
    hashmap.insert(Star, Shift(82));
    hashmap.insert(Rbrace, Reduce(39));
    hashmap.insert(Semicolon, Reduce(39));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(39));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(39));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(39));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(39));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(39));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(39));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(39));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(39));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(39));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(39));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(39));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(39));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Shift(81));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Shift(81));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Shift(81));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Shift(81));
    hashmap.insert(And, Shift(83));
    hashmap.insert(AssignOp, Reduce(39));
    table.push(hashmap);

    // for state 41
    let mut hashmap = HashMap::new();
    hashmap.insert(Star, Reduce(43));
    hashmap.insert(Rbrace, Reduce(43));
    hashmap.insert(Semicolon, Reduce(43));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(43));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(43));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(43));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(43));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(43));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(43));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(43));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(43));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(43));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(43));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(43));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(43));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(43));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(43));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(43));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(43));
    hashmap.insert(And, Reduce(43));
    hashmap.insert(AssignOp, Reduce(43));
    table.push(hashmap);

    // for state 42
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(48));
    hashmap.insert(Star, Shift(43));
    hashmap.insert(Identifier, Shift(50));
    hashmap.insert(Lparen, Shift(51));
    hashmap.insert(StringLit, Shift(49));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(45));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(45));
    hashmap.insert(And, Shift(44));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(42));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(42));
    hashmap.insert(OPERAND, Goto(47));
    hashmap.insert(PRIMARY_EXPR, Goto(46));
    hashmap.insert(UNARY_EXPR, Goto(84));
    table.push(hashmap);

    // for state 43
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(48));
    hashmap.insert(Star, Shift(43));
    hashmap.insert(Identifier, Shift(50));
    hashmap.insert(Lparen, Shift(51));
    hashmap.insert(StringLit, Shift(49));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(45));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(45));
    hashmap.insert(And, Shift(44));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(42));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(42));
    hashmap.insert(OPERAND, Goto(47));
    hashmap.insert(PRIMARY_EXPR, Goto(46));
    hashmap.insert(UNARY_EXPR, Goto(85));
    table.push(hashmap);

    // for state 44
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(48));
    hashmap.insert(Star, Shift(43));
    hashmap.insert(Identifier, Shift(50));
    hashmap.insert(Lparen, Shift(51));
    hashmap.insert(StringLit, Shift(49));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(45));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(45));
    hashmap.insert(And, Shift(44));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(42));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(42));
    hashmap.insert(OPERAND, Goto(47));
    hashmap.insert(PRIMARY_EXPR, Goto(46));
    hashmap.insert(UNARY_EXPR, Goto(86));
    table.push(hashmap);

    // for state 45
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(48));
    hashmap.insert(Star, Shift(43));
    hashmap.insert(Identifier, Shift(50));
    hashmap.insert(Lparen, Shift(51));
    hashmap.insert(StringLit, Shift(49));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(45));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(45));
    hashmap.insert(And, Shift(44));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(42));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(42));
    hashmap.insert(OPERAND, Goto(47));
    hashmap.insert(PRIMARY_EXPR, Goto(46));
    hashmap.insert(UNARY_EXPR, Goto(87));
    table.push(hashmap);

    // for state 46
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Shift(90));
    hashmap.insert(Star, Reduce(48));
    hashmap.insert(Rbrace, Reduce(48));
    hashmap.insert(Semicolon, Reduce(48));
    hashmap.insert(Lparen, Shift(91));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(48));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(48));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(48));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(48));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(48));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(48));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(48));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(48));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(48));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(48));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(48));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(48));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(48));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(48));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(48));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(48));
    hashmap.insert(And, Reduce(48));
    hashmap.insert(AssignOp, Reduce(48));
    hashmap.insert(INDEX, Goto(88));
    hashmap.insert(ARGUMENTS, Goto(89));
    table.push(hashmap);

    // for state 47
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(26));
    hashmap.insert(Star, Reduce(26));
    hashmap.insert(Rbrace, Reduce(26));
    hashmap.insert(Semicolon, Reduce(26));
    hashmap.insert(Lparen, Reduce(26));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(26));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(26));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(26));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(26));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(26));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(26));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(26));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(26));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(26));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(26));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(26));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(26));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(26));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(26));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(26));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(26));
    hashmap.insert(And, Reduce(26));
    hashmap.insert(AssignOp, Reduce(26));
    table.push(hashmap);

    // for state 48
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(20));
    hashmap.insert(Star, Reduce(20));
    hashmap.insert(Rbrace, Reduce(20));
    hashmap.insert(Semicolon, Reduce(20));
    hashmap.insert(Lparen, Reduce(20));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(20));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(20));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(20));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(20));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(20));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(20));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(20));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(20));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(20));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(20));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(20));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(20));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(20));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(20));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(20));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(20));
    hashmap.insert(And, Reduce(20));
    hashmap.insert(AssignOp, Reduce(20));
    table.push(hashmap);

    // for state 49
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(21));
    hashmap.insert(Star, Reduce(21));
    hashmap.insert(Rbrace, Reduce(21));
    hashmap.insert(Semicolon, Reduce(21));
    hashmap.insert(Lparen, Reduce(21));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(21));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(21));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(21));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(21));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(21));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(21));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(21));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(21));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(21));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(21));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(21));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(21));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(21));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(21));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(21));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(21));
    hashmap.insert(And, Reduce(21));
    hashmap.insert(AssignOp, Reduce(21));
    table.push(hashmap);

    // for state 50
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(22));
    hashmap.insert(Star, Reduce(22));
    hashmap.insert(Rbrace, Reduce(22));
    hashmap.insert(Semicolon, Reduce(22));
    hashmap.insert(Lparen, Reduce(22));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(22));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(22));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(22));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(22));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(22));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(22));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(22));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(22));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(22));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(22));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(22));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(22));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(22));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(22));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(22));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(22));
    hashmap.insert(And, Reduce(22));
    hashmap.insert(AssignOp, Reduce(22));
    table.push(hashmap);

    // for state 51
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(104));
    hashmap.insert(Star, Shift(99));
    hashmap.insert(Identifier, Shift(106));
    hashmap.insert(Lparen, Shift(107));
    hashmap.insert(StringLit, Shift(105));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(101));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(101));
    hashmap.insert(And, Shift(100));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(98));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(98));
    hashmap.insert(OPERAND, Goto(103));
    hashmap.insert(PRIMARY_EXPR, Goto(102));
    hashmap.insert(EXPRESSION, Goto(92));
    hashmap.insert(LOGICAL_EXPR, Goto(93));
    hashmap.insert(RELATIONAL_EXPR, Goto(94));
    hashmap.insert(ADDITIVE_EXPR, Goto(95));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(96));
    hashmap.insert(UNARY_EXPR, Goto(97));
    table.push(hashmap);

    // for state 52
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbrace, Reduce(15));
    table.push(hashmap);

    // for state 53
    let mut hashmap = HashMap::new();
    hashmap.insert(Int, Shift(4));
    hashmap.insert(Void, Shift(5));
    hashmap.insert(Rparen, Reduce(18));
    hashmap.insert(TYPE, Goto(19));
    hashmap.insert(ARRAY_TYPE, Goto(6));
    hashmap.insert(POINTER_TYPE, Goto(7));
    hashmap.insert(PARAMETER_LIST, Goto(108));
    hashmap.insert(PARAMETER_DECL, Goto(18));
    table.push(hashmap);

    // for state 54
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Reduce(19));
    hashmap.insert(Comma, Reduce(19));
    table.push(hashmap);

    // for state 55
    let mut hashmap = HashMap::new();
    hashmap.insert(Int, Reduce(9));
    hashmap.insert(Void, Reduce(9));
    hashmap.insert(EOL, Reduce(9));
    table.push(hashmap);

    // for state 56
    let mut hashmap = HashMap::new();
    hashmap.insert(Int, Shift(4));
    hashmap.insert(Void, Shift(5));
    hashmap.insert(IntLit, Shift(48));
    hashmap.insert(Star, Shift(43));
    hashmap.insert(Rbrace, Reduce(12));
    hashmap.insert(Identifier, Shift(50));
    hashmap.insert(Lparen, Shift(51));
    hashmap.insert(StringLit, Shift(49));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(45));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(45));
    hashmap.insert(And, Shift(44));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(42));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(42));
    hashmap.insert(If, Shift(35));
    hashmap.insert(While, Shift(36));
    hashmap.insert(Return, Shift(32));
    hashmap.insert(Break, Shift(33));
    hashmap.insert(Continue, Shift(34));
    hashmap.insert(TYPE, Goto(31));
    hashmap.insert(ARRAY_TYPE, Goto(6));
    hashmap.insert(POINTER_TYPE, Goto(7));
    hashmap.insert(STATEMENT_LIST, Goto(109));
    hashmap.insert(VAR_DECL, Goto(24));
    hashmap.insert(OPERAND, Goto(47));
    hashmap.insert(PRIMARY_EXPR, Goto(46));
    hashmap.insert(EXPRESSION, Goto(30));
    hashmap.insert(LOGICAL_EXPR, Goto(37));
    hashmap.insert(RELATIONAL_EXPR, Goto(38));
    hashmap.insert(ADDITIVE_EXPR, Goto(39));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(40));
    hashmap.insert(UNARY_EXPR, Goto(41));
    hashmap.insert(STATEMENT, Goto(22));
    hashmap.insert(ASSIGNMENT, Goto(23));
    hashmap.insert(IF_STMT, Goto(28));
    hashmap.insert(WHILE_STMT, Goto(29));
    hashmap.insert(RETURN_STMT, Goto(25));
    hashmap.insert(BREAK_STMT, Goto(26));
    hashmap.insert(CONTINUE_STMT, Goto(27));
    table.push(hashmap);

    // for state 57
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(71));
    hashmap.insert(Star, Shift(66));
    hashmap.insert(Identifier, Shift(73));
    hashmap.insert(Lparen, Shift(74));
    hashmap.insert(StringLit, Shift(72));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(68));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(68));
    hashmap.insert(And, Shift(67));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(65));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(65));
    hashmap.insert(OPERAND, Goto(70));
    hashmap.insert(PRIMARY_EXPR, Goto(69));
    hashmap.insert(EXPRESSION, Goto(110));
    hashmap.insert(LOGICAL_EXPR, Goto(60));
    hashmap.insert(RELATIONAL_EXPR, Goto(61));
    hashmap.insert(ADDITIVE_EXPR, Goto(62));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(63));
    hashmap.insert(UNARY_EXPR, Goto(64));
    table.push(hashmap);

    // for state 58
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(13));
    hashmap.insert(Semicolon, Reduce(13));
    table.push(hashmap);

    // for state 59
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(60));
    hashmap.insert(Semicolon, Reduce(60));
    table.push(hashmap);

    // for state 60
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(32));
    hashmap.insert(Semicolon, Reduce(32));
    hashmap.insert(LogOp(LogicalOperator::Or), Shift(111));
    hashmap.insert(LogOp(LogicalOperator::And), Shift(111));
    table.push(hashmap);

    // for state 61
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(34));
    hashmap.insert(Semicolon, Reduce(34));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(34));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(34));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Shift(112));
    hashmap.insert(RelOp(RelativeOperator::Greater), Shift(112));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Shift(112));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Shift(112));
    hashmap.insert(RelOp(RelativeOperator::Equal), Shift(112));
    hashmap.insert(RelOp(RelativeOperator::Less), Shift(112));
    table.push(hashmap);

    // for state 62
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(36));
    hashmap.insert(Semicolon, Reduce(36));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(36));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(36));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(36));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(36));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(36));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(36));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(36));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(36));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Shift(113));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Shift(113));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(114));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(114));
    table.push(hashmap);

    // for state 63
    let mut hashmap = HashMap::new();
    hashmap.insert(Star, Shift(116));
    hashmap.insert(Rbrace, Reduce(39));
    hashmap.insert(Semicolon, Reduce(39));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(39));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(39));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(39));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(39));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(39));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(39));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(39));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(39));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(39));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(39));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(39));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(39));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Shift(115));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Shift(115));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Shift(115));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Shift(115));
    hashmap.insert(And, Shift(117));
    table.push(hashmap);

    // for state 64
    let mut hashmap = HashMap::new();
    hashmap.insert(Star, Reduce(43));
    hashmap.insert(Rbrace, Reduce(43));
    hashmap.insert(Semicolon, Reduce(43));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(43));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(43));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(43));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(43));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(43));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(43));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(43));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(43));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(43));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(43));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(43));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(43));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(43));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(43));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(43));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(43));
    hashmap.insert(And, Reduce(43));
    table.push(hashmap);

    // for state 65
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(71));
    hashmap.insert(Star, Shift(66));
    hashmap.insert(Identifier, Shift(73));
    hashmap.insert(Lparen, Shift(74));
    hashmap.insert(StringLit, Shift(72));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(68));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(68));
    hashmap.insert(And, Shift(67));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(65));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(65));
    hashmap.insert(OPERAND, Goto(70));
    hashmap.insert(PRIMARY_EXPR, Goto(69));
    hashmap.insert(UNARY_EXPR, Goto(118));
    table.push(hashmap);

    // for state 66
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(71));
    hashmap.insert(Star, Shift(66));
    hashmap.insert(Identifier, Shift(73));
    hashmap.insert(Lparen, Shift(74));
    hashmap.insert(StringLit, Shift(72));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(68));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(68));
    hashmap.insert(And, Shift(67));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(65));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(65));
    hashmap.insert(OPERAND, Goto(70));
    hashmap.insert(PRIMARY_EXPR, Goto(69));
    hashmap.insert(UNARY_EXPR, Goto(119));
    table.push(hashmap);

    // for state 67
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(71));
    hashmap.insert(Star, Shift(66));
    hashmap.insert(Identifier, Shift(73));
    hashmap.insert(Lparen, Shift(74));
    hashmap.insert(StringLit, Shift(72));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(68));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(68));
    hashmap.insert(And, Shift(67));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(65));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(65));
    hashmap.insert(OPERAND, Goto(70));
    hashmap.insert(PRIMARY_EXPR, Goto(69));
    hashmap.insert(UNARY_EXPR, Goto(120));
    table.push(hashmap);

    // for state 68
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(71));
    hashmap.insert(Star, Shift(66));
    hashmap.insert(Identifier, Shift(73));
    hashmap.insert(Lparen, Shift(74));
    hashmap.insert(StringLit, Shift(72));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(68));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(68));
    hashmap.insert(And, Shift(67));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(65));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(65));
    hashmap.insert(OPERAND, Goto(70));
    hashmap.insert(PRIMARY_EXPR, Goto(69));
    hashmap.insert(UNARY_EXPR, Goto(121));
    table.push(hashmap);

    // for state 69
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Shift(124));
    hashmap.insert(Star, Reduce(48));
    hashmap.insert(Rbrace, Reduce(48));
    hashmap.insert(Semicolon, Reduce(48));
    hashmap.insert(Lparen, Shift(125));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(48));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(48));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(48));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(48));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(48));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(48));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(48));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(48));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(48));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(48));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(48));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(48));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(48));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(48));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(48));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(48));
    hashmap.insert(And, Reduce(48));
    hashmap.insert(INDEX, Goto(122));
    hashmap.insert(ARGUMENTS, Goto(123));
    table.push(hashmap);

    // for state 70
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(26));
    hashmap.insert(Star, Reduce(26));
    hashmap.insert(Rbrace, Reduce(26));
    hashmap.insert(Semicolon, Reduce(26));
    hashmap.insert(Lparen, Reduce(26));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(26));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(26));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(26));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(26));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(26));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(26));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(26));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(26));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(26));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(26));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(26));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(26));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(26));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(26));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(26));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(26));
    hashmap.insert(And, Reduce(26));
    table.push(hashmap);

    // for state 71
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(20));
    hashmap.insert(Star, Reduce(20));
    hashmap.insert(Rbrace, Reduce(20));
    hashmap.insert(Semicolon, Reduce(20));
    hashmap.insert(Lparen, Reduce(20));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(20));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(20));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(20));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(20));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(20));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(20));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(20));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(20));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(20));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(20));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(20));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(20));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(20));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(20));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(20));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(20));
    hashmap.insert(And, Reduce(20));
    table.push(hashmap);

    // for state 72
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(21));
    hashmap.insert(Star, Reduce(21));
    hashmap.insert(Rbrace, Reduce(21));
    hashmap.insert(Semicolon, Reduce(21));
    hashmap.insert(Lparen, Reduce(21));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(21));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(21));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(21));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(21));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(21));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(21));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(21));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(21));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(21));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(21));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(21));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(21));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(21));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(21));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(21));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(21));
    hashmap.insert(And, Reduce(21));
    table.push(hashmap);

    // for state 73
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(22));
    hashmap.insert(Star, Reduce(22));
    hashmap.insert(Rbrace, Reduce(22));
    hashmap.insert(Semicolon, Reduce(22));
    hashmap.insert(Lparen, Reduce(22));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(22));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(22));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(22));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(22));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(22));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(22));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(22));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(22));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(22));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(22));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(22));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(22));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(22));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(22));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(22));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(22));
    hashmap.insert(And, Reduce(22));
    table.push(hashmap);

    // for state 74
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(104));
    hashmap.insert(Star, Shift(99));
    hashmap.insert(Identifier, Shift(106));
    hashmap.insert(Lparen, Shift(107));
    hashmap.insert(StringLit, Shift(105));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(101));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(101));
    hashmap.insert(And, Shift(100));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(98));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(98));
    hashmap.insert(OPERAND, Goto(103));
    hashmap.insert(PRIMARY_EXPR, Goto(102));
    hashmap.insert(EXPRESSION, Goto(126));
    hashmap.insert(LOGICAL_EXPR, Goto(93));
    hashmap.insert(RELATIONAL_EXPR, Goto(94));
    hashmap.insert(ADDITIVE_EXPR, Goto(95));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(96));
    hashmap.insert(UNARY_EXPR, Goto(97));
    table.push(hashmap);

    // for state 75
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(104));
    hashmap.insert(Star, Shift(99));
    hashmap.insert(Identifier, Shift(106));
    hashmap.insert(Lparen, Shift(107));
    hashmap.insert(StringLit, Shift(105));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(101));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(101));
    hashmap.insert(And, Shift(100));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(98));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(98));
    hashmap.insert(OPERAND, Goto(103));
    hashmap.insert(PRIMARY_EXPR, Goto(102));
    hashmap.insert(EXPRESSION, Goto(127));
    hashmap.insert(LOGICAL_EXPR, Goto(93));
    hashmap.insert(RELATIONAL_EXPR, Goto(94));
    hashmap.insert(ADDITIVE_EXPR, Goto(95));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(96));
    hashmap.insert(UNARY_EXPR, Goto(97));
    table.push(hashmap);

    // for state 76
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(104));
    hashmap.insert(Star, Shift(99));
    hashmap.insert(Identifier, Shift(106));
    hashmap.insert(Lparen, Shift(107));
    hashmap.insert(StringLit, Shift(105));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(101));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(101));
    hashmap.insert(And, Shift(100));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(98));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(98));
    hashmap.insert(OPERAND, Goto(103));
    hashmap.insert(PRIMARY_EXPR, Goto(102));
    hashmap.insert(EXPRESSION, Goto(128));
    hashmap.insert(LOGICAL_EXPR, Goto(93));
    hashmap.insert(RELATIONAL_EXPR, Goto(94));
    hashmap.insert(ADDITIVE_EXPR, Goto(95));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(96));
    hashmap.insert(UNARY_EXPR, Goto(97));
    table.push(hashmap);

    // for state 77
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(48));
    hashmap.insert(Star, Shift(43));
    hashmap.insert(Identifier, Shift(50));
    hashmap.insert(Lparen, Shift(51));
    hashmap.insert(StringLit, Shift(49));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(45));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(45));
    hashmap.insert(And, Shift(44));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(42));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(42));
    hashmap.insert(OPERAND, Goto(47));
    hashmap.insert(PRIMARY_EXPR, Goto(46));
    hashmap.insert(RELATIONAL_EXPR, Goto(129));
    hashmap.insert(ADDITIVE_EXPR, Goto(39));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(40));
    hashmap.insert(UNARY_EXPR, Goto(41));
    table.push(hashmap);

    // for state 78
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(48));
    hashmap.insert(Star, Shift(43));
    hashmap.insert(Identifier, Shift(50));
    hashmap.insert(Lparen, Shift(51));
    hashmap.insert(StringLit, Shift(49));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(45));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(45));
    hashmap.insert(And, Shift(44));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(42));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(42));
    hashmap.insert(OPERAND, Goto(47));
    hashmap.insert(PRIMARY_EXPR, Goto(46));
    hashmap.insert(ADDITIVE_EXPR, Goto(130));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(40));
    hashmap.insert(UNARY_EXPR, Goto(41));
    table.push(hashmap);

    // for state 79
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(48));
    hashmap.insert(Star, Shift(43));
    hashmap.insert(Identifier, Shift(50));
    hashmap.insert(Lparen, Shift(51));
    hashmap.insert(StringLit, Shift(49));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(45));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(45));
    hashmap.insert(And, Shift(44));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(42));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(42));
    hashmap.insert(OPERAND, Goto(47));
    hashmap.insert(PRIMARY_EXPR, Goto(46));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(131));
    hashmap.insert(UNARY_EXPR, Goto(41));
    table.push(hashmap);

    // for state 80
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(48));
    hashmap.insert(Star, Shift(43));
    hashmap.insert(Identifier, Shift(50));
    hashmap.insert(Lparen, Shift(51));
    hashmap.insert(StringLit, Shift(49));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(45));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(45));
    hashmap.insert(And, Shift(44));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(42));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(42));
    hashmap.insert(OPERAND, Goto(47));
    hashmap.insert(PRIMARY_EXPR, Goto(46));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(132));
    hashmap.insert(UNARY_EXPR, Goto(41));
    table.push(hashmap);

    // for state 81
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(48));
    hashmap.insert(Star, Shift(43));
    hashmap.insert(Identifier, Shift(50));
    hashmap.insert(Lparen, Shift(51));
    hashmap.insert(StringLit, Shift(49));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(45));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(45));
    hashmap.insert(And, Shift(44));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(42));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(42));
    hashmap.insert(OPERAND, Goto(47));
    hashmap.insert(PRIMARY_EXPR, Goto(46));
    hashmap.insert(UNARY_EXPR, Goto(133));
    table.push(hashmap);

    // for state 82
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(48));
    hashmap.insert(Star, Shift(43));
    hashmap.insert(Identifier, Shift(50));
    hashmap.insert(Lparen, Shift(51));
    hashmap.insert(StringLit, Shift(49));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(45));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(45));
    hashmap.insert(And, Shift(44));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(42));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(42));
    hashmap.insert(OPERAND, Goto(47));
    hashmap.insert(PRIMARY_EXPR, Goto(46));
    hashmap.insert(UNARY_EXPR, Goto(134));
    table.push(hashmap);

    // for state 83
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(48));
    hashmap.insert(Star, Shift(43));
    hashmap.insert(Identifier, Shift(50));
    hashmap.insert(Lparen, Shift(51));
    hashmap.insert(StringLit, Shift(49));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(45));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(45));
    hashmap.insert(And, Shift(44));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(42));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(42));
    hashmap.insert(OPERAND, Goto(47));
    hashmap.insert(PRIMARY_EXPR, Goto(46));
    hashmap.insert(UNARY_EXPR, Goto(135));
    table.push(hashmap);

    // for state 84
    let mut hashmap = HashMap::new();
    hashmap.insert(Star, Reduce(44));
    hashmap.insert(Rbrace, Reduce(44));
    hashmap.insert(Semicolon, Reduce(44));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(44));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(44));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(44));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(44));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(44));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(44));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(44));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(44));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(44));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(44));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(44));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(44));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(44));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(44));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(44));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(44));
    hashmap.insert(And, Reduce(44));
    hashmap.insert(AssignOp, Reduce(44));
    table.push(hashmap);

    // for state 85
    let mut hashmap = HashMap::new();
    hashmap.insert(Star, Reduce(45));
    hashmap.insert(Rbrace, Reduce(45));
    hashmap.insert(Semicolon, Reduce(45));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(45));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(45));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(45));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(45));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(45));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(45));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(45));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(45));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(45));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(45));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(45));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(45));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(45));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(45));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(45));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(45));
    hashmap.insert(And, Reduce(45));
    hashmap.insert(AssignOp, Reduce(45));
    table.push(hashmap);

    // for state 86
    let mut hashmap = HashMap::new();
    hashmap.insert(Star, Reduce(46));
    hashmap.insert(Rbrace, Reduce(46));
    hashmap.insert(Semicolon, Reduce(46));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(46));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(46));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(46));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(46));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(46));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(46));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(46));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(46));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(46));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(46));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(46));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(46));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(46));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(46));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(46));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(46));
    hashmap.insert(And, Reduce(46));
    hashmap.insert(AssignOp, Reduce(46));
    table.push(hashmap);

    // for state 87
    let mut hashmap = HashMap::new();
    hashmap.insert(Star, Reduce(47));
    hashmap.insert(Rbrace, Reduce(47));
    hashmap.insert(Semicolon, Reduce(47));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(47));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(47));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(47));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(47));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(47));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(47));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(47));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(47));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(47));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(47));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(47));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(47));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(47));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(47));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(47));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(47));
    hashmap.insert(And, Reduce(47));
    hashmap.insert(AssignOp, Reduce(47));
    table.push(hashmap);

    // for state 88
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(24));
    hashmap.insert(Star, Reduce(24));
    hashmap.insert(Rbrace, Reduce(24));
    hashmap.insert(Semicolon, Reduce(24));
    hashmap.insert(Lparen, Reduce(24));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(24));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(24));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(24));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(24));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(24));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(24));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(24));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(24));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(24));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(24));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(24));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(24));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(24));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(24));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(24));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(24));
    hashmap.insert(And, Reduce(24));
    hashmap.insert(AssignOp, Reduce(24));
    table.push(hashmap);

    // for state 89
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(25));
    hashmap.insert(Star, Reduce(25));
    hashmap.insert(Rbrace, Reduce(25));
    hashmap.insert(Semicolon, Reduce(25));
    hashmap.insert(Lparen, Reduce(25));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(25));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(25));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(25));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(25));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(25));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(25));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(25));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(25));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(25));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(25));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(25));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(25));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(25));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(25));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(25));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(25));
    hashmap.insert(And, Reduce(25));
    hashmap.insert(AssignOp, Reduce(25));
    table.push(hashmap);

    // for state 90
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(148));
    hashmap.insert(Star, Shift(143));
    hashmap.insert(Identifier, Shift(150));
    hashmap.insert(Lparen, Shift(151));
    hashmap.insert(StringLit, Shift(149));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(145));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(145));
    hashmap.insert(And, Shift(144));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(142));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(142));
    hashmap.insert(OPERAND, Goto(147));
    hashmap.insert(PRIMARY_EXPR, Goto(146));
    hashmap.insert(EXPRESSION, Goto(136));
    hashmap.insert(LOGICAL_EXPR, Goto(137));
    hashmap.insert(RELATIONAL_EXPR, Goto(138));
    hashmap.insert(ADDITIVE_EXPR, Goto(139));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(140));
    hashmap.insert(UNARY_EXPR, Goto(141));
    table.push(hashmap);

    // for state 91
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(165));
    hashmap.insert(Star, Shift(160));
    hashmap.insert(Identifier, Shift(167));
    hashmap.insert(Lparen, Shift(168));
    hashmap.insert(Rparen, Reduce(31));
    hashmap.insert(StringLit, Shift(166));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(162));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(162));
    hashmap.insert(And, Shift(161));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(159));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(159));
    hashmap.insert(OPERAND, Goto(164));
    hashmap.insert(PRIMARY_EXPR, Goto(163));
    hashmap.insert(EXPRESSION_LIST, Goto(152));
    hashmap.insert(EXPRESSION, Goto(153));
    hashmap.insert(LOGICAL_EXPR, Goto(154));
    hashmap.insert(RELATIONAL_EXPR, Goto(155));
    hashmap.insert(ADDITIVE_EXPR, Goto(156));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(157));
    hashmap.insert(UNARY_EXPR, Goto(158));
    table.push(hashmap);

    // for state 92
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Shift(169));
    table.push(hashmap);

    // for state 93
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Reduce(32));
    hashmap.insert(LogOp(LogicalOperator::Or), Shift(170));
    hashmap.insert(LogOp(LogicalOperator::And), Shift(170));
    table.push(hashmap);

    // for state 94
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Reduce(34));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(34));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(34));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Shift(171));
    hashmap.insert(RelOp(RelativeOperator::Greater), Shift(171));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Shift(171));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Shift(171));
    hashmap.insert(RelOp(RelativeOperator::Equal), Shift(171));
    hashmap.insert(RelOp(RelativeOperator::Less), Shift(171));
    table.push(hashmap);

    // for state 95
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Reduce(36));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(36));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(36));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(36));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(36));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(36));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(36));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(36));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(36));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Shift(172));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Shift(172));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(173));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(173));
    table.push(hashmap);

    // for state 96
    let mut hashmap = HashMap::new();
    hashmap.insert(Star, Shift(175));
    hashmap.insert(Rparen, Reduce(39));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(39));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(39));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(39));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(39));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(39));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(39));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(39));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(39));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(39));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(39));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(39));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(39));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Shift(174));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Shift(174));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Shift(174));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Shift(174));
    hashmap.insert(And, Shift(176));
    table.push(hashmap);

    // for state 97
    let mut hashmap = HashMap::new();
    hashmap.insert(Star, Reduce(43));
    hashmap.insert(Rparen, Reduce(43));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(43));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(43));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(43));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(43));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(43));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(43));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(43));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(43));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(43));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(43));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(43));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(43));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(43));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(43));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(43));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(43));
    hashmap.insert(And, Reduce(43));
    table.push(hashmap);

    // for state 98
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(104));
    hashmap.insert(Star, Shift(99));
    hashmap.insert(Identifier, Shift(106));
    hashmap.insert(Lparen, Shift(107));
    hashmap.insert(StringLit, Shift(105));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(101));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(101));
    hashmap.insert(And, Shift(100));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(98));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(98));
    hashmap.insert(OPERAND, Goto(103));
    hashmap.insert(PRIMARY_EXPR, Goto(102));
    hashmap.insert(UNARY_EXPR, Goto(177));
    table.push(hashmap);

    // for state 99
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(104));
    hashmap.insert(Star, Shift(99));
    hashmap.insert(Identifier, Shift(106));
    hashmap.insert(Lparen, Shift(107));
    hashmap.insert(StringLit, Shift(105));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(101));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(101));
    hashmap.insert(And, Shift(100));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(98));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(98));
    hashmap.insert(OPERAND, Goto(103));
    hashmap.insert(PRIMARY_EXPR, Goto(102));
    hashmap.insert(UNARY_EXPR, Goto(178));
    table.push(hashmap);

    // for state 100
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(104));
    hashmap.insert(Star, Shift(99));
    hashmap.insert(Identifier, Shift(106));
    hashmap.insert(Lparen, Shift(107));
    hashmap.insert(StringLit, Shift(105));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(101));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(101));
    hashmap.insert(And, Shift(100));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(98));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(98));
    hashmap.insert(OPERAND, Goto(103));
    hashmap.insert(PRIMARY_EXPR, Goto(102));
    hashmap.insert(UNARY_EXPR, Goto(179));
    table.push(hashmap);

    // for state 101
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(104));
    hashmap.insert(Star, Shift(99));
    hashmap.insert(Identifier, Shift(106));
    hashmap.insert(Lparen, Shift(107));
    hashmap.insert(StringLit, Shift(105));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(101));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(101));
    hashmap.insert(And, Shift(100));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(98));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(98));
    hashmap.insert(OPERAND, Goto(103));
    hashmap.insert(PRIMARY_EXPR, Goto(102));
    hashmap.insert(UNARY_EXPR, Goto(180));
    table.push(hashmap);

    // for state 102
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Shift(183));
    hashmap.insert(Star, Reduce(48));
    hashmap.insert(Lparen, Shift(184));
    hashmap.insert(Rparen, Reduce(48));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(48));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(48));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(48));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(48));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(48));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(48));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(48));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(48));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(48));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(48));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(48));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(48));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(48));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(48));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(48));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(48));
    hashmap.insert(And, Reduce(48));
    hashmap.insert(INDEX, Goto(181));
    hashmap.insert(ARGUMENTS, Goto(182));
    table.push(hashmap);

    // for state 103
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(26));
    hashmap.insert(Star, Reduce(26));
    hashmap.insert(Lparen, Reduce(26));
    hashmap.insert(Rparen, Reduce(26));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(26));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(26));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(26));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(26));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(26));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(26));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(26));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(26));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(26));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(26));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(26));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(26));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(26));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(26));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(26));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(26));
    hashmap.insert(And, Reduce(26));
    table.push(hashmap);

    // for state 104
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(20));
    hashmap.insert(Star, Reduce(20));
    hashmap.insert(Lparen, Reduce(20));
    hashmap.insert(Rparen, Reduce(20));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(20));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(20));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(20));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(20));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(20));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(20));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(20));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(20));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(20));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(20));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(20));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(20));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(20));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(20));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(20));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(20));
    hashmap.insert(And, Reduce(20));
    table.push(hashmap);

    // for state 105
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(21));
    hashmap.insert(Star, Reduce(21));
    hashmap.insert(Lparen, Reduce(21));
    hashmap.insert(Rparen, Reduce(21));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(21));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(21));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(21));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(21));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(21));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(21));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(21));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(21));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(21));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(21));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(21));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(21));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(21));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(21));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(21));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(21));
    hashmap.insert(And, Reduce(21));
    table.push(hashmap);

    // for state 106
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(22));
    hashmap.insert(Star, Reduce(22));
    hashmap.insert(Lparen, Reduce(22));
    hashmap.insert(Rparen, Reduce(22));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(22));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(22));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(22));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(22));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(22));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(22));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(22));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(22));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(22));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(22));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(22));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(22));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(22));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(22));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(22));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(22));
    hashmap.insert(And, Reduce(22));
    table.push(hashmap);

    // for state 107
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(104));
    hashmap.insert(Star, Shift(99));
    hashmap.insert(Identifier, Shift(106));
    hashmap.insert(Lparen, Shift(107));
    hashmap.insert(StringLit, Shift(105));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(101));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(101));
    hashmap.insert(And, Shift(100));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(98));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(98));
    hashmap.insert(OPERAND, Goto(103));
    hashmap.insert(PRIMARY_EXPR, Goto(102));
    hashmap.insert(EXPRESSION, Goto(185));
    hashmap.insert(LOGICAL_EXPR, Goto(93));
    hashmap.insert(RELATIONAL_EXPR, Goto(94));
    hashmap.insert(ADDITIVE_EXPR, Goto(95));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(96));
    hashmap.insert(UNARY_EXPR, Goto(97));
    table.push(hashmap);

    // for state 108
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Reduce(16));
    table.push(hashmap);

    // for state 109
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(10));
    table.push(hashmap);

    // for state 110
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(57));
    hashmap.insert(Semicolon, Reduce(57));
    table.push(hashmap);

    // for state 111
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(71));
    hashmap.insert(Star, Shift(66));
    hashmap.insert(Identifier, Shift(73));
    hashmap.insert(Lparen, Shift(74));
    hashmap.insert(StringLit, Shift(72));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(68));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(68));
    hashmap.insert(And, Shift(67));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(65));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(65));
    hashmap.insert(OPERAND, Goto(70));
    hashmap.insert(PRIMARY_EXPR, Goto(69));
    hashmap.insert(RELATIONAL_EXPR, Goto(186));
    hashmap.insert(ADDITIVE_EXPR, Goto(62));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(63));
    hashmap.insert(UNARY_EXPR, Goto(64));
    table.push(hashmap);

    // for state 112
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(71));
    hashmap.insert(Star, Shift(66));
    hashmap.insert(Identifier, Shift(73));
    hashmap.insert(Lparen, Shift(74));
    hashmap.insert(StringLit, Shift(72));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(68));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(68));
    hashmap.insert(And, Shift(67));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(65));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(65));
    hashmap.insert(OPERAND, Goto(70));
    hashmap.insert(PRIMARY_EXPR, Goto(69));
    hashmap.insert(ADDITIVE_EXPR, Goto(187));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(63));
    hashmap.insert(UNARY_EXPR, Goto(64));
    table.push(hashmap);

    // for state 113
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(71));
    hashmap.insert(Star, Shift(66));
    hashmap.insert(Identifier, Shift(73));
    hashmap.insert(Lparen, Shift(74));
    hashmap.insert(StringLit, Shift(72));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(68));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(68));
    hashmap.insert(And, Shift(67));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(65));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(65));
    hashmap.insert(OPERAND, Goto(70));
    hashmap.insert(PRIMARY_EXPR, Goto(69));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(188));
    hashmap.insert(UNARY_EXPR, Goto(64));
    table.push(hashmap);

    // for state 114
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(71));
    hashmap.insert(Star, Shift(66));
    hashmap.insert(Identifier, Shift(73));
    hashmap.insert(Lparen, Shift(74));
    hashmap.insert(StringLit, Shift(72));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(68));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(68));
    hashmap.insert(And, Shift(67));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(65));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(65));
    hashmap.insert(OPERAND, Goto(70));
    hashmap.insert(PRIMARY_EXPR, Goto(69));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(189));
    hashmap.insert(UNARY_EXPR, Goto(64));
    table.push(hashmap);

    // for state 115
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(71));
    hashmap.insert(Star, Shift(66));
    hashmap.insert(Identifier, Shift(73));
    hashmap.insert(Lparen, Shift(74));
    hashmap.insert(StringLit, Shift(72));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(68));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(68));
    hashmap.insert(And, Shift(67));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(65));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(65));
    hashmap.insert(OPERAND, Goto(70));
    hashmap.insert(PRIMARY_EXPR, Goto(69));
    hashmap.insert(UNARY_EXPR, Goto(190));
    table.push(hashmap);

    // for state 116
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(71));
    hashmap.insert(Star, Shift(66));
    hashmap.insert(Identifier, Shift(73));
    hashmap.insert(Lparen, Shift(74));
    hashmap.insert(StringLit, Shift(72));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(68));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(68));
    hashmap.insert(And, Shift(67));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(65));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(65));
    hashmap.insert(OPERAND, Goto(70));
    hashmap.insert(PRIMARY_EXPR, Goto(69));
    hashmap.insert(UNARY_EXPR, Goto(191));
    table.push(hashmap);

    // for state 117
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(71));
    hashmap.insert(Star, Shift(66));
    hashmap.insert(Identifier, Shift(73));
    hashmap.insert(Lparen, Shift(74));
    hashmap.insert(StringLit, Shift(72));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(68));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(68));
    hashmap.insert(And, Shift(67));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(65));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(65));
    hashmap.insert(OPERAND, Goto(70));
    hashmap.insert(PRIMARY_EXPR, Goto(69));
    hashmap.insert(UNARY_EXPR, Goto(192));
    table.push(hashmap);

    // for state 118
    let mut hashmap = HashMap::new();
    hashmap.insert(Star, Reduce(44));
    hashmap.insert(Rbrace, Reduce(44));
    hashmap.insert(Semicolon, Reduce(44));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(44));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(44));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(44));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(44));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(44));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(44));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(44));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(44));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(44));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(44));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(44));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(44));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(44));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(44));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(44));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(44));
    hashmap.insert(And, Reduce(44));
    table.push(hashmap);

    // for state 119
    let mut hashmap = HashMap::new();
    hashmap.insert(Star, Reduce(45));
    hashmap.insert(Rbrace, Reduce(45));
    hashmap.insert(Semicolon, Reduce(45));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(45));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(45));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(45));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(45));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(45));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(45));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(45));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(45));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(45));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(45));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(45));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(45));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(45));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(45));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(45));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(45));
    hashmap.insert(And, Reduce(45));
    table.push(hashmap);

    // for state 120
    let mut hashmap = HashMap::new();
    hashmap.insert(Star, Reduce(46));
    hashmap.insert(Rbrace, Reduce(46));
    hashmap.insert(Semicolon, Reduce(46));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(46));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(46));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(46));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(46));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(46));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(46));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(46));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(46));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(46));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(46));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(46));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(46));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(46));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(46));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(46));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(46));
    hashmap.insert(And, Reduce(46));
    table.push(hashmap);

    // for state 121
    let mut hashmap = HashMap::new();
    hashmap.insert(Star, Reduce(47));
    hashmap.insert(Rbrace, Reduce(47));
    hashmap.insert(Semicolon, Reduce(47));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(47));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(47));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(47));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(47));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(47));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(47));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(47));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(47));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(47));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(47));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(47));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(47));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(47));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(47));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(47));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(47));
    hashmap.insert(And, Reduce(47));
    table.push(hashmap);

    // for state 122
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(24));
    hashmap.insert(Star, Reduce(24));
    hashmap.insert(Rbrace, Reduce(24));
    hashmap.insert(Semicolon, Reduce(24));
    hashmap.insert(Lparen, Reduce(24));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(24));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(24));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(24));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(24));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(24));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(24));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(24));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(24));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(24));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(24));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(24));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(24));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(24));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(24));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(24));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(24));
    hashmap.insert(And, Reduce(24));
    table.push(hashmap);

    // for state 123
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(25));
    hashmap.insert(Star, Reduce(25));
    hashmap.insert(Rbrace, Reduce(25));
    hashmap.insert(Semicolon, Reduce(25));
    hashmap.insert(Lparen, Reduce(25));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(25));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(25));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(25));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(25));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(25));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(25));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(25));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(25));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(25));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(25));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(25));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(25));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(25));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(25));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(25));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(25));
    hashmap.insert(And, Reduce(25));
    table.push(hashmap);

    // for state 124
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(148));
    hashmap.insert(Star, Shift(143));
    hashmap.insert(Identifier, Shift(150));
    hashmap.insert(Lparen, Shift(151));
    hashmap.insert(StringLit, Shift(149));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(145));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(145));
    hashmap.insert(And, Shift(144));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(142));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(142));
    hashmap.insert(OPERAND, Goto(147));
    hashmap.insert(PRIMARY_EXPR, Goto(146));
    hashmap.insert(EXPRESSION, Goto(193));
    hashmap.insert(LOGICAL_EXPR, Goto(137));
    hashmap.insert(RELATIONAL_EXPR, Goto(138));
    hashmap.insert(ADDITIVE_EXPR, Goto(139));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(140));
    hashmap.insert(UNARY_EXPR, Goto(141));
    table.push(hashmap);

    // for state 125
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(165));
    hashmap.insert(Star, Shift(160));
    hashmap.insert(Identifier, Shift(167));
    hashmap.insert(Lparen, Shift(168));
    hashmap.insert(Rparen, Reduce(31));
    hashmap.insert(StringLit, Shift(166));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(162));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(162));
    hashmap.insert(And, Shift(161));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(159));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(159));
    hashmap.insert(OPERAND, Goto(164));
    hashmap.insert(PRIMARY_EXPR, Goto(163));
    hashmap.insert(EXPRESSION_LIST, Goto(194));
    hashmap.insert(EXPRESSION, Goto(153));
    hashmap.insert(LOGICAL_EXPR, Goto(154));
    hashmap.insert(RELATIONAL_EXPR, Goto(155));
    hashmap.insert(ADDITIVE_EXPR, Goto(156));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(157));
    hashmap.insert(UNARY_EXPR, Goto(158));
    table.push(hashmap);

    // for state 126
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Shift(195));
    table.push(hashmap);

    // for state 127
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Shift(196));
    table.push(hashmap);

    // for state 128
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Shift(197));
    table.push(hashmap);

    // for state 129
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(33));
    hashmap.insert(Semicolon, Reduce(33));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(33));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(33));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Shift(78));
    hashmap.insert(RelOp(RelativeOperator::Greater), Shift(78));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Shift(78));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Shift(78));
    hashmap.insert(RelOp(RelativeOperator::Equal), Shift(78));
    hashmap.insert(RelOp(RelativeOperator::Less), Shift(78));
    hashmap.insert(AssignOp, Reduce(33));
    table.push(hashmap);

    // for state 130
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(35));
    hashmap.insert(Semicolon, Reduce(35));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(35));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(35));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(35));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(35));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(35));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(35));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(35));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(35));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Shift(79));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Shift(79));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(80));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(80));
    hashmap.insert(AssignOp, Reduce(35));
    table.push(hashmap);

    // for state 131
    let mut hashmap = HashMap::new();
    hashmap.insert(Star, Shift(82));
    hashmap.insert(Rbrace, Reduce(37));
    hashmap.insert(Semicolon, Reduce(37));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(37));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(37));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(37));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(37));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(37));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(37));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(37));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(37));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(37));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(37));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(37));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(37));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Shift(81));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Shift(81));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Shift(81));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Shift(81));
    hashmap.insert(And, Shift(83));
    hashmap.insert(AssignOp, Reduce(37));
    table.push(hashmap);

    // for state 132
    let mut hashmap = HashMap::new();
    hashmap.insert(Star, Shift(82));
    hashmap.insert(Rbrace, Reduce(38));
    hashmap.insert(Semicolon, Reduce(38));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(38));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(38));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(38));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(38));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(38));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(38));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(38));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(38));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(38));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(38));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(38));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(38));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Shift(81));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Shift(81));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Shift(81));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Shift(81));
    hashmap.insert(And, Shift(83));
    hashmap.insert(AssignOp, Reduce(38));
    table.push(hashmap);

    // for state 133
    let mut hashmap = HashMap::new();
    hashmap.insert(Star, Reduce(40));
    hashmap.insert(Rbrace, Reduce(40));
    hashmap.insert(Semicolon, Reduce(40));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(40));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(40));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(40));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(40));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(40));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(40));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(40));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(40));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(40));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(40));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(40));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(40));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(40));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(40));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(40));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(40));
    hashmap.insert(And, Reduce(40));
    hashmap.insert(AssignOp, Reduce(40));
    table.push(hashmap);

    // for state 134
    let mut hashmap = HashMap::new();
    hashmap.insert(Star, Reduce(41));
    hashmap.insert(Rbrace, Reduce(41));
    hashmap.insert(Semicolon, Reduce(41));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(41));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(41));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(41));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(41));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(41));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(41));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(41));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(41));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(41));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(41));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(41));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(41));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(41));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(41));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(41));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(41));
    hashmap.insert(And, Reduce(41));
    hashmap.insert(AssignOp, Reduce(41));
    table.push(hashmap);

    // for state 135
    let mut hashmap = HashMap::new();
    hashmap.insert(Star, Reduce(42));
    hashmap.insert(Rbrace, Reduce(42));
    hashmap.insert(Semicolon, Reduce(42));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(42));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(42));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(42));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(42));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(42));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(42));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(42));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(42));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(42));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(42));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(42));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(42));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(42));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(42));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(42));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(42));
    hashmap.insert(And, Reduce(42));
    hashmap.insert(AssignOp, Reduce(42));
    table.push(hashmap);

    // for state 136
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbracket, Shift(198));
    table.push(hashmap);

    // for state 137
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbracket, Reduce(32));
    hashmap.insert(LogOp(LogicalOperator::Or), Shift(199));
    hashmap.insert(LogOp(LogicalOperator::And), Shift(199));
    table.push(hashmap);

    // for state 138
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbracket, Reduce(34));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(34));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(34));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Shift(200));
    hashmap.insert(RelOp(RelativeOperator::Greater), Shift(200));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Shift(200));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Shift(200));
    hashmap.insert(RelOp(RelativeOperator::Equal), Shift(200));
    hashmap.insert(RelOp(RelativeOperator::Less), Shift(200));
    table.push(hashmap);

    // for state 139
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbracket, Reduce(36));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(36));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(36));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(36));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(36));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(36));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(36));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(36));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(36));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Shift(201));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Shift(201));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(202));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(202));
    table.push(hashmap);

    // for state 140
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbracket, Reduce(39));
    hashmap.insert(Star, Shift(204));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(39));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(39));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(39));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(39));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(39));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(39));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(39));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(39));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(39));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(39));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(39));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(39));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Shift(203));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Shift(203));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Shift(203));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Shift(203));
    hashmap.insert(And, Shift(205));
    table.push(hashmap);

    // for state 141
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbracket, Reduce(43));
    hashmap.insert(Star, Reduce(43));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(43));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(43));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(43));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(43));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(43));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(43));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(43));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(43));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(43));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(43));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(43));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(43));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(43));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(43));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(43));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(43));
    hashmap.insert(And, Reduce(43));
    table.push(hashmap);

    // for state 142
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(148));
    hashmap.insert(Star, Shift(143));
    hashmap.insert(Identifier, Shift(150));
    hashmap.insert(Lparen, Shift(151));
    hashmap.insert(StringLit, Shift(149));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(145));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(145));
    hashmap.insert(And, Shift(144));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(142));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(142));
    hashmap.insert(OPERAND, Goto(147));
    hashmap.insert(PRIMARY_EXPR, Goto(146));
    hashmap.insert(UNARY_EXPR, Goto(206));
    table.push(hashmap);

    // for state 143
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(148));
    hashmap.insert(Star, Shift(143));
    hashmap.insert(Identifier, Shift(150));
    hashmap.insert(Lparen, Shift(151));
    hashmap.insert(StringLit, Shift(149));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(145));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(145));
    hashmap.insert(And, Shift(144));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(142));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(142));
    hashmap.insert(OPERAND, Goto(147));
    hashmap.insert(PRIMARY_EXPR, Goto(146));
    hashmap.insert(UNARY_EXPR, Goto(207));
    table.push(hashmap);

    // for state 144
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(148));
    hashmap.insert(Star, Shift(143));
    hashmap.insert(Identifier, Shift(150));
    hashmap.insert(Lparen, Shift(151));
    hashmap.insert(StringLit, Shift(149));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(145));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(145));
    hashmap.insert(And, Shift(144));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(142));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(142));
    hashmap.insert(OPERAND, Goto(147));
    hashmap.insert(PRIMARY_EXPR, Goto(146));
    hashmap.insert(UNARY_EXPR, Goto(208));
    table.push(hashmap);

    // for state 145
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(148));
    hashmap.insert(Star, Shift(143));
    hashmap.insert(Identifier, Shift(150));
    hashmap.insert(Lparen, Shift(151));
    hashmap.insert(StringLit, Shift(149));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(145));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(145));
    hashmap.insert(And, Shift(144));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(142));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(142));
    hashmap.insert(OPERAND, Goto(147));
    hashmap.insert(PRIMARY_EXPR, Goto(146));
    hashmap.insert(UNARY_EXPR, Goto(209));
    table.push(hashmap);

    // for state 146
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Shift(212));
    hashmap.insert(Rbracket, Reduce(48));
    hashmap.insert(Star, Reduce(48));
    hashmap.insert(Lparen, Shift(213));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(48));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(48));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(48));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(48));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(48));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(48));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(48));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(48));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(48));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(48));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(48));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(48));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(48));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(48));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(48));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(48));
    hashmap.insert(And, Reduce(48));
    hashmap.insert(INDEX, Goto(210));
    hashmap.insert(ARGUMENTS, Goto(211));
    table.push(hashmap);

    // for state 147
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(26));
    hashmap.insert(Rbracket, Reduce(26));
    hashmap.insert(Star, Reduce(26));
    hashmap.insert(Lparen, Reduce(26));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(26));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(26));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(26));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(26));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(26));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(26));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(26));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(26));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(26));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(26));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(26));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(26));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(26));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(26));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(26));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(26));
    hashmap.insert(And, Reduce(26));
    table.push(hashmap);

    // for state 148
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(20));
    hashmap.insert(Rbracket, Reduce(20));
    hashmap.insert(Star, Reduce(20));
    hashmap.insert(Lparen, Reduce(20));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(20));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(20));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(20));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(20));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(20));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(20));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(20));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(20));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(20));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(20));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(20));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(20));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(20));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(20));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(20));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(20));
    hashmap.insert(And, Reduce(20));
    table.push(hashmap);

    // for state 149
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(21));
    hashmap.insert(Rbracket, Reduce(21));
    hashmap.insert(Star, Reduce(21));
    hashmap.insert(Lparen, Reduce(21));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(21));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(21));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(21));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(21));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(21));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(21));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(21));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(21));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(21));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(21));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(21));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(21));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(21));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(21));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(21));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(21));
    hashmap.insert(And, Reduce(21));
    table.push(hashmap);

    // for state 150
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(22));
    hashmap.insert(Rbracket, Reduce(22));
    hashmap.insert(Star, Reduce(22));
    hashmap.insert(Lparen, Reduce(22));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(22));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(22));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(22));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(22));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(22));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(22));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(22));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(22));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(22));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(22));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(22));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(22));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(22));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(22));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(22));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(22));
    hashmap.insert(And, Reduce(22));
    table.push(hashmap);

    // for state 151
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(104));
    hashmap.insert(Star, Shift(99));
    hashmap.insert(Identifier, Shift(106));
    hashmap.insert(Lparen, Shift(107));
    hashmap.insert(StringLit, Shift(105));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(101));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(101));
    hashmap.insert(And, Shift(100));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(98));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(98));
    hashmap.insert(OPERAND, Goto(103));
    hashmap.insert(PRIMARY_EXPR, Goto(102));
    hashmap.insert(EXPRESSION, Goto(214));
    hashmap.insert(LOGICAL_EXPR, Goto(93));
    hashmap.insert(RELATIONAL_EXPR, Goto(94));
    hashmap.insert(ADDITIVE_EXPR, Goto(95));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(96));
    hashmap.insert(UNARY_EXPR, Goto(97));
    table.push(hashmap);

    // for state 152
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Shift(215));
    table.push(hashmap);

    // for state 153
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Reduce(30));
    hashmap.insert(Comma, Shift(216));
    table.push(hashmap);

    // for state 154
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Reduce(32));
    hashmap.insert(Comma, Reduce(32));
    hashmap.insert(LogOp(LogicalOperator::Or), Shift(217));
    hashmap.insert(LogOp(LogicalOperator::And), Shift(217));
    table.push(hashmap);

    // for state 155
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Reduce(34));
    hashmap.insert(Comma, Reduce(34));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(34));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(34));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Shift(218));
    hashmap.insert(RelOp(RelativeOperator::Greater), Shift(218));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Shift(218));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Shift(218));
    hashmap.insert(RelOp(RelativeOperator::Equal), Shift(218));
    hashmap.insert(RelOp(RelativeOperator::Less), Shift(218));
    table.push(hashmap);

    // for state 156
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Reduce(36));
    hashmap.insert(Comma, Reduce(36));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(36));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(36));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(36));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(36));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(36));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(36));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(36));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(36));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Shift(219));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Shift(219));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(220));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(220));
    table.push(hashmap);

    // for state 157
    let mut hashmap = HashMap::new();
    hashmap.insert(Star, Shift(222));
    hashmap.insert(Rparen, Reduce(39));
    hashmap.insert(Comma, Reduce(39));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(39));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(39));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(39));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(39));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(39));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(39));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(39));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(39));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(39));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(39));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(39));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(39));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Shift(221));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Shift(221));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Shift(221));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Shift(221));
    hashmap.insert(And, Shift(223));
    table.push(hashmap);

    // for state 158
    let mut hashmap = HashMap::new();
    hashmap.insert(Star, Reduce(43));
    hashmap.insert(Rparen, Reduce(43));
    hashmap.insert(Comma, Reduce(43));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(43));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(43));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(43));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(43));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(43));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(43));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(43));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(43));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(43));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(43));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(43));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(43));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(43));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(43));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(43));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(43));
    hashmap.insert(And, Reduce(43));
    table.push(hashmap);

    // for state 159
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(165));
    hashmap.insert(Star, Shift(160));
    hashmap.insert(Identifier, Shift(167));
    hashmap.insert(Lparen, Shift(168));
    hashmap.insert(StringLit, Shift(166));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(162));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(162));
    hashmap.insert(And, Shift(161));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(159));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(159));
    hashmap.insert(OPERAND, Goto(164));
    hashmap.insert(PRIMARY_EXPR, Goto(163));
    hashmap.insert(UNARY_EXPR, Goto(224));
    table.push(hashmap);

    // for state 160
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(165));
    hashmap.insert(Star, Shift(160));
    hashmap.insert(Identifier, Shift(167));
    hashmap.insert(Lparen, Shift(168));
    hashmap.insert(StringLit, Shift(166));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(162));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(162));
    hashmap.insert(And, Shift(161));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(159));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(159));
    hashmap.insert(OPERAND, Goto(164));
    hashmap.insert(PRIMARY_EXPR, Goto(163));
    hashmap.insert(UNARY_EXPR, Goto(225));
    table.push(hashmap);

    // for state 161
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(165));
    hashmap.insert(Star, Shift(160));
    hashmap.insert(Identifier, Shift(167));
    hashmap.insert(Lparen, Shift(168));
    hashmap.insert(StringLit, Shift(166));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(162));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(162));
    hashmap.insert(And, Shift(161));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(159));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(159));
    hashmap.insert(OPERAND, Goto(164));
    hashmap.insert(PRIMARY_EXPR, Goto(163));
    hashmap.insert(UNARY_EXPR, Goto(226));
    table.push(hashmap);

    // for state 162
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(165));
    hashmap.insert(Star, Shift(160));
    hashmap.insert(Identifier, Shift(167));
    hashmap.insert(Lparen, Shift(168));
    hashmap.insert(StringLit, Shift(166));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(162));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(162));
    hashmap.insert(And, Shift(161));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(159));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(159));
    hashmap.insert(OPERAND, Goto(164));
    hashmap.insert(PRIMARY_EXPR, Goto(163));
    hashmap.insert(UNARY_EXPR, Goto(227));
    table.push(hashmap);

    // for state 163
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Shift(230));
    hashmap.insert(Star, Reduce(48));
    hashmap.insert(Lparen, Shift(231));
    hashmap.insert(Rparen, Reduce(48));
    hashmap.insert(Comma, Reduce(48));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(48));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(48));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(48));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(48));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(48));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(48));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(48));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(48));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(48));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(48));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(48));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(48));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(48));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(48));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(48));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(48));
    hashmap.insert(And, Reduce(48));
    hashmap.insert(INDEX, Goto(228));
    hashmap.insert(ARGUMENTS, Goto(229));
    table.push(hashmap);

    // for state 164
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(26));
    hashmap.insert(Star, Reduce(26));
    hashmap.insert(Lparen, Reduce(26));
    hashmap.insert(Rparen, Reduce(26));
    hashmap.insert(Comma, Reduce(26));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(26));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(26));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(26));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(26));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(26));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(26));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(26));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(26));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(26));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(26));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(26));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(26));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(26));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(26));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(26));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(26));
    hashmap.insert(And, Reduce(26));
    table.push(hashmap);

    // for state 165
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(20));
    hashmap.insert(Star, Reduce(20));
    hashmap.insert(Lparen, Reduce(20));
    hashmap.insert(Rparen, Reduce(20));
    hashmap.insert(Comma, Reduce(20));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(20));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(20));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(20));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(20));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(20));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(20));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(20));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(20));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(20));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(20));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(20));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(20));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(20));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(20));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(20));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(20));
    hashmap.insert(And, Reduce(20));
    table.push(hashmap);

    // for state 166
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(21));
    hashmap.insert(Star, Reduce(21));
    hashmap.insert(Lparen, Reduce(21));
    hashmap.insert(Rparen, Reduce(21));
    hashmap.insert(Comma, Reduce(21));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(21));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(21));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(21));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(21));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(21));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(21));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(21));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(21));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(21));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(21));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(21));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(21));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(21));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(21));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(21));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(21));
    hashmap.insert(And, Reduce(21));
    table.push(hashmap);

    // for state 167
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(22));
    hashmap.insert(Star, Reduce(22));
    hashmap.insert(Lparen, Reduce(22));
    hashmap.insert(Rparen, Reduce(22));
    hashmap.insert(Comma, Reduce(22));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(22));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(22));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(22));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(22));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(22));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(22));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(22));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(22));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(22));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(22));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(22));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(22));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(22));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(22));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(22));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(22));
    hashmap.insert(And, Reduce(22));
    table.push(hashmap);

    // for state 168
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(104));
    hashmap.insert(Star, Shift(99));
    hashmap.insert(Identifier, Shift(106));
    hashmap.insert(Lparen, Shift(107));
    hashmap.insert(StringLit, Shift(105));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(101));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(101));
    hashmap.insert(And, Shift(100));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(98));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(98));
    hashmap.insert(OPERAND, Goto(103));
    hashmap.insert(PRIMARY_EXPR, Goto(102));
    hashmap.insert(EXPRESSION, Goto(232));
    hashmap.insert(LOGICAL_EXPR, Goto(93));
    hashmap.insert(RELATIONAL_EXPR, Goto(94));
    hashmap.insert(ADDITIVE_EXPR, Goto(95));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(96));
    hashmap.insert(UNARY_EXPR, Goto(97));
    table.push(hashmap);

    // for state 169
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(23));
    hashmap.insert(Star, Reduce(23));
    hashmap.insert(Rbrace, Reduce(23));
    hashmap.insert(Semicolon, Reduce(23));
    hashmap.insert(Lparen, Reduce(23));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(23));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(23));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(23));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(23));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(23));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(23));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(23));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(23));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(23));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(23));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(23));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(23));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(23));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(23));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(23));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(23));
    hashmap.insert(And, Reduce(23));
    hashmap.insert(AssignOp, Reduce(23));
    table.push(hashmap);

    // for state 170
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(104));
    hashmap.insert(Star, Shift(99));
    hashmap.insert(Identifier, Shift(106));
    hashmap.insert(Lparen, Shift(107));
    hashmap.insert(StringLit, Shift(105));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(101));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(101));
    hashmap.insert(And, Shift(100));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(98));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(98));
    hashmap.insert(OPERAND, Goto(103));
    hashmap.insert(PRIMARY_EXPR, Goto(102));
    hashmap.insert(RELATIONAL_EXPR, Goto(233));
    hashmap.insert(ADDITIVE_EXPR, Goto(95));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(96));
    hashmap.insert(UNARY_EXPR, Goto(97));
    table.push(hashmap);

    // for state 171
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(104));
    hashmap.insert(Star, Shift(99));
    hashmap.insert(Identifier, Shift(106));
    hashmap.insert(Lparen, Shift(107));
    hashmap.insert(StringLit, Shift(105));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(101));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(101));
    hashmap.insert(And, Shift(100));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(98));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(98));
    hashmap.insert(OPERAND, Goto(103));
    hashmap.insert(PRIMARY_EXPR, Goto(102));
    hashmap.insert(ADDITIVE_EXPR, Goto(234));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(96));
    hashmap.insert(UNARY_EXPR, Goto(97));
    table.push(hashmap);

    // for state 172
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(104));
    hashmap.insert(Star, Shift(99));
    hashmap.insert(Identifier, Shift(106));
    hashmap.insert(Lparen, Shift(107));
    hashmap.insert(StringLit, Shift(105));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(101));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(101));
    hashmap.insert(And, Shift(100));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(98));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(98));
    hashmap.insert(OPERAND, Goto(103));
    hashmap.insert(PRIMARY_EXPR, Goto(102));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(235));
    hashmap.insert(UNARY_EXPR, Goto(97));
    table.push(hashmap);

    // for state 173
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(104));
    hashmap.insert(Star, Shift(99));
    hashmap.insert(Identifier, Shift(106));
    hashmap.insert(Lparen, Shift(107));
    hashmap.insert(StringLit, Shift(105));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(101));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(101));
    hashmap.insert(And, Shift(100));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(98));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(98));
    hashmap.insert(OPERAND, Goto(103));
    hashmap.insert(PRIMARY_EXPR, Goto(102));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(236));
    hashmap.insert(UNARY_EXPR, Goto(97));
    table.push(hashmap);

    // for state 174
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(104));
    hashmap.insert(Star, Shift(99));
    hashmap.insert(Identifier, Shift(106));
    hashmap.insert(Lparen, Shift(107));
    hashmap.insert(StringLit, Shift(105));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(101));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(101));
    hashmap.insert(And, Shift(100));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(98));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(98));
    hashmap.insert(OPERAND, Goto(103));
    hashmap.insert(PRIMARY_EXPR, Goto(102));
    hashmap.insert(UNARY_EXPR, Goto(237));
    table.push(hashmap);

    // for state 175
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(104));
    hashmap.insert(Star, Shift(99));
    hashmap.insert(Identifier, Shift(106));
    hashmap.insert(Lparen, Shift(107));
    hashmap.insert(StringLit, Shift(105));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(101));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(101));
    hashmap.insert(And, Shift(100));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(98));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(98));
    hashmap.insert(OPERAND, Goto(103));
    hashmap.insert(PRIMARY_EXPR, Goto(102));
    hashmap.insert(UNARY_EXPR, Goto(238));
    table.push(hashmap);

    // for state 176
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(104));
    hashmap.insert(Star, Shift(99));
    hashmap.insert(Identifier, Shift(106));
    hashmap.insert(Lparen, Shift(107));
    hashmap.insert(StringLit, Shift(105));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(101));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(101));
    hashmap.insert(And, Shift(100));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(98));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(98));
    hashmap.insert(OPERAND, Goto(103));
    hashmap.insert(PRIMARY_EXPR, Goto(102));
    hashmap.insert(UNARY_EXPR, Goto(239));
    table.push(hashmap);

    // for state 177
    let mut hashmap = HashMap::new();
    hashmap.insert(Star, Reduce(44));
    hashmap.insert(Rparen, Reduce(44));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(44));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(44));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(44));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(44));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(44));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(44));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(44));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(44));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(44));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(44));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(44));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(44));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(44));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(44));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(44));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(44));
    hashmap.insert(And, Reduce(44));
    table.push(hashmap);

    // for state 178
    let mut hashmap = HashMap::new();
    hashmap.insert(Star, Reduce(45));
    hashmap.insert(Rparen, Reduce(45));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(45));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(45));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(45));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(45));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(45));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(45));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(45));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(45));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(45));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(45));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(45));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(45));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(45));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(45));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(45));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(45));
    hashmap.insert(And, Reduce(45));
    table.push(hashmap);

    // for state 179
    let mut hashmap = HashMap::new();
    hashmap.insert(Star, Reduce(46));
    hashmap.insert(Rparen, Reduce(46));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(46));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(46));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(46));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(46));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(46));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(46));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(46));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(46));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(46));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(46));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(46));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(46));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(46));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(46));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(46));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(46));
    hashmap.insert(And, Reduce(46));
    table.push(hashmap);

    // for state 180
    let mut hashmap = HashMap::new();
    hashmap.insert(Star, Reduce(47));
    hashmap.insert(Rparen, Reduce(47));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(47));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(47));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(47));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(47));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(47));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(47));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(47));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(47));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(47));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(47));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(47));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(47));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(47));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(47));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(47));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(47));
    hashmap.insert(And, Reduce(47));
    table.push(hashmap);

    // for state 181
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(24));
    hashmap.insert(Star, Reduce(24));
    hashmap.insert(Lparen, Reduce(24));
    hashmap.insert(Rparen, Reduce(24));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(24));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(24));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(24));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(24));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(24));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(24));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(24));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(24));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(24));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(24));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(24));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(24));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(24));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(24));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(24));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(24));
    hashmap.insert(And, Reduce(24));
    table.push(hashmap);

    // for state 182
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(25));
    hashmap.insert(Star, Reduce(25));
    hashmap.insert(Lparen, Reduce(25));
    hashmap.insert(Rparen, Reduce(25));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(25));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(25));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(25));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(25));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(25));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(25));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(25));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(25));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(25));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(25));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(25));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(25));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(25));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(25));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(25));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(25));
    hashmap.insert(And, Reduce(25));
    table.push(hashmap);

    // for state 183
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(148));
    hashmap.insert(Star, Shift(143));
    hashmap.insert(Identifier, Shift(150));
    hashmap.insert(Lparen, Shift(151));
    hashmap.insert(StringLit, Shift(149));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(145));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(145));
    hashmap.insert(And, Shift(144));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(142));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(142));
    hashmap.insert(OPERAND, Goto(147));
    hashmap.insert(PRIMARY_EXPR, Goto(146));
    hashmap.insert(EXPRESSION, Goto(240));
    hashmap.insert(LOGICAL_EXPR, Goto(137));
    hashmap.insert(RELATIONAL_EXPR, Goto(138));
    hashmap.insert(ADDITIVE_EXPR, Goto(139));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(140));
    hashmap.insert(UNARY_EXPR, Goto(141));
    table.push(hashmap);

    // for state 184
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(165));
    hashmap.insert(Star, Shift(160));
    hashmap.insert(Identifier, Shift(167));
    hashmap.insert(Lparen, Shift(168));
    hashmap.insert(Rparen, Reduce(31));
    hashmap.insert(StringLit, Shift(166));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(162));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(162));
    hashmap.insert(And, Shift(161));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(159));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(159));
    hashmap.insert(OPERAND, Goto(164));
    hashmap.insert(PRIMARY_EXPR, Goto(163));
    hashmap.insert(EXPRESSION_LIST, Goto(241));
    hashmap.insert(EXPRESSION, Goto(153));
    hashmap.insert(LOGICAL_EXPR, Goto(154));
    hashmap.insert(RELATIONAL_EXPR, Goto(155));
    hashmap.insert(ADDITIVE_EXPR, Goto(156));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(157));
    hashmap.insert(UNARY_EXPR, Goto(158));
    table.push(hashmap);

    // for state 185
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Shift(242));
    table.push(hashmap);

    // for state 186
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(33));
    hashmap.insert(Semicolon, Reduce(33));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(33));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(33));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Shift(112));
    hashmap.insert(RelOp(RelativeOperator::Greater), Shift(112));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Shift(112));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Shift(112));
    hashmap.insert(RelOp(RelativeOperator::Equal), Shift(112));
    hashmap.insert(RelOp(RelativeOperator::Less), Shift(112));
    table.push(hashmap);

    // for state 187
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(35));
    hashmap.insert(Semicolon, Reduce(35));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(35));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(35));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(35));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(35));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(35));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(35));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(35));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(35));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Shift(113));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Shift(113));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(114));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(114));
    table.push(hashmap);

    // for state 188
    let mut hashmap = HashMap::new();
    hashmap.insert(Star, Shift(116));
    hashmap.insert(Rbrace, Reduce(37));
    hashmap.insert(Semicolon, Reduce(37));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(37));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(37));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(37));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(37));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(37));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(37));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(37));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(37));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(37));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(37));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(37));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(37));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Shift(115));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Shift(115));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Shift(115));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Shift(115));
    hashmap.insert(And, Shift(117));
    table.push(hashmap);

    // for state 189
    let mut hashmap = HashMap::new();
    hashmap.insert(Star, Shift(116));
    hashmap.insert(Rbrace, Reduce(38));
    hashmap.insert(Semicolon, Reduce(38));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(38));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(38));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(38));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(38));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(38));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(38));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(38));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(38));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(38));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(38));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(38));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(38));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Shift(115));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Shift(115));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Shift(115));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Shift(115));
    hashmap.insert(And, Shift(117));
    table.push(hashmap);

    // for state 190
    let mut hashmap = HashMap::new();
    hashmap.insert(Star, Reduce(40));
    hashmap.insert(Rbrace, Reduce(40));
    hashmap.insert(Semicolon, Reduce(40));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(40));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(40));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(40));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(40));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(40));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(40));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(40));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(40));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(40));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(40));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(40));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(40));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(40));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(40));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(40));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(40));
    hashmap.insert(And, Reduce(40));
    table.push(hashmap);

    // for state 191
    let mut hashmap = HashMap::new();
    hashmap.insert(Star, Reduce(41));
    hashmap.insert(Rbrace, Reduce(41));
    hashmap.insert(Semicolon, Reduce(41));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(41));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(41));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(41));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(41));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(41));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(41));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(41));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(41));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(41));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(41));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(41));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(41));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(41));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(41));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(41));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(41));
    hashmap.insert(And, Reduce(41));
    table.push(hashmap);

    // for state 192
    let mut hashmap = HashMap::new();
    hashmap.insert(Star, Reduce(42));
    hashmap.insert(Rbrace, Reduce(42));
    hashmap.insert(Semicolon, Reduce(42));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(42));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(42));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(42));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(42));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(42));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(42));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(42));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(42));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(42));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(42));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(42));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(42));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(42));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(42));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(42));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(42));
    hashmap.insert(And, Reduce(42));
    table.push(hashmap);

    // for state 193
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbracket, Shift(243));
    table.push(hashmap);

    // for state 194
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Shift(244));
    table.push(hashmap);

    // for state 195
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(23));
    hashmap.insert(Star, Reduce(23));
    hashmap.insert(Rbrace, Reduce(23));
    hashmap.insert(Semicolon, Reduce(23));
    hashmap.insert(Lparen, Reduce(23));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(23));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(23));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(23));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(23));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(23));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(23));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(23));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(23));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(23));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(23));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(23));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(23));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(23));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(23));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(23));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(23));
    hashmap.insert(And, Reduce(23));
    table.push(hashmap);

    // for state 196
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbrace, Shift(246));
    hashmap.insert(BLOCK, Goto(245));
    table.push(hashmap);

    // for state 197
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbrace, Shift(246));
    hashmap.insert(BLOCK, Goto(247));
    table.push(hashmap);

    // for state 198
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(27));
    hashmap.insert(Star, Reduce(27));
    hashmap.insert(Rbrace, Reduce(27));
    hashmap.insert(Semicolon, Reduce(27));
    hashmap.insert(Lparen, Reduce(27));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(27));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(27));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(27));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(27));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(27));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(27));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(27));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(27));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(27));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(27));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(27));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(27));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(27));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(27));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(27));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(27));
    hashmap.insert(And, Reduce(27));
    hashmap.insert(AssignOp, Reduce(27));
    table.push(hashmap);

    // for state 199
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(148));
    hashmap.insert(Star, Shift(143));
    hashmap.insert(Identifier, Shift(150));
    hashmap.insert(Lparen, Shift(151));
    hashmap.insert(StringLit, Shift(149));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(145));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(145));
    hashmap.insert(And, Shift(144));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(142));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(142));
    hashmap.insert(OPERAND, Goto(147));
    hashmap.insert(PRIMARY_EXPR, Goto(146));
    hashmap.insert(RELATIONAL_EXPR, Goto(248));
    hashmap.insert(ADDITIVE_EXPR, Goto(139));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(140));
    hashmap.insert(UNARY_EXPR, Goto(141));
    table.push(hashmap);

    // for state 200
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(148));
    hashmap.insert(Star, Shift(143));
    hashmap.insert(Identifier, Shift(150));
    hashmap.insert(Lparen, Shift(151));
    hashmap.insert(StringLit, Shift(149));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(145));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(145));
    hashmap.insert(And, Shift(144));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(142));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(142));
    hashmap.insert(OPERAND, Goto(147));
    hashmap.insert(PRIMARY_EXPR, Goto(146));
    hashmap.insert(ADDITIVE_EXPR, Goto(249));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(140));
    hashmap.insert(UNARY_EXPR, Goto(141));
    table.push(hashmap);

    // for state 201
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(148));
    hashmap.insert(Star, Shift(143));
    hashmap.insert(Identifier, Shift(150));
    hashmap.insert(Lparen, Shift(151));
    hashmap.insert(StringLit, Shift(149));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(145));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(145));
    hashmap.insert(And, Shift(144));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(142));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(142));
    hashmap.insert(OPERAND, Goto(147));
    hashmap.insert(PRIMARY_EXPR, Goto(146));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(250));
    hashmap.insert(UNARY_EXPR, Goto(141));
    table.push(hashmap);

    // for state 202
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(148));
    hashmap.insert(Star, Shift(143));
    hashmap.insert(Identifier, Shift(150));
    hashmap.insert(Lparen, Shift(151));
    hashmap.insert(StringLit, Shift(149));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(145));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(145));
    hashmap.insert(And, Shift(144));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(142));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(142));
    hashmap.insert(OPERAND, Goto(147));
    hashmap.insert(PRIMARY_EXPR, Goto(146));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(251));
    hashmap.insert(UNARY_EXPR, Goto(141));
    table.push(hashmap);

    // for state 203
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(148));
    hashmap.insert(Star, Shift(143));
    hashmap.insert(Identifier, Shift(150));
    hashmap.insert(Lparen, Shift(151));
    hashmap.insert(StringLit, Shift(149));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(145));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(145));
    hashmap.insert(And, Shift(144));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(142));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(142));
    hashmap.insert(OPERAND, Goto(147));
    hashmap.insert(PRIMARY_EXPR, Goto(146));
    hashmap.insert(UNARY_EXPR, Goto(252));
    table.push(hashmap);

    // for state 204
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(148));
    hashmap.insert(Star, Shift(143));
    hashmap.insert(Identifier, Shift(150));
    hashmap.insert(Lparen, Shift(151));
    hashmap.insert(StringLit, Shift(149));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(145));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(145));
    hashmap.insert(And, Shift(144));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(142));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(142));
    hashmap.insert(OPERAND, Goto(147));
    hashmap.insert(PRIMARY_EXPR, Goto(146));
    hashmap.insert(UNARY_EXPR, Goto(253));
    table.push(hashmap);

    // for state 205
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(148));
    hashmap.insert(Star, Shift(143));
    hashmap.insert(Identifier, Shift(150));
    hashmap.insert(Lparen, Shift(151));
    hashmap.insert(StringLit, Shift(149));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(145));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(145));
    hashmap.insert(And, Shift(144));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(142));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(142));
    hashmap.insert(OPERAND, Goto(147));
    hashmap.insert(PRIMARY_EXPR, Goto(146));
    hashmap.insert(UNARY_EXPR, Goto(254));
    table.push(hashmap);

    // for state 206
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbracket, Reduce(44));
    hashmap.insert(Star, Reduce(44));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(44));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(44));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(44));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(44));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(44));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(44));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(44));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(44));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(44));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(44));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(44));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(44));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(44));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(44));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(44));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(44));
    hashmap.insert(And, Reduce(44));
    table.push(hashmap);

    // for state 207
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbracket, Reduce(45));
    hashmap.insert(Star, Reduce(45));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(45));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(45));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(45));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(45));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(45));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(45));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(45));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(45));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(45));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(45));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(45));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(45));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(45));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(45));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(45));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(45));
    hashmap.insert(And, Reduce(45));
    table.push(hashmap);

    // for state 208
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbracket, Reduce(46));
    hashmap.insert(Star, Reduce(46));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(46));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(46));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(46));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(46));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(46));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(46));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(46));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(46));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(46));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(46));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(46));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(46));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(46));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(46));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(46));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(46));
    hashmap.insert(And, Reduce(46));
    table.push(hashmap);

    // for state 209
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbracket, Reduce(47));
    hashmap.insert(Star, Reduce(47));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(47));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(47));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(47));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(47));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(47));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(47));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(47));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(47));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(47));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(47));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(47));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(47));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(47));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(47));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(47));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(47));
    hashmap.insert(And, Reduce(47));
    table.push(hashmap);

    // for state 210
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(24));
    hashmap.insert(Rbracket, Reduce(24));
    hashmap.insert(Star, Reduce(24));
    hashmap.insert(Lparen, Reduce(24));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(24));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(24));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(24));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(24));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(24));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(24));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(24));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(24));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(24));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(24));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(24));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(24));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(24));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(24));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(24));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(24));
    hashmap.insert(And, Reduce(24));
    table.push(hashmap);

    // for state 211
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(25));
    hashmap.insert(Rbracket, Reduce(25));
    hashmap.insert(Star, Reduce(25));
    hashmap.insert(Lparen, Reduce(25));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(25));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(25));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(25));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(25));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(25));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(25));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(25));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(25));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(25));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(25));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(25));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(25));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(25));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(25));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(25));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(25));
    hashmap.insert(And, Reduce(25));
    table.push(hashmap);

    // for state 212
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(148));
    hashmap.insert(Star, Shift(143));
    hashmap.insert(Identifier, Shift(150));
    hashmap.insert(Lparen, Shift(151));
    hashmap.insert(StringLit, Shift(149));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(145));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(145));
    hashmap.insert(And, Shift(144));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(142));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(142));
    hashmap.insert(OPERAND, Goto(147));
    hashmap.insert(PRIMARY_EXPR, Goto(146));
    hashmap.insert(EXPRESSION, Goto(255));
    hashmap.insert(LOGICAL_EXPR, Goto(137));
    hashmap.insert(RELATIONAL_EXPR, Goto(138));
    hashmap.insert(ADDITIVE_EXPR, Goto(139));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(140));
    hashmap.insert(UNARY_EXPR, Goto(141));
    table.push(hashmap);

    // for state 213
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(165));
    hashmap.insert(Star, Shift(160));
    hashmap.insert(Identifier, Shift(167));
    hashmap.insert(Lparen, Shift(168));
    hashmap.insert(Rparen, Reduce(31));
    hashmap.insert(StringLit, Shift(166));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(162));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(162));
    hashmap.insert(And, Shift(161));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(159));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(159));
    hashmap.insert(OPERAND, Goto(164));
    hashmap.insert(PRIMARY_EXPR, Goto(163));
    hashmap.insert(EXPRESSION_LIST, Goto(256));
    hashmap.insert(EXPRESSION, Goto(153));
    hashmap.insert(LOGICAL_EXPR, Goto(154));
    hashmap.insert(RELATIONAL_EXPR, Goto(155));
    hashmap.insert(ADDITIVE_EXPR, Goto(156));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(157));
    hashmap.insert(UNARY_EXPR, Goto(158));
    table.push(hashmap);

    // for state 214
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Shift(257));
    table.push(hashmap);

    // for state 215
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(28));
    hashmap.insert(Star, Reduce(28));
    hashmap.insert(Rbrace, Reduce(28));
    hashmap.insert(Semicolon, Reduce(28));
    hashmap.insert(Lparen, Reduce(28));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(28));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(28));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(28));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(28));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(28));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(28));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(28));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(28));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(28));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(28));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(28));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(28));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(28));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(28));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(28));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(28));
    hashmap.insert(And, Reduce(28));
    hashmap.insert(AssignOp, Reduce(28));
    table.push(hashmap);

    // for state 216
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(165));
    hashmap.insert(Star, Shift(160));
    hashmap.insert(Identifier, Shift(167));
    hashmap.insert(Lparen, Shift(168));
    hashmap.insert(Rparen, Reduce(31));
    hashmap.insert(StringLit, Shift(166));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(162));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(162));
    hashmap.insert(And, Shift(161));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(159));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(159));
    hashmap.insert(OPERAND, Goto(164));
    hashmap.insert(PRIMARY_EXPR, Goto(163));
    hashmap.insert(EXPRESSION_LIST, Goto(258));
    hashmap.insert(EXPRESSION, Goto(153));
    hashmap.insert(LOGICAL_EXPR, Goto(154));
    hashmap.insert(RELATIONAL_EXPR, Goto(155));
    hashmap.insert(ADDITIVE_EXPR, Goto(156));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(157));
    hashmap.insert(UNARY_EXPR, Goto(158));
    table.push(hashmap);

    // for state 217
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(165));
    hashmap.insert(Star, Shift(160));
    hashmap.insert(Identifier, Shift(167));
    hashmap.insert(Lparen, Shift(168));
    hashmap.insert(StringLit, Shift(166));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(162));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(162));
    hashmap.insert(And, Shift(161));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(159));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(159));
    hashmap.insert(OPERAND, Goto(164));
    hashmap.insert(PRIMARY_EXPR, Goto(163));
    hashmap.insert(RELATIONAL_EXPR, Goto(259));
    hashmap.insert(ADDITIVE_EXPR, Goto(156));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(157));
    hashmap.insert(UNARY_EXPR, Goto(158));
    table.push(hashmap);

    // for state 218
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(165));
    hashmap.insert(Star, Shift(160));
    hashmap.insert(Identifier, Shift(167));
    hashmap.insert(Lparen, Shift(168));
    hashmap.insert(StringLit, Shift(166));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(162));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(162));
    hashmap.insert(And, Shift(161));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(159));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(159));
    hashmap.insert(OPERAND, Goto(164));
    hashmap.insert(PRIMARY_EXPR, Goto(163));
    hashmap.insert(ADDITIVE_EXPR, Goto(260));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(157));
    hashmap.insert(UNARY_EXPR, Goto(158));
    table.push(hashmap);

    // for state 219
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(165));
    hashmap.insert(Star, Shift(160));
    hashmap.insert(Identifier, Shift(167));
    hashmap.insert(Lparen, Shift(168));
    hashmap.insert(StringLit, Shift(166));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(162));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(162));
    hashmap.insert(And, Shift(161));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(159));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(159));
    hashmap.insert(OPERAND, Goto(164));
    hashmap.insert(PRIMARY_EXPR, Goto(163));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(261));
    hashmap.insert(UNARY_EXPR, Goto(158));
    table.push(hashmap);

    // for state 220
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(165));
    hashmap.insert(Star, Shift(160));
    hashmap.insert(Identifier, Shift(167));
    hashmap.insert(Lparen, Shift(168));
    hashmap.insert(StringLit, Shift(166));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(162));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(162));
    hashmap.insert(And, Shift(161));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(159));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(159));
    hashmap.insert(OPERAND, Goto(164));
    hashmap.insert(PRIMARY_EXPR, Goto(163));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(262));
    hashmap.insert(UNARY_EXPR, Goto(158));
    table.push(hashmap);

    // for state 221
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(165));
    hashmap.insert(Star, Shift(160));
    hashmap.insert(Identifier, Shift(167));
    hashmap.insert(Lparen, Shift(168));
    hashmap.insert(StringLit, Shift(166));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(162));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(162));
    hashmap.insert(And, Shift(161));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(159));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(159));
    hashmap.insert(OPERAND, Goto(164));
    hashmap.insert(PRIMARY_EXPR, Goto(163));
    hashmap.insert(UNARY_EXPR, Goto(263));
    table.push(hashmap);

    // for state 222
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(165));
    hashmap.insert(Star, Shift(160));
    hashmap.insert(Identifier, Shift(167));
    hashmap.insert(Lparen, Shift(168));
    hashmap.insert(StringLit, Shift(166));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(162));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(162));
    hashmap.insert(And, Shift(161));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(159));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(159));
    hashmap.insert(OPERAND, Goto(164));
    hashmap.insert(PRIMARY_EXPR, Goto(163));
    hashmap.insert(UNARY_EXPR, Goto(264));
    table.push(hashmap);

    // for state 223
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(165));
    hashmap.insert(Star, Shift(160));
    hashmap.insert(Identifier, Shift(167));
    hashmap.insert(Lparen, Shift(168));
    hashmap.insert(StringLit, Shift(166));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(162));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(162));
    hashmap.insert(And, Shift(161));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(159));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(159));
    hashmap.insert(OPERAND, Goto(164));
    hashmap.insert(PRIMARY_EXPR, Goto(163));
    hashmap.insert(UNARY_EXPR, Goto(265));
    table.push(hashmap);

    // for state 224
    let mut hashmap = HashMap::new();
    hashmap.insert(Star, Reduce(44));
    hashmap.insert(Rparen, Reduce(44));
    hashmap.insert(Comma, Reduce(44));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(44));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(44));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(44));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(44));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(44));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(44));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(44));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(44));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(44));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(44));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(44));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(44));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(44));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(44));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(44));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(44));
    hashmap.insert(And, Reduce(44));
    table.push(hashmap);

    // for state 225
    let mut hashmap = HashMap::new();
    hashmap.insert(Star, Reduce(45));
    hashmap.insert(Rparen, Reduce(45));
    hashmap.insert(Comma, Reduce(45));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(45));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(45));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(45));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(45));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(45));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(45));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(45));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(45));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(45));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(45));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(45));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(45));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(45));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(45));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(45));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(45));
    hashmap.insert(And, Reduce(45));
    table.push(hashmap);

    // for state 226
    let mut hashmap = HashMap::new();
    hashmap.insert(Star, Reduce(46));
    hashmap.insert(Rparen, Reduce(46));
    hashmap.insert(Comma, Reduce(46));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(46));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(46));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(46));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(46));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(46));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(46));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(46));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(46));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(46));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(46));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(46));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(46));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(46));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(46));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(46));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(46));
    hashmap.insert(And, Reduce(46));
    table.push(hashmap);

    // for state 227
    let mut hashmap = HashMap::new();
    hashmap.insert(Star, Reduce(47));
    hashmap.insert(Rparen, Reduce(47));
    hashmap.insert(Comma, Reduce(47));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(47));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(47));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(47));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(47));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(47));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(47));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(47));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(47));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(47));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(47));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(47));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(47));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(47));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(47));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(47));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(47));
    hashmap.insert(And, Reduce(47));
    table.push(hashmap);

    // for state 228
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(24));
    hashmap.insert(Star, Reduce(24));
    hashmap.insert(Lparen, Reduce(24));
    hashmap.insert(Rparen, Reduce(24));
    hashmap.insert(Comma, Reduce(24));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(24));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(24));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(24));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(24));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(24));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(24));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(24));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(24));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(24));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(24));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(24));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(24));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(24));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(24));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(24));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(24));
    hashmap.insert(And, Reduce(24));
    table.push(hashmap);

    // for state 229
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(25));
    hashmap.insert(Star, Reduce(25));
    hashmap.insert(Lparen, Reduce(25));
    hashmap.insert(Rparen, Reduce(25));
    hashmap.insert(Comma, Reduce(25));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(25));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(25));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(25));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(25));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(25));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(25));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(25));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(25));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(25));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(25));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(25));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(25));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(25));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(25));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(25));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(25));
    hashmap.insert(And, Reduce(25));
    table.push(hashmap);

    // for state 230
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(148));
    hashmap.insert(Star, Shift(143));
    hashmap.insert(Identifier, Shift(150));
    hashmap.insert(Lparen, Shift(151));
    hashmap.insert(StringLit, Shift(149));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(145));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(145));
    hashmap.insert(And, Shift(144));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(142));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(142));
    hashmap.insert(OPERAND, Goto(147));
    hashmap.insert(PRIMARY_EXPR, Goto(146));
    hashmap.insert(EXPRESSION, Goto(266));
    hashmap.insert(LOGICAL_EXPR, Goto(137));
    hashmap.insert(RELATIONAL_EXPR, Goto(138));
    hashmap.insert(ADDITIVE_EXPR, Goto(139));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(140));
    hashmap.insert(UNARY_EXPR, Goto(141));
    table.push(hashmap);

    // for state 231
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(165));
    hashmap.insert(Star, Shift(160));
    hashmap.insert(Identifier, Shift(167));
    hashmap.insert(Lparen, Shift(168));
    hashmap.insert(Rparen, Reduce(31));
    hashmap.insert(StringLit, Shift(166));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(162));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(162));
    hashmap.insert(And, Shift(161));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(159));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(159));
    hashmap.insert(OPERAND, Goto(164));
    hashmap.insert(PRIMARY_EXPR, Goto(163));
    hashmap.insert(EXPRESSION_LIST, Goto(267));
    hashmap.insert(EXPRESSION, Goto(153));
    hashmap.insert(LOGICAL_EXPR, Goto(154));
    hashmap.insert(RELATIONAL_EXPR, Goto(155));
    hashmap.insert(ADDITIVE_EXPR, Goto(156));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(157));
    hashmap.insert(UNARY_EXPR, Goto(158));
    table.push(hashmap);

    // for state 232
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Shift(268));
    table.push(hashmap);

    // for state 233
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Reduce(33));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(33));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(33));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Shift(171));
    hashmap.insert(RelOp(RelativeOperator::Greater), Shift(171));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Shift(171));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Shift(171));
    hashmap.insert(RelOp(RelativeOperator::Equal), Shift(171));
    hashmap.insert(RelOp(RelativeOperator::Less), Shift(171));
    table.push(hashmap);

    // for state 234
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Reduce(35));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(35));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(35));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(35));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(35));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(35));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(35));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(35));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(35));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Shift(172));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Shift(172));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(173));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(173));
    table.push(hashmap);

    // for state 235
    let mut hashmap = HashMap::new();
    hashmap.insert(Star, Shift(175));
    hashmap.insert(Rparen, Reduce(37));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(37));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(37));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(37));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(37));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(37));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(37));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(37));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(37));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(37));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(37));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(37));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(37));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Shift(174));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Shift(174));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Shift(174));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Shift(174));
    hashmap.insert(And, Shift(176));
    table.push(hashmap);

    // for state 236
    let mut hashmap = HashMap::new();
    hashmap.insert(Star, Shift(175));
    hashmap.insert(Rparen, Reduce(38));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(38));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(38));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(38));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(38));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(38));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(38));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(38));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(38));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(38));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(38));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(38));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(38));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Shift(174));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Shift(174));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Shift(174));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Shift(174));
    hashmap.insert(And, Shift(176));
    table.push(hashmap);

    // for state 237
    let mut hashmap = HashMap::new();
    hashmap.insert(Star, Reduce(40));
    hashmap.insert(Rparen, Reduce(40));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(40));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(40));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(40));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(40));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(40));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(40));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(40));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(40));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(40));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(40));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(40));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(40));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(40));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(40));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(40));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(40));
    hashmap.insert(And, Reduce(40));
    table.push(hashmap);

    // for state 238
    let mut hashmap = HashMap::new();
    hashmap.insert(Star, Reduce(41));
    hashmap.insert(Rparen, Reduce(41));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(41));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(41));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(41));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(41));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(41));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(41));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(41));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(41));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(41));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(41));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(41));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(41));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(41));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(41));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(41));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(41));
    hashmap.insert(And, Reduce(41));
    table.push(hashmap);

    // for state 239
    let mut hashmap = HashMap::new();
    hashmap.insert(Star, Reduce(42));
    hashmap.insert(Rparen, Reduce(42));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(42));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(42));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(42));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(42));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(42));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(42));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(42));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(42));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(42));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(42));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(42));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(42));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(42));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(42));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(42));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(42));
    hashmap.insert(And, Reduce(42));
    table.push(hashmap);

    // for state 240
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbracket, Shift(269));
    table.push(hashmap);

    // for state 241
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Shift(270));
    table.push(hashmap);

    // for state 242
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(23));
    hashmap.insert(Star, Reduce(23));
    hashmap.insert(Lparen, Reduce(23));
    hashmap.insert(Rparen, Reduce(23));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(23));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(23));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(23));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(23));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(23));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(23));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(23));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(23));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(23));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(23));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(23));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(23));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(23));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(23));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(23));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(23));
    hashmap.insert(And, Reduce(23));
    table.push(hashmap);

    // for state 243
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(27));
    hashmap.insert(Star, Reduce(27));
    hashmap.insert(Rbrace, Reduce(27));
    hashmap.insert(Semicolon, Reduce(27));
    hashmap.insert(Lparen, Reduce(27));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(27));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(27));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(27));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(27));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(27));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(27));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(27));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(27));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(27));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(27));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(27));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(27));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(27));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(27));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(27));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(27));
    hashmap.insert(And, Reduce(27));
    table.push(hashmap);

    // for state 244
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(28));
    hashmap.insert(Star, Reduce(28));
    hashmap.insert(Rbrace, Reduce(28));
    hashmap.insert(Semicolon, Reduce(28));
    hashmap.insert(Lparen, Reduce(28));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(28));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(28));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(28));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(28));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(28));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(28));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(28));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(28));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(28));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(28));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(28));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(28));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(28));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(28));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(28));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(28));
    hashmap.insert(And, Reduce(28));
    table.push(hashmap);

    // for state 245
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(58));
    hashmap.insert(Semicolon, Reduce(58));
    table.push(hashmap);

    // for state 246
    let mut hashmap = HashMap::new();
    hashmap.insert(Int, Shift(4));
    hashmap.insert(Void, Shift(5));
    hashmap.insert(IntLit, Shift(48));
    hashmap.insert(Star, Shift(43));
    hashmap.insert(Rbrace, Reduce(12));
    hashmap.insert(Identifier, Shift(50));
    hashmap.insert(Lparen, Shift(51));
    hashmap.insert(StringLit, Shift(49));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(45));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(45));
    hashmap.insert(And, Shift(44));
    hashmap.insert(UnaryOp(UnaryOperator::Not), Shift(42));
    hashmap.insert(UnaryOp(UnaryOperator::BitwiseNot), Shift(42));
    hashmap.insert(If, Shift(35));
    hashmap.insert(While, Shift(36));
    hashmap.insert(Return, Shift(32));
    hashmap.insert(Break, Shift(33));
    hashmap.insert(Continue, Shift(34));
    hashmap.insert(TYPE, Goto(31));
    hashmap.insert(ARRAY_TYPE, Goto(6));
    hashmap.insert(POINTER_TYPE, Goto(7));
    hashmap.insert(STATEMENT_LIST, Goto(271));
    hashmap.insert(VAR_DECL, Goto(24));
    hashmap.insert(OPERAND, Goto(47));
    hashmap.insert(PRIMARY_EXPR, Goto(46));
    hashmap.insert(EXPRESSION, Goto(30));
    hashmap.insert(LOGICAL_EXPR, Goto(37));
    hashmap.insert(RELATIONAL_EXPR, Goto(38));
    hashmap.insert(ADDITIVE_EXPR, Goto(39));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(40));
    hashmap.insert(UNARY_EXPR, Goto(41));
    hashmap.insert(STATEMENT, Goto(22));
    hashmap.insert(ASSIGNMENT, Goto(23));
    hashmap.insert(IF_STMT, Goto(28));
    hashmap.insert(WHILE_STMT, Goto(29));
    hashmap.insert(RETURN_STMT, Goto(25));
    hashmap.insert(BREAK_STMT, Goto(26));
    hashmap.insert(CONTINUE_STMT, Goto(27));
    table.push(hashmap);

    // for state 247
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(59));
    hashmap.insert(Semicolon, Reduce(59));
    table.push(hashmap);

    // for state 248
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbracket, Reduce(33));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(33));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(33));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Shift(200));
    hashmap.insert(RelOp(RelativeOperator::Greater), Shift(200));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Shift(200));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Shift(200));
    hashmap.insert(RelOp(RelativeOperator::Equal), Shift(200));
    hashmap.insert(RelOp(RelativeOperator::Less), Shift(200));
    table.push(hashmap);

    // for state 249
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbracket, Reduce(35));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(35));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(35));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(35));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(35));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(35));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(35));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(35));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(35));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Shift(201));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Shift(201));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(202));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(202));
    table.push(hashmap);

    // for state 250
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbracket, Reduce(37));
    hashmap.insert(Star, Shift(204));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(37));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(37));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(37));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(37));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(37));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(37));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(37));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(37));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(37));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(37));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(37));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(37));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Shift(203));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Shift(203));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Shift(203));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Shift(203));
    hashmap.insert(And, Shift(205));
    table.push(hashmap);

    // for state 251
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbracket, Reduce(38));
    hashmap.insert(Star, Shift(204));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(38));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(38));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(38));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(38));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(38));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(38));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(38));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(38));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(38));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(38));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(38));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(38));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Shift(203));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Shift(203));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Shift(203));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Shift(203));
    hashmap.insert(And, Shift(205));
    table.push(hashmap);

    // for state 252
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbracket, Reduce(40));
    hashmap.insert(Star, Reduce(40));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(40));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(40));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(40));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(40));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(40));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(40));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(40));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(40));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(40));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(40));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(40));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(40));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(40));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(40));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(40));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(40));
    hashmap.insert(And, Reduce(40));
    table.push(hashmap);

    // for state 253
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbracket, Reduce(41));
    hashmap.insert(Star, Reduce(41));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(41));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(41));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(41));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(41));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(41));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(41));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(41));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(41));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(41));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(41));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(41));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(41));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(41));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(41));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(41));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(41));
    hashmap.insert(And, Reduce(41));
    table.push(hashmap);

    // for state 254
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbracket, Reduce(42));
    hashmap.insert(Star, Reduce(42));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(42));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(42));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(42));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(42));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(42));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(42));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(42));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(42));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(42));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(42));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(42));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(42));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(42));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(42));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(42));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(42));
    hashmap.insert(And, Reduce(42));
    table.push(hashmap);

    // for state 255
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbracket, Shift(272));
    table.push(hashmap);

    // for state 256
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Shift(273));
    table.push(hashmap);

    // for state 257
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(23));
    hashmap.insert(Rbracket, Reduce(23));
    hashmap.insert(Star, Reduce(23));
    hashmap.insert(Lparen, Reduce(23));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(23));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(23));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(23));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(23));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(23));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(23));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(23));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(23));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(23));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(23));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(23));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(23));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(23));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(23));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(23));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(23));
    hashmap.insert(And, Reduce(23));
    table.push(hashmap);

    // for state 258
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Reduce(29));
    table.push(hashmap);

    // for state 259
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Reduce(33));
    hashmap.insert(Comma, Reduce(33));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(33));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(33));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Shift(218));
    hashmap.insert(RelOp(RelativeOperator::Greater), Shift(218));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Shift(218));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Shift(218));
    hashmap.insert(RelOp(RelativeOperator::Equal), Shift(218));
    hashmap.insert(RelOp(RelativeOperator::Less), Shift(218));
    table.push(hashmap);

    // for state 260
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Reduce(35));
    hashmap.insert(Comma, Reduce(35));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(35));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(35));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(35));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(35));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(35));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(35));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(35));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(35));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Shift(219));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Shift(219));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Shift(220));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Shift(220));
    table.push(hashmap);

    // for state 261
    let mut hashmap = HashMap::new();
    hashmap.insert(Star, Shift(222));
    hashmap.insert(Rparen, Reduce(37));
    hashmap.insert(Comma, Reduce(37));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(37));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(37));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(37));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(37));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(37));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(37));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(37));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(37));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(37));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(37));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(37));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(37));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Shift(221));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Shift(221));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Shift(221));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Shift(221));
    hashmap.insert(And, Shift(223));
    table.push(hashmap);

    // for state 262
    let mut hashmap = HashMap::new();
    hashmap.insert(Star, Shift(222));
    hashmap.insert(Rparen, Reduce(38));
    hashmap.insert(Comma, Reduce(38));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(38));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(38));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(38));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(38));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(38));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(38));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(38));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(38));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(38));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(38));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(38));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(38));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Shift(221));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Shift(221));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Shift(221));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Shift(221));
    hashmap.insert(And, Shift(223));
    table.push(hashmap);

    // for state 263
    let mut hashmap = HashMap::new();
    hashmap.insert(Star, Reduce(40));
    hashmap.insert(Rparen, Reduce(40));
    hashmap.insert(Comma, Reduce(40));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(40));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(40));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(40));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(40));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(40));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(40));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(40));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(40));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(40));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(40));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(40));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(40));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(40));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(40));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(40));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(40));
    hashmap.insert(And, Reduce(40));
    table.push(hashmap);

    // for state 264
    let mut hashmap = HashMap::new();
    hashmap.insert(Star, Reduce(41));
    hashmap.insert(Rparen, Reduce(41));
    hashmap.insert(Comma, Reduce(41));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(41));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(41));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(41));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(41));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(41));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(41));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(41));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(41));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(41));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(41));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(41));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(41));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(41));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(41));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(41));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(41));
    hashmap.insert(And, Reduce(41));
    table.push(hashmap);

    // for state 265
    let mut hashmap = HashMap::new();
    hashmap.insert(Star, Reduce(42));
    hashmap.insert(Rparen, Reduce(42));
    hashmap.insert(Comma, Reduce(42));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(42));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(42));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(42));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(42));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(42));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(42));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(42));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(42));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(42));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(42));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(42));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(42));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(42));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(42));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(42));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(42));
    hashmap.insert(And, Reduce(42));
    table.push(hashmap);

    // for state 266
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbracket, Shift(274));
    table.push(hashmap);

    // for state 267
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Shift(275));
    table.push(hashmap);

    // for state 268
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(23));
    hashmap.insert(Star, Reduce(23));
    hashmap.insert(Lparen, Reduce(23));
    hashmap.insert(Rparen, Reduce(23));
    hashmap.insert(Comma, Reduce(23));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(23));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(23));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(23));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(23));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(23));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(23));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(23));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(23));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(23));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(23));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(23));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(23));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(23));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(23));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(23));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(23));
    hashmap.insert(And, Reduce(23));
    table.push(hashmap);

    // for state 269
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(27));
    hashmap.insert(Star, Reduce(27));
    hashmap.insert(Lparen, Reduce(27));
    hashmap.insert(Rparen, Reduce(27));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(27));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(27));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(27));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(27));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(27));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(27));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(27));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(27));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(27));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(27));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(27));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(27));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(27));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(27));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(27));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(27));
    hashmap.insert(And, Reduce(27));
    table.push(hashmap);

    // for state 270
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(28));
    hashmap.insert(Star, Reduce(28));
    hashmap.insert(Lparen, Reduce(28));
    hashmap.insert(Rparen, Reduce(28));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(28));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(28));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(28));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(28));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(28));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(28));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(28));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(28));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(28));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(28));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(28));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(28));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(28));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(28));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(28));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(28));
    hashmap.insert(And, Reduce(28));
    table.push(hashmap);

    // for state 271
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Shift(276));
    table.push(hashmap);

    // for state 272
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(27));
    hashmap.insert(Rbracket, Reduce(27));
    hashmap.insert(Star, Reduce(27));
    hashmap.insert(Lparen, Reduce(27));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(27));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(27));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(27));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(27));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(27));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(27));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(27));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(27));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(27));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(27));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(27));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(27));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(27));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(27));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(27));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(27));
    hashmap.insert(And, Reduce(27));
    table.push(hashmap);

    // for state 273
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(28));
    hashmap.insert(Rbracket, Reduce(28));
    hashmap.insert(Star, Reduce(28));
    hashmap.insert(Lparen, Reduce(28));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(28));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(28));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(28));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(28));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(28));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(28));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(28));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(28));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(28));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(28));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(28));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(28));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(28));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(28));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(28));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(28));
    hashmap.insert(And, Reduce(28));
    table.push(hashmap);

    // for state 274
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(27));
    hashmap.insert(Star, Reduce(27));
    hashmap.insert(Lparen, Reduce(27));
    hashmap.insert(Rparen, Reduce(27));
    hashmap.insert(Comma, Reduce(27));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(27));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(27));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(27));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(27));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(27));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(27));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(27));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(27));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(27));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(27));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(27));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(27));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(27));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(27));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(27));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(27));
    hashmap.insert(And, Reduce(27));
    table.push(hashmap);

    // for state 275
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(28));
    hashmap.insert(Star, Reduce(28));
    hashmap.insert(Lparen, Reduce(28));
    hashmap.insert(Rparen, Reduce(28));
    hashmap.insert(Comma, Reduce(28));
    hashmap.insert(LogOp(LogicalOperator::Or), Reduce(28));
    hashmap.insert(LogOp(LogicalOperator::And), Reduce(28));
    hashmap.insert(RelOp(RelativeOperator::NotEqual), Reduce(28));
    hashmap.insert(RelOp(RelativeOperator::Greater), Reduce(28));
    hashmap.insert(RelOp(RelativeOperator::LessEqual), Reduce(28));
    hashmap.insert(RelOp(RelativeOperator::GreaterEqual), Reduce(28));
    hashmap.insert(RelOp(RelativeOperator::Equal), Reduce(28));
    hashmap.insert(RelOp(RelativeOperator::Less), Reduce(28));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseAnd), Reduce(28));
    hashmap.insert(AddOp(AdditiveOperator::BitwiseOr), Reduce(28));
    hashmap.insert(AddMinus(AddMinusOperator::Minus), Reduce(28));
    hashmap.insert(AddMinus(AddMinusOperator::Add), Reduce(28));
    hashmap.insert(MulOp(MultiplicativeOperator::Mod), Reduce(28));
    hashmap.insert(MulOp(MultiplicativeOperator::RightShift), Reduce(28));
    hashmap.insert(MulOp(MultiplicativeOperator::LeftShift), Reduce(28));
    hashmap.insert(MulOp(MultiplicativeOperator::Div), Reduce(28));
    hashmap.insert(And, Reduce(28));
    table.push(hashmap);

    // for state 276
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(9));
    hashmap.insert(Semicolon, Reduce(9));
    table.push(hashmap);
