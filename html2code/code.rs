
    if state == 0 {
        match token {
            Int => return Some(Shift(4)),
            Void => return Some(Shift(5)),
            EOL => return Some(Reduce(2)),
            PROGRAM => return Some(Goto(1)),
            TYPE => return Some(Goto(3)),
            ARRAY_TYPE => return Some(Goto(6)),
            POINTER_TYPE => return Some(Goto(7)),
            FUNCTION_DECL => return Some(Goto(2)),
            _ => return None,
        };
    }

    if state == 1 {
        match token {
            EOL => return Some(Goto(acc)),
            _ => return None,
        };
    }

    if state == 2 {
        match token {
            Int => return Some(Shift(4)),
            Void => return Some(Shift(5)),
            EOL => return Some(Reduce(2)),
            PROGRAM => return Some(Goto(8)),
            TYPE => return Some(Goto(3)),
            ARRAY_TYPE => return Some(Goto(6)),
            POINTER_TYPE => return Some(Goto(7)),
            FUNCTION_DECL => return Some(Goto(2)),
            _ => return None,
        };
    }

    if state == 3 {
        match token {
            Lbracket => return Some(Shift(10)),
            Star => return Some(Shift(11)),
            Identifier(_) => return Some(Shift(9)),
            _ => return None,
        };
    }

    if state == 4 {
        match token {
            Lbracket => return Some(Reduce(3)),
            Star => return Some(Reduce(3)),
            Identifier(_) => return Some(Reduce(3)),
            _ => return None,
        };
    }

    if state == 5 {
        match token {
            Lbracket => return Some(Reduce(4)),
            Star => return Some(Reduce(4)),
            Identifier(_) => return Some(Reduce(4)),
            _ => return None,
        };
    }

    if state == 6 {
        match token {
            Lbracket => return Some(Reduce(5)),
            Star => return Some(Reduce(5)),
            Identifier(_) => return Some(Reduce(5)),
            _ => return None,
        };
    }

    if state == 7 {
        match token {
            Lbracket => return Some(Reduce(6)),
            Star => return Some(Reduce(6)),
            Identifier(_) => return Some(Reduce(6)),
            _ => return None,
        };
    }

    if state == 8 {
        match token {
            EOL => return Some(Reduce(1)),
            _ => return None,
        };
    }

    if state == 9 {
        match token {
            Lparen => return Some(Shift(13)),
            PARAMETERS => return Some(Goto(12)),
            _ => return None,
        };
    }

    if state == 10 {
        match token {
            IntLit(_) => return Some(Shift(14)),
            _ => return None,
        };
    }

    if state == 11 {
        match token {
            Identifier(_) => return Some(Reduce(8)),
            _ => return None,
        };
    }

    if state == 12 {
        match token {
            Lbrace => return Some(Shift(16)),
            BLOCK => return Some(Goto(15)),
            _ => return None,
        };
    }

    if state == 13 {
        match token {
            Int => return Some(Shift(4)),
            Void => return Some(Shift(5)),
            Rparen => return Some(Reduce(18)),
            TYPE => return Some(Goto(19)),
            ARRAY_TYPE => return Some(Goto(6)),
            POINTER_TYPE => return Some(Goto(7)),
            PARAMETER_LIST => return Some(Goto(17)),
            PARAMETER_DECL => return Some(Goto(18)),
            _ => return None,
        };
    }

    if state == 14 {
        match token {
            Rbracket => return Some(Shift(20)),
            _ => return None,
        };
    }

    if state == 15 {
        match token {
            Int => return Some(Reduce(14)),
            Void => return Some(Reduce(14)),
            EOL => return Some(Reduce(14)),
            _ => return None,
        };
    }

    if state == 16 {
        match token {
            Int => return Some(Shift(4)),
            Void => return Some(Shift(5)),
            IntLit(_) => return Some(Shift(48)),
            Star => return Some(Shift(43)),
            Rbrace => return Some(Reduce(12)),
            Identifier(_) => return Some(Shift(50)),
            Lparen => return Some(Shift(51)),
            StringLit(_) => return Some(Shift(49)),
            AddMinus(_) => return Some(Shift(45)),
            And => return Some(Shift(44)),
            UnaryOp(_) => return Some(Shift(42)),
            If => return Some(Shift(35)),
            While => return Some(Shift(36)),
            Return => return Some(Shift(32)),
            Break => return Some(Shift(33)),
            Continue => return Some(Shift(34)),
            TYPE => return Some(Goto(31)),
            ARRAY_TYPE => return Some(Goto(6)),
            POINTER_TYPE => return Some(Goto(7)),
            STATEMENT_LIST => return Some(Goto(21)),
            VAR_DECL => return Some(Goto(24)),
            OPERAND => return Some(Goto(47)),
            PRIMARY_EXPR => return Some(Goto(46)),
            EXPRESSION => return Some(Goto(30)),
            LOGICAL_EXPR => return Some(Goto(37)),
            RELATIONAL_EXPR => return Some(Goto(38)),
            ADDITIVE_EXPR => return Some(Goto(39)),
            MULTIPLICATIVE_EXPR => return Some(Goto(40)),
            UNARY_EXPR => return Some(Goto(41)),
            STATEMENT => return Some(Goto(22)),
            ASSIGNMENT => return Some(Goto(23)),
            IF_STMT => return Some(Goto(28)),
            WHILE_STMT => return Some(Goto(29)),
            RETURN_STMT => return Some(Goto(25)),
            BREAK_STMT => return Some(Goto(26)),
            CONTINUE_STMT => return Some(Goto(27)),
            _ => return None,
        };
    }

    if state == 17 {
        match token {
            Rparen => return Some(Shift(52)),
            _ => return None,
        };
    }

    if state == 18 {
        match token {
            Rparen => return Some(Reduce(17)),
            Comma => return Some(Shift(53)),
            _ => return None,
        };
    }

    if state == 19 {
        match token {
            Lbracket => return Some(Shift(10)),
            Star => return Some(Shift(11)),
            Identifier(_) => return Some(Shift(54)),
            _ => return None,
        };
    }

    if state == 20 {
        match token {
            Identifier(_) => return Some(Reduce(7)),
            _ => return None,
        };
    }

    if state == 21 {
        match token {
            Rbrace => return Some(Shift(55)),
            _ => return None,
        };
    }

    if state == 22 {
        match token {
            Rbrace => return Some(Reduce(11)),
            Semicolon => return Some(Shift(56)),
            _ => return None,
        };
    }

    if state == 23 {
        match token {
            Rbrace => return Some(Reduce(49)),
            Semicolon => return Some(Reduce(49)),
            _ => return None,
        };
    }

    if state == 24 {
        match token {
            Rbrace => return Some(Reduce(50)),
            Semicolon => return Some(Reduce(50)),
            _ => return None,
        };
    }

    if state == 25 {
        match token {
            Rbrace => return Some(Reduce(51)),
            Semicolon => return Some(Reduce(51)),
            _ => return None,
        };
    }

    if state == 26 {
        match token {
            Rbrace => return Some(Reduce(52)),
            Semicolon => return Some(Reduce(52)),
            _ => return None,
        };
    }

    if state == 27 {
        match token {
            Rbrace => return Some(Reduce(53)),
            Semicolon => return Some(Reduce(53)),
            _ => return None,
        };
    }

    if state == 28 {
        match token {
            Rbrace => return Some(Reduce(54)),
            Semicolon => return Some(Reduce(54)),
            _ => return None,
        };
    }

    if state == 29 {
        match token {
            Rbrace => return Some(Reduce(55)),
            Semicolon => return Some(Reduce(55)),
            _ => return None,
        };
    }

    if state == 30 {
        match token {
            Rbrace => return Some(Reduce(56)),
            Semicolon => return Some(Reduce(56)),
            AssignOp => return Some(Shift(57)),
            _ => return None,
        };
    }

    if state == 31 {
        match token {
            Lbracket => return Some(Shift(10)),
            Star => return Some(Shift(11)),
            Identifier(_) => return Some(Shift(58)),
            _ => return None,
        };
    }

    if state == 32 {
        match token {
            IntLit(_) => return Some(Shift(71)),
            Star => return Some(Shift(66)),
            Identifier(_) => return Some(Shift(73)),
            Lparen => return Some(Shift(74)),
            StringLit(_) => return Some(Shift(72)),
            AddMinus(_) => return Some(Shift(68)),
            And => return Some(Shift(67)),
            UnaryOp(_) => return Some(Shift(65)),
            OPERAND => return Some(Goto(70)),
            PRIMARY_EXPR => return Some(Goto(69)),
            EXPRESSION => return Some(Goto(59)),
            LOGICAL_EXPR => return Some(Goto(60)),
            RELATIONAL_EXPR => return Some(Goto(61)),
            ADDITIVE_EXPR => return Some(Goto(62)),
            MULTIPLICATIVE_EXPR => return Some(Goto(63)),
            UNARY_EXPR => return Some(Goto(64)),
            _ => return None,
        };
    }

    if state == 33 {
        match token {
            Rbrace => return Some(Reduce(61)),
            Semicolon => return Some(Reduce(61)),
            _ => return None,
        };
    }

    if state == 34 {
        match token {
            Rbrace => return Some(Reduce(62)),
            Semicolon => return Some(Reduce(62)),
            _ => return None,
        };
    }

    if state == 35 {
        match token {
            Lparen => return Some(Shift(75)),
            _ => return None,
        };
    }

    if state == 36 {
        match token {
            Lparen => return Some(Shift(76)),
            _ => return None,
        };
    }

    if state == 37 {
        match token {
            Rbrace => return Some(Reduce(32)),
            Semicolon => return Some(Reduce(32)),
            LogOp(_) => return Some(Shift(77)),
            AssignOp => return Some(Reduce(32)),
            _ => return None,
        };
    }

    if state == 38 {
        match token {
            Rbrace => return Some(Reduce(34)),
            Semicolon => return Some(Reduce(34)),
            LogOp(_) => return Some(Reduce(34)),
            RelOp(_) => return Some(Shift(78)),
            AssignOp => return Some(Reduce(34)),
            _ => return None,
        };
    }

    if state == 39 {
        match token {
            Rbrace => return Some(Reduce(36)),
            Semicolon => return Some(Reduce(36)),
            LogOp(_) => return Some(Reduce(36)),
            RelOp(_) => return Some(Reduce(36)),
            AddOp(_) => return Some(Shift(79)),
            AddMinus(_) => return Some(Shift(80)),
            AssignOp => return Some(Reduce(36)),
            _ => return None,
        };
    }

    if state == 40 {
        match token {
            Star => return Some(Shift(82)),
            Rbrace => return Some(Reduce(39)),
            Semicolon => return Some(Reduce(39)),
            LogOp(_) => return Some(Reduce(39)),
            RelOp(_) => return Some(Reduce(39)),
            AddOp(_) => return Some(Reduce(39)),
            AddMinus(_) => return Some(Reduce(39)),
            MulOp(_) => return Some(Shift(81)),
            And => return Some(Shift(83)),
            AssignOp => return Some(Reduce(39)),
            _ => return None,
        };
    }

    if state == 41 {
        match token {
            Star => return Some(Reduce(43)),
            Rbrace => return Some(Reduce(43)),
            Semicolon => return Some(Reduce(43)),
            LogOp(_) => return Some(Reduce(43)),
            RelOp(_) => return Some(Reduce(43)),
            AddOp(_) => return Some(Reduce(43)),
            AddMinus(_) => return Some(Reduce(43)),
            MulOp(_) => return Some(Reduce(43)),
            And => return Some(Reduce(43)),
            AssignOp => return Some(Reduce(43)),
            _ => return None,
        };
    }

    if state == 42 {
        match token {
            IntLit(_) => return Some(Shift(48)),
            Star => return Some(Shift(43)),
            Identifier(_) => return Some(Shift(50)),
            Lparen => return Some(Shift(51)),
            StringLit(_) => return Some(Shift(49)),
            AddMinus(_) => return Some(Shift(45)),
            And => return Some(Shift(44)),
            UnaryOp(_) => return Some(Shift(42)),
            OPERAND => return Some(Goto(47)),
            PRIMARY_EXPR => return Some(Goto(46)),
            UNARY_EXPR => return Some(Goto(84)),
            _ => return None,
        };
    }

    if state == 43 {
        match token {
            IntLit(_) => return Some(Shift(48)),
            Star => return Some(Shift(43)),
            Identifier(_) => return Some(Shift(50)),
            Lparen => return Some(Shift(51)),
            StringLit(_) => return Some(Shift(49)),
            AddMinus(_) => return Some(Shift(45)),
            And => return Some(Shift(44)),
            UnaryOp(_) => return Some(Shift(42)),
            OPERAND => return Some(Goto(47)),
            PRIMARY_EXPR => return Some(Goto(46)),
            UNARY_EXPR => return Some(Goto(85)),
            _ => return None,
        };
    }

    if state == 44 {
        match token {
            IntLit(_) => return Some(Shift(48)),
            Star => return Some(Shift(43)),
            Identifier(_) => return Some(Shift(50)),
            Lparen => return Some(Shift(51)),
            StringLit(_) => return Some(Shift(49)),
            AddMinus(_) => return Some(Shift(45)),
            And => return Some(Shift(44)),
            UnaryOp(_) => return Some(Shift(42)),
            OPERAND => return Some(Goto(47)),
            PRIMARY_EXPR => return Some(Goto(46)),
            UNARY_EXPR => return Some(Goto(86)),
            _ => return None,
        };
    }

    if state == 45 {
        match token {
            IntLit(_) => return Some(Shift(48)),
            Star => return Some(Shift(43)),
            Identifier(_) => return Some(Shift(50)),
            Lparen => return Some(Shift(51)),
            StringLit(_) => return Some(Shift(49)),
            AddMinus(_) => return Some(Shift(45)),
            And => return Some(Shift(44)),
            UnaryOp(_) => return Some(Shift(42)),
            OPERAND => return Some(Goto(47)),
            PRIMARY_EXPR => return Some(Goto(46)),
            UNARY_EXPR => return Some(Goto(87)),
            _ => return None,
        };
    }

    if state == 46 {
        match token {
            Lbracket => return Some(Shift(90)),
            Star => return Some(Reduce(48)),
            Rbrace => return Some(Reduce(48)),
            Semicolon => return Some(Reduce(48)),
            Lparen => return Some(Shift(91)),
            LogOp(_) => return Some(Reduce(48)),
            RelOp(_) => return Some(Reduce(48)),
            AddOp(_) => return Some(Reduce(48)),
            AddMinus(_) => return Some(Reduce(48)),
            MulOp(_) => return Some(Reduce(48)),
            And => return Some(Reduce(48)),
            AssignOp => return Some(Reduce(48)),
            INDEX => return Some(Goto(88)),
            ARGUMENTS => return Some(Goto(89)),
            _ => return None,
        };
    }

    if state == 47 {
        match token {
            Lbracket => return Some(Reduce(26)),
            Star => return Some(Reduce(26)),
            Rbrace => return Some(Reduce(26)),
            Semicolon => return Some(Reduce(26)),
            Lparen => return Some(Reduce(26)),
            LogOp(_) => return Some(Reduce(26)),
            RelOp(_) => return Some(Reduce(26)),
            AddOp(_) => return Some(Reduce(26)),
            AddMinus(_) => return Some(Reduce(26)),
            MulOp(_) => return Some(Reduce(26)),
            And => return Some(Reduce(26)),
            AssignOp => return Some(Reduce(26)),
            _ => return None,
        };
    }

    if state == 48 {
        match token {
            Lbracket => return Some(Reduce(20)),
            Star => return Some(Reduce(20)),
            Rbrace => return Some(Reduce(20)),
            Semicolon => return Some(Reduce(20)),
            Lparen => return Some(Reduce(20)),
            LogOp(_) => return Some(Reduce(20)),
            RelOp(_) => return Some(Reduce(20)),
            AddOp(_) => return Some(Reduce(20)),
            AddMinus(_) => return Some(Reduce(20)),
            MulOp(_) => return Some(Reduce(20)),
            And => return Some(Reduce(20)),
            AssignOp => return Some(Reduce(20)),
            _ => return None,
        };
    }

    if state == 49 {
        match token {
            Lbracket => return Some(Reduce(21)),
            Star => return Some(Reduce(21)),
            Rbrace => return Some(Reduce(21)),
            Semicolon => return Some(Reduce(21)),
            Lparen => return Some(Reduce(21)),
            LogOp(_) => return Some(Reduce(21)),
            RelOp(_) => return Some(Reduce(21)),
            AddOp(_) => return Some(Reduce(21)),
            AddMinus(_) => return Some(Reduce(21)),
            MulOp(_) => return Some(Reduce(21)),
            And => return Some(Reduce(21)),
            AssignOp => return Some(Reduce(21)),
            _ => return None,
        };
    }

    if state == 50 {
        match token {
            Lbracket => return Some(Reduce(22)),
            Star => return Some(Reduce(22)),
            Rbrace => return Some(Reduce(22)),
            Semicolon => return Some(Reduce(22)),
            Lparen => return Some(Reduce(22)),
            LogOp(_) => return Some(Reduce(22)),
            RelOp(_) => return Some(Reduce(22)),
            AddOp(_) => return Some(Reduce(22)),
            AddMinus(_) => return Some(Reduce(22)),
            MulOp(_) => return Some(Reduce(22)),
            And => return Some(Reduce(22)),
            AssignOp => return Some(Reduce(22)),
            _ => return None,
        };
    }

    if state == 51 {
        match token {
            IntLit(_) => return Some(Shift(104)),
            Star => return Some(Shift(99)),
            Identifier(_) => return Some(Shift(106)),
            Lparen => return Some(Shift(107)),
            StringLit(_) => return Some(Shift(105)),
            AddMinus(_) => return Some(Shift(101)),
            And => return Some(Shift(100)),
            UnaryOp(_) => return Some(Shift(98)),
            OPERAND => return Some(Goto(103)),
            PRIMARY_EXPR => return Some(Goto(102)),
            EXPRESSION => return Some(Goto(92)),
            LOGICAL_EXPR => return Some(Goto(93)),
            RELATIONAL_EXPR => return Some(Goto(94)),
            ADDITIVE_EXPR => return Some(Goto(95)),
            MULTIPLICATIVE_EXPR => return Some(Goto(96)),
            UNARY_EXPR => return Some(Goto(97)),
            _ => return None,
        };
    }

    if state == 52 {
        match token {
            Lbrace => return Some(Reduce(15)),
            _ => return None,
        };
    }

    if state == 53 {
        match token {
            Int => return Some(Shift(4)),
            Void => return Some(Shift(5)),
            Rparen => return Some(Reduce(18)),
            TYPE => return Some(Goto(19)),
            ARRAY_TYPE => return Some(Goto(6)),
            POINTER_TYPE => return Some(Goto(7)),
            PARAMETER_LIST => return Some(Goto(108)),
            PARAMETER_DECL => return Some(Goto(18)),
            _ => return None,
        };
    }

    if state == 54 {
        match token {
            Rparen => return Some(Reduce(19)),
            Comma => return Some(Reduce(19)),
            _ => return None,
        };
    }

    if state == 55 {
        match token {
            Int => return Some(Reduce(9)),
            Void => return Some(Reduce(9)),
            EOL => return Some(Reduce(9)),
            _ => return None,
        };
    }

    if state == 56 {
        match token {
            Int => return Some(Shift(4)),
            Void => return Some(Shift(5)),
            IntLit(_) => return Some(Shift(48)),
            Star => return Some(Shift(43)),
            Rbrace => return Some(Reduce(12)),
            Identifier(_) => return Some(Shift(50)),
            Lparen => return Some(Shift(51)),
            StringLit(_) => return Some(Shift(49)),
            AddMinus(_) => return Some(Shift(45)),
            And => return Some(Shift(44)),
            UnaryOp(_) => return Some(Shift(42)),
            If => return Some(Shift(35)),
            While => return Some(Shift(36)),
            Return => return Some(Shift(32)),
            Break => return Some(Shift(33)),
            Continue => return Some(Shift(34)),
            TYPE => return Some(Goto(31)),
            ARRAY_TYPE => return Some(Goto(6)),
            POINTER_TYPE => return Some(Goto(7)),
            STATEMENT_LIST => return Some(Goto(109)),
            VAR_DECL => return Some(Goto(24)),
            OPERAND => return Some(Goto(47)),
            PRIMARY_EXPR => return Some(Goto(46)),
            EXPRESSION => return Some(Goto(30)),
            LOGICAL_EXPR => return Some(Goto(37)),
            RELATIONAL_EXPR => return Some(Goto(38)),
            ADDITIVE_EXPR => return Some(Goto(39)),
            MULTIPLICATIVE_EXPR => return Some(Goto(40)),
            UNARY_EXPR => return Some(Goto(41)),
            STATEMENT => return Some(Goto(22)),
            ASSIGNMENT => return Some(Goto(23)),
            IF_STMT => return Some(Goto(28)),
            WHILE_STMT => return Some(Goto(29)),
            RETURN_STMT => return Some(Goto(25)),
            BREAK_STMT => return Some(Goto(26)),
            CONTINUE_STMT => return Some(Goto(27)),
            _ => return None,
        };
    }

    if state == 57 {
        match token {
            IntLit(_) => return Some(Shift(71)),
            Star => return Some(Shift(66)),
            Identifier(_) => return Some(Shift(73)),
            Lparen => return Some(Shift(74)),
            StringLit(_) => return Some(Shift(72)),
            AddMinus(_) => return Some(Shift(68)),
            And => return Some(Shift(67)),
            UnaryOp(_) => return Some(Shift(65)),
            OPERAND => return Some(Goto(70)),
            PRIMARY_EXPR => return Some(Goto(69)),
            EXPRESSION => return Some(Goto(110)),
            LOGICAL_EXPR => return Some(Goto(60)),
            RELATIONAL_EXPR => return Some(Goto(61)),
            ADDITIVE_EXPR => return Some(Goto(62)),
            MULTIPLICATIVE_EXPR => return Some(Goto(63)),
            UNARY_EXPR => return Some(Goto(64)),
            _ => return None,
        };
    }

    if state == 58 {
        match token {
            Rbrace => return Some(Reduce(13)),
            Semicolon => return Some(Reduce(13)),
            _ => return None,
        };
    }

    if state == 59 {
        match token {
            Rbrace => return Some(Reduce(60)),
            Semicolon => return Some(Reduce(60)),
            _ => return None,
        };
    }

    if state == 60 {
        match token {
            Rbrace => return Some(Reduce(32)),
            Semicolon => return Some(Reduce(32)),
            LogOp(_) => return Some(Shift(111)),
            _ => return None,
        };
    }

    if state == 61 {
        match token {
            Rbrace => return Some(Reduce(34)),
            Semicolon => return Some(Reduce(34)),
            LogOp(_) => return Some(Reduce(34)),
            RelOp(_) => return Some(Shift(112)),
            _ => return None,
        };
    }

    if state == 62 {
        match token {
            Rbrace => return Some(Reduce(36)),
            Semicolon => return Some(Reduce(36)),
            LogOp(_) => return Some(Reduce(36)),
            RelOp(_) => return Some(Reduce(36)),
            AddOp(_) => return Some(Shift(113)),
            AddMinus(_) => return Some(Shift(114)),
            _ => return None,
        };
    }

    if state == 63 {
        match token {
            Star => return Some(Shift(116)),
            Rbrace => return Some(Reduce(39)),
            Semicolon => return Some(Reduce(39)),
            LogOp(_) => return Some(Reduce(39)),
            RelOp(_) => return Some(Reduce(39)),
            AddOp(_) => return Some(Reduce(39)),
            AddMinus(_) => return Some(Reduce(39)),
            MulOp(_) => return Some(Shift(115)),
            And => return Some(Shift(117)),
            _ => return None,
        };
    }

    if state == 64 {
        match token {
            Star => return Some(Reduce(43)),
            Rbrace => return Some(Reduce(43)),
            Semicolon => return Some(Reduce(43)),
            LogOp(_) => return Some(Reduce(43)),
            RelOp(_) => return Some(Reduce(43)),
            AddOp(_) => return Some(Reduce(43)),
            AddMinus(_) => return Some(Reduce(43)),
            MulOp(_) => return Some(Reduce(43)),
            And => return Some(Reduce(43)),
            _ => return None,
        };
    }

    if state == 65 {
        match token {
            IntLit(_) => return Some(Shift(71)),
            Star => return Some(Shift(66)),
            Identifier(_) => return Some(Shift(73)),
            Lparen => return Some(Shift(74)),
            StringLit(_) => return Some(Shift(72)),
            AddMinus(_) => return Some(Shift(68)),
            And => return Some(Shift(67)),
            UnaryOp(_) => return Some(Shift(65)),
            OPERAND => return Some(Goto(70)),
            PRIMARY_EXPR => return Some(Goto(69)),
            UNARY_EXPR => return Some(Goto(118)),
            _ => return None,
        };
    }

    if state == 66 {
        match token {
            IntLit(_) => return Some(Shift(71)),
            Star => return Some(Shift(66)),
            Identifier(_) => return Some(Shift(73)),
            Lparen => return Some(Shift(74)),
            StringLit(_) => return Some(Shift(72)),
            AddMinus(_) => return Some(Shift(68)),
            And => return Some(Shift(67)),
            UnaryOp(_) => return Some(Shift(65)),
            OPERAND => return Some(Goto(70)),
            PRIMARY_EXPR => return Some(Goto(69)),
            UNARY_EXPR => return Some(Goto(119)),
            _ => return None,
        };
    }

    if state == 67 {
        match token {
            IntLit(_) => return Some(Shift(71)),
            Star => return Some(Shift(66)),
            Identifier(_) => return Some(Shift(73)),
            Lparen => return Some(Shift(74)),
            StringLit(_) => return Some(Shift(72)),
            AddMinus(_) => return Some(Shift(68)),
            And => return Some(Shift(67)),
            UnaryOp(_) => return Some(Shift(65)),
            OPERAND => return Some(Goto(70)),
            PRIMARY_EXPR => return Some(Goto(69)),
            UNARY_EXPR => return Some(Goto(120)),
            _ => return None,
        };
    }

    if state == 68 {
        match token {
            IntLit(_) => return Some(Shift(71)),
            Star => return Some(Shift(66)),
            Identifier(_) => return Some(Shift(73)),
            Lparen => return Some(Shift(74)),
            StringLit(_) => return Some(Shift(72)),
            AddMinus(_) => return Some(Shift(68)),
            And => return Some(Shift(67)),
            UnaryOp(_) => return Some(Shift(65)),
            OPERAND => return Some(Goto(70)),
            PRIMARY_EXPR => return Some(Goto(69)),
            UNARY_EXPR => return Some(Goto(121)),
            _ => return None,
        };
    }

    if state == 69 {
        match token {
            Lbracket => return Some(Shift(124)),
            Star => return Some(Reduce(48)),
            Rbrace => return Some(Reduce(48)),
            Semicolon => return Some(Reduce(48)),
            Lparen => return Some(Shift(125)),
            LogOp(_) => return Some(Reduce(48)),
            RelOp(_) => return Some(Reduce(48)),
            AddOp(_) => return Some(Reduce(48)),
            AddMinus(_) => return Some(Reduce(48)),
            MulOp(_) => return Some(Reduce(48)),
            And => return Some(Reduce(48)),
            INDEX => return Some(Goto(122)),
            ARGUMENTS => return Some(Goto(123)),
            _ => return None,
        };
    }

    if state == 70 {
        match token {
            Lbracket => return Some(Reduce(26)),
            Star => return Some(Reduce(26)),
            Rbrace => return Some(Reduce(26)),
            Semicolon => return Some(Reduce(26)),
            Lparen => return Some(Reduce(26)),
            LogOp(_) => return Some(Reduce(26)),
            RelOp(_) => return Some(Reduce(26)),
            AddOp(_) => return Some(Reduce(26)),
            AddMinus(_) => return Some(Reduce(26)),
            MulOp(_) => return Some(Reduce(26)),
            And => return Some(Reduce(26)),
            _ => return None,
        };
    }

    if state == 71 {
        match token {
            Lbracket => return Some(Reduce(20)),
            Star => return Some(Reduce(20)),
            Rbrace => return Some(Reduce(20)),
            Semicolon => return Some(Reduce(20)),
            Lparen => return Some(Reduce(20)),
            LogOp(_) => return Some(Reduce(20)),
            RelOp(_) => return Some(Reduce(20)),
            AddOp(_) => return Some(Reduce(20)),
            AddMinus(_) => return Some(Reduce(20)),
            MulOp(_) => return Some(Reduce(20)),
            And => return Some(Reduce(20)),
            _ => return None,
        };
    }

    if state == 72 {
        match token {
            Lbracket => return Some(Reduce(21)),
            Star => return Some(Reduce(21)),
            Rbrace => return Some(Reduce(21)),
            Semicolon => return Some(Reduce(21)),
            Lparen => return Some(Reduce(21)),
            LogOp(_) => return Some(Reduce(21)),
            RelOp(_) => return Some(Reduce(21)),
            AddOp(_) => return Some(Reduce(21)),
            AddMinus(_) => return Some(Reduce(21)),
            MulOp(_) => return Some(Reduce(21)),
            And => return Some(Reduce(21)),
            _ => return None,
        };
    }

    if state == 73 {
        match token {
            Lbracket => return Some(Reduce(22)),
            Star => return Some(Reduce(22)),
            Rbrace => return Some(Reduce(22)),
            Semicolon => return Some(Reduce(22)),
            Lparen => return Some(Reduce(22)),
            LogOp(_) => return Some(Reduce(22)),
            RelOp(_) => return Some(Reduce(22)),
            AddOp(_) => return Some(Reduce(22)),
            AddMinus(_) => return Some(Reduce(22)),
            MulOp(_) => return Some(Reduce(22)),
            And => return Some(Reduce(22)),
            _ => return None,
        };
    }

    if state == 74 {
        match token {
            IntLit(_) => return Some(Shift(104)),
            Star => return Some(Shift(99)),
            Identifier(_) => return Some(Shift(106)),
            Lparen => return Some(Shift(107)),
            StringLit(_) => return Some(Shift(105)),
            AddMinus(_) => return Some(Shift(101)),
            And => return Some(Shift(100)),
            UnaryOp(_) => return Some(Shift(98)),
            OPERAND => return Some(Goto(103)),
            PRIMARY_EXPR => return Some(Goto(102)),
            EXPRESSION => return Some(Goto(126)),
            LOGICAL_EXPR => return Some(Goto(93)),
            RELATIONAL_EXPR => return Some(Goto(94)),
            ADDITIVE_EXPR => return Some(Goto(95)),
            MULTIPLICATIVE_EXPR => return Some(Goto(96)),
            UNARY_EXPR => return Some(Goto(97)),
            _ => return None,
        };
    }

    if state == 75 {
        match token {
            IntLit(_) => return Some(Shift(104)),
            Star => return Some(Shift(99)),
            Identifier(_) => return Some(Shift(106)),
            Lparen => return Some(Shift(107)),
            StringLit(_) => return Some(Shift(105)),
            AddMinus(_) => return Some(Shift(101)),
            And => return Some(Shift(100)),
            UnaryOp(_) => return Some(Shift(98)),
            OPERAND => return Some(Goto(103)),
            PRIMARY_EXPR => return Some(Goto(102)),
            EXPRESSION => return Some(Goto(127)),
            LOGICAL_EXPR => return Some(Goto(93)),
            RELATIONAL_EXPR => return Some(Goto(94)),
            ADDITIVE_EXPR => return Some(Goto(95)),
            MULTIPLICATIVE_EXPR => return Some(Goto(96)),
            UNARY_EXPR => return Some(Goto(97)),
            _ => return None,
        };
    }

    if state == 76 {
        match token {
            IntLit(_) => return Some(Shift(104)),
            Star => return Some(Shift(99)),
            Identifier(_) => return Some(Shift(106)),
            Lparen => return Some(Shift(107)),
            StringLit(_) => return Some(Shift(105)),
            AddMinus(_) => return Some(Shift(101)),
            And => return Some(Shift(100)),
            UnaryOp(_) => return Some(Shift(98)),
            OPERAND => return Some(Goto(103)),
            PRIMARY_EXPR => return Some(Goto(102)),
            EXPRESSION => return Some(Goto(128)),
            LOGICAL_EXPR => return Some(Goto(93)),
            RELATIONAL_EXPR => return Some(Goto(94)),
            ADDITIVE_EXPR => return Some(Goto(95)),
            MULTIPLICATIVE_EXPR => return Some(Goto(96)),
            UNARY_EXPR => return Some(Goto(97)),
            _ => return None,
        };
    }

    if state == 77 {
        match token {
            IntLit(_) => return Some(Shift(48)),
            Star => return Some(Shift(43)),
            Identifier(_) => return Some(Shift(50)),
            Lparen => return Some(Shift(51)),
            StringLit(_) => return Some(Shift(49)),
            AddMinus(_) => return Some(Shift(45)),
            And => return Some(Shift(44)),
            UnaryOp(_) => return Some(Shift(42)),
            OPERAND => return Some(Goto(47)),
            PRIMARY_EXPR => return Some(Goto(46)),
            RELATIONAL_EXPR => return Some(Goto(129)),
            ADDITIVE_EXPR => return Some(Goto(39)),
            MULTIPLICATIVE_EXPR => return Some(Goto(40)),
            UNARY_EXPR => return Some(Goto(41)),
            _ => return None,
        };
    }

    if state == 78 {
        match token {
            IntLit(_) => return Some(Shift(48)),
            Star => return Some(Shift(43)),
            Identifier(_) => return Some(Shift(50)),
            Lparen => return Some(Shift(51)),
            StringLit(_) => return Some(Shift(49)),
            AddMinus(_) => return Some(Shift(45)),
            And => return Some(Shift(44)),
            UnaryOp(_) => return Some(Shift(42)),
            OPERAND => return Some(Goto(47)),
            PRIMARY_EXPR => return Some(Goto(46)),
            ADDITIVE_EXPR => return Some(Goto(130)),
            MULTIPLICATIVE_EXPR => return Some(Goto(40)),
            UNARY_EXPR => return Some(Goto(41)),
            _ => return None,
        };
    }

    if state == 79 {
        match token {
            IntLit(_) => return Some(Shift(48)),
            Star => return Some(Shift(43)),
            Identifier(_) => return Some(Shift(50)),
            Lparen => return Some(Shift(51)),
            StringLit(_) => return Some(Shift(49)),
            AddMinus(_) => return Some(Shift(45)),
            And => return Some(Shift(44)),
            UnaryOp(_) => return Some(Shift(42)),
            OPERAND => return Some(Goto(47)),
            PRIMARY_EXPR => return Some(Goto(46)),
            MULTIPLICATIVE_EXPR => return Some(Goto(131)),
            UNARY_EXPR => return Some(Goto(41)),
            _ => return None,
        };
    }

    if state == 80 {
        match token {
            IntLit(_) => return Some(Shift(48)),
            Star => return Some(Shift(43)),
            Identifier(_) => return Some(Shift(50)),
            Lparen => return Some(Shift(51)),
            StringLit(_) => return Some(Shift(49)),
            AddMinus(_) => return Some(Shift(45)),
            And => return Some(Shift(44)),
            UnaryOp(_) => return Some(Shift(42)),
            OPERAND => return Some(Goto(47)),
            PRIMARY_EXPR => return Some(Goto(46)),
            MULTIPLICATIVE_EXPR => return Some(Goto(132)),
            UNARY_EXPR => return Some(Goto(41)),
            _ => return None,
        };
    }

    if state == 81 {
        match token {
            IntLit(_) => return Some(Shift(48)),
            Star => return Some(Shift(43)),
            Identifier(_) => return Some(Shift(50)),
            Lparen => return Some(Shift(51)),
            StringLit(_) => return Some(Shift(49)),
            AddMinus(_) => return Some(Shift(45)),
            And => return Some(Shift(44)),
            UnaryOp(_) => return Some(Shift(42)),
            OPERAND => return Some(Goto(47)),
            PRIMARY_EXPR => return Some(Goto(46)),
            UNARY_EXPR => return Some(Goto(133)),
            _ => return None,
        };
    }

    if state == 82 {
        match token {
            IntLit(_) => return Some(Shift(48)),
            Star => return Some(Shift(43)),
            Identifier(_) => return Some(Shift(50)),
            Lparen => return Some(Shift(51)),
            StringLit(_) => return Some(Shift(49)),
            AddMinus(_) => return Some(Shift(45)),
            And => return Some(Shift(44)),
            UnaryOp(_) => return Some(Shift(42)),
            OPERAND => return Some(Goto(47)),
            PRIMARY_EXPR => return Some(Goto(46)),
            UNARY_EXPR => return Some(Goto(134)),
            _ => return None,
        };
    }

    if state == 83 {
        match token {
            IntLit(_) => return Some(Shift(48)),
            Star => return Some(Shift(43)),
            Identifier(_) => return Some(Shift(50)),
            Lparen => return Some(Shift(51)),
            StringLit(_) => return Some(Shift(49)),
            AddMinus(_) => return Some(Shift(45)),
            And => return Some(Shift(44)),
            UnaryOp(_) => return Some(Shift(42)),
            OPERAND => return Some(Goto(47)),
            PRIMARY_EXPR => return Some(Goto(46)),
            UNARY_EXPR => return Some(Goto(135)),
            _ => return None,
        };
    }

    if state == 84 {
        match token {
            Star => return Some(Reduce(44)),
            Rbrace => return Some(Reduce(44)),
            Semicolon => return Some(Reduce(44)),
            LogOp(_) => return Some(Reduce(44)),
            RelOp(_) => return Some(Reduce(44)),
            AddOp(_) => return Some(Reduce(44)),
            AddMinus(_) => return Some(Reduce(44)),
            MulOp(_) => return Some(Reduce(44)),
            And => return Some(Reduce(44)),
            AssignOp => return Some(Reduce(44)),
            _ => return None,
        };
    }

    if state == 85 {
        match token {
            Star => return Some(Reduce(45)),
            Rbrace => return Some(Reduce(45)),
            Semicolon => return Some(Reduce(45)),
            LogOp(_) => return Some(Reduce(45)),
            RelOp(_) => return Some(Reduce(45)),
            AddOp(_) => return Some(Reduce(45)),
            AddMinus(_) => return Some(Reduce(45)),
            MulOp(_) => return Some(Reduce(45)),
            And => return Some(Reduce(45)),
            AssignOp => return Some(Reduce(45)),
            _ => return None,
        };
    }

    if state == 86 {
        match token {
            Star => return Some(Reduce(46)),
            Rbrace => return Some(Reduce(46)),
            Semicolon => return Some(Reduce(46)),
            LogOp(_) => return Some(Reduce(46)),
            RelOp(_) => return Some(Reduce(46)),
            AddOp(_) => return Some(Reduce(46)),
            AddMinus(_) => return Some(Reduce(46)),
            MulOp(_) => return Some(Reduce(46)),
            And => return Some(Reduce(46)),
            AssignOp => return Some(Reduce(46)),
            _ => return None,
        };
    }

    if state == 87 {
        match token {
            Star => return Some(Reduce(47)),
            Rbrace => return Some(Reduce(47)),
            Semicolon => return Some(Reduce(47)),
            LogOp(_) => return Some(Reduce(47)),
            RelOp(_) => return Some(Reduce(47)),
            AddOp(_) => return Some(Reduce(47)),
            AddMinus(_) => return Some(Reduce(47)),
            MulOp(_) => return Some(Reduce(47)),
            And => return Some(Reduce(47)),
            AssignOp => return Some(Reduce(47)),
            _ => return None,
        };
    }

    if state == 88 {
        match token {
            Lbracket => return Some(Reduce(24)),
            Star => return Some(Reduce(24)),
            Rbrace => return Some(Reduce(24)),
            Semicolon => return Some(Reduce(24)),
            Lparen => return Some(Reduce(24)),
            LogOp(_) => return Some(Reduce(24)),
            RelOp(_) => return Some(Reduce(24)),
            AddOp(_) => return Some(Reduce(24)),
            AddMinus(_) => return Some(Reduce(24)),
            MulOp(_) => return Some(Reduce(24)),
            And => return Some(Reduce(24)),
            AssignOp => return Some(Reduce(24)),
            _ => return None,
        };
    }

    if state == 89 {
        match token {
            Lbracket => return Some(Reduce(25)),
            Star => return Some(Reduce(25)),
            Rbrace => return Some(Reduce(25)),
            Semicolon => return Some(Reduce(25)),
            Lparen => return Some(Reduce(25)),
            LogOp(_) => return Some(Reduce(25)),
            RelOp(_) => return Some(Reduce(25)),
            AddOp(_) => return Some(Reduce(25)),
            AddMinus(_) => return Some(Reduce(25)),
            MulOp(_) => return Some(Reduce(25)),
            And => return Some(Reduce(25)),
            AssignOp => return Some(Reduce(25)),
            _ => return None,
        };
    }

    if state == 90 {
        match token {
            IntLit(_) => return Some(Shift(148)),
            Star => return Some(Shift(143)),
            Identifier(_) => return Some(Shift(150)),
            Lparen => return Some(Shift(151)),
            StringLit(_) => return Some(Shift(149)),
            AddMinus(_) => return Some(Shift(145)),
            And => return Some(Shift(144)),
            UnaryOp(_) => return Some(Shift(142)),
            OPERAND => return Some(Goto(147)),
            PRIMARY_EXPR => return Some(Goto(146)),
            EXPRESSION => return Some(Goto(136)),
            LOGICAL_EXPR => return Some(Goto(137)),
            RELATIONAL_EXPR => return Some(Goto(138)),
            ADDITIVE_EXPR => return Some(Goto(139)),
            MULTIPLICATIVE_EXPR => return Some(Goto(140)),
            UNARY_EXPR => return Some(Goto(141)),
            _ => return None,
        };
    }

    if state == 91 {
        match token {
            IntLit(_) => return Some(Shift(165)),
            Star => return Some(Shift(160)),
            Identifier(_) => return Some(Shift(167)),
            Lparen => return Some(Shift(168)),
            Rparen => return Some(Reduce(31)),
            StringLit(_) => return Some(Shift(166)),
            AddMinus(_) => return Some(Shift(162)),
            And => return Some(Shift(161)),
            UnaryOp(_) => return Some(Shift(159)),
            OPERAND => return Some(Goto(164)),
            PRIMARY_EXPR => return Some(Goto(163)),
            EXPRESSION_LIST => return Some(Goto(152)),
            EXPRESSION => return Some(Goto(153)),
            LOGICAL_EXPR => return Some(Goto(154)),
            RELATIONAL_EXPR => return Some(Goto(155)),
            ADDITIVE_EXPR => return Some(Goto(156)),
            MULTIPLICATIVE_EXPR => return Some(Goto(157)),
            UNARY_EXPR => return Some(Goto(158)),
            _ => return None,
        };
    }

    if state == 92 {
        match token {
            Rparen => return Some(Shift(169)),
            _ => return None,
        };
    }

    if state == 93 {
        match token {
            Rparen => return Some(Reduce(32)),
            LogOp(_) => return Some(Shift(170)),
            _ => return None,
        };
    }

    if state == 94 {
        match token {
            Rparen => return Some(Reduce(34)),
            LogOp(_) => return Some(Reduce(34)),
            RelOp(_) => return Some(Shift(171)),
            _ => return None,
        };
    }

    if state == 95 {
        match token {
            Rparen => return Some(Reduce(36)),
            LogOp(_) => return Some(Reduce(36)),
            RelOp(_) => return Some(Reduce(36)),
            AddOp(_) => return Some(Shift(172)),
            AddMinus(_) => return Some(Shift(173)),
            _ => return None,
        };
    }

    if state == 96 {
        match token {
            Star => return Some(Shift(175)),
            Rparen => return Some(Reduce(39)),
            LogOp(_) => return Some(Reduce(39)),
            RelOp(_) => return Some(Reduce(39)),
            AddOp(_) => return Some(Reduce(39)),
            AddMinus(_) => return Some(Reduce(39)),
            MulOp(_) => return Some(Shift(174)),
            And => return Some(Shift(176)),
            _ => return None,
        };
    }

    if state == 97 {
        match token {
            Star => return Some(Reduce(43)),
            Rparen => return Some(Reduce(43)),
            LogOp(_) => return Some(Reduce(43)),
            RelOp(_) => return Some(Reduce(43)),
            AddOp(_) => return Some(Reduce(43)),
            AddMinus(_) => return Some(Reduce(43)),
            MulOp(_) => return Some(Reduce(43)),
            And => return Some(Reduce(43)),
            _ => return None,
        };
    }

    if state == 98 {
        match token {
            IntLit(_) => return Some(Shift(104)),
            Star => return Some(Shift(99)),
            Identifier(_) => return Some(Shift(106)),
            Lparen => return Some(Shift(107)),
            StringLit(_) => return Some(Shift(105)),
            AddMinus(_) => return Some(Shift(101)),
            And => return Some(Shift(100)),
            UnaryOp(_) => return Some(Shift(98)),
            OPERAND => return Some(Goto(103)),
            PRIMARY_EXPR => return Some(Goto(102)),
            UNARY_EXPR => return Some(Goto(177)),
            _ => return None,
        };
    }

    if state == 99 {
        match token {
            IntLit(_) => return Some(Shift(104)),
            Star => return Some(Shift(99)),
            Identifier(_) => return Some(Shift(106)),
            Lparen => return Some(Shift(107)),
            StringLit(_) => return Some(Shift(105)),
            AddMinus(_) => return Some(Shift(101)),
            And => return Some(Shift(100)),
            UnaryOp(_) => return Some(Shift(98)),
            OPERAND => return Some(Goto(103)),
            PRIMARY_EXPR => return Some(Goto(102)),
            UNARY_EXPR => return Some(Goto(178)),
            _ => return None,
        };
    }

    if state == 100 {
        match token {
            IntLit(_) => return Some(Shift(104)),
            Star => return Some(Shift(99)),
            Identifier(_) => return Some(Shift(106)),
            Lparen => return Some(Shift(107)),
            StringLit(_) => return Some(Shift(105)),
            AddMinus(_) => return Some(Shift(101)),
            And => return Some(Shift(100)),
            UnaryOp(_) => return Some(Shift(98)),
            OPERAND => return Some(Goto(103)),
            PRIMARY_EXPR => return Some(Goto(102)),
            UNARY_EXPR => return Some(Goto(179)),
            _ => return None,
        };
    }

    if state == 101 {
        match token {
            IntLit(_) => return Some(Shift(104)),
            Star => return Some(Shift(99)),
            Identifier(_) => return Some(Shift(106)),
            Lparen => return Some(Shift(107)),
            StringLit(_) => return Some(Shift(105)),
            AddMinus(_) => return Some(Shift(101)),
            And => return Some(Shift(100)),
            UnaryOp(_) => return Some(Shift(98)),
            OPERAND => return Some(Goto(103)),
            PRIMARY_EXPR => return Some(Goto(102)),
            UNARY_EXPR => return Some(Goto(180)),
            _ => return None,
        };
    }

    if state == 102 {
        match token {
            Lbracket => return Some(Shift(183)),
            Star => return Some(Reduce(48)),
            Lparen => return Some(Shift(184)),
            Rparen => return Some(Reduce(48)),
            LogOp(_) => return Some(Reduce(48)),
            RelOp(_) => return Some(Reduce(48)),
            AddOp(_) => return Some(Reduce(48)),
            AddMinus(_) => return Some(Reduce(48)),
            MulOp(_) => return Some(Reduce(48)),
            And => return Some(Reduce(48)),
            INDEX => return Some(Goto(181)),
            ARGUMENTS => return Some(Goto(182)),
            _ => return None,
        };
    }

    if state == 103 {
        match token {
            Lbracket => return Some(Reduce(26)),
            Star => return Some(Reduce(26)),
            Lparen => return Some(Reduce(26)),
            Rparen => return Some(Reduce(26)),
            LogOp(_) => return Some(Reduce(26)),
            RelOp(_) => return Some(Reduce(26)),
            AddOp(_) => return Some(Reduce(26)),
            AddMinus(_) => return Some(Reduce(26)),
            MulOp(_) => return Some(Reduce(26)),
            And => return Some(Reduce(26)),
            _ => return None,
        };
    }

    if state == 104 {
        match token {
            Lbracket => return Some(Reduce(20)),
            Star => return Some(Reduce(20)),
            Lparen => return Some(Reduce(20)),
            Rparen => return Some(Reduce(20)),
            LogOp(_) => return Some(Reduce(20)),
            RelOp(_) => return Some(Reduce(20)),
            AddOp(_) => return Some(Reduce(20)),
            AddMinus(_) => return Some(Reduce(20)),
            MulOp(_) => return Some(Reduce(20)),
            And => return Some(Reduce(20)),
            _ => return None,
        };
    }

    if state == 105 {
        match token {
            Lbracket => return Some(Reduce(21)),
            Star => return Some(Reduce(21)),
            Lparen => return Some(Reduce(21)),
            Rparen => return Some(Reduce(21)),
            LogOp(_) => return Some(Reduce(21)),
            RelOp(_) => return Some(Reduce(21)),
            AddOp(_) => return Some(Reduce(21)),
            AddMinus(_) => return Some(Reduce(21)),
            MulOp(_) => return Some(Reduce(21)),
            And => return Some(Reduce(21)),
            _ => return None,
        };
    }

    if state == 106 {
        match token {
            Lbracket => return Some(Reduce(22)),
            Star => return Some(Reduce(22)),
            Lparen => return Some(Reduce(22)),
            Rparen => return Some(Reduce(22)),
            LogOp(_) => return Some(Reduce(22)),
            RelOp(_) => return Some(Reduce(22)),
            AddOp(_) => return Some(Reduce(22)),
            AddMinus(_) => return Some(Reduce(22)),
            MulOp(_) => return Some(Reduce(22)),
            And => return Some(Reduce(22)),
            _ => return None,
        };
    }

    if state == 107 {
        match token {
            IntLit(_) => return Some(Shift(104)),
            Star => return Some(Shift(99)),
            Identifier(_) => return Some(Shift(106)),
            Lparen => return Some(Shift(107)),
            StringLit(_) => return Some(Shift(105)),
            AddMinus(_) => return Some(Shift(101)),
            And => return Some(Shift(100)),
            UnaryOp(_) => return Some(Shift(98)),
            OPERAND => return Some(Goto(103)),
            PRIMARY_EXPR => return Some(Goto(102)),
            EXPRESSION => return Some(Goto(185)),
            LOGICAL_EXPR => return Some(Goto(93)),
            RELATIONAL_EXPR => return Some(Goto(94)),
            ADDITIVE_EXPR => return Some(Goto(95)),
            MULTIPLICATIVE_EXPR => return Some(Goto(96)),
            UNARY_EXPR => return Some(Goto(97)),
            _ => return None,
        };
    }

    if state == 108 {
        match token {
            Rparen => return Some(Reduce(16)),
            _ => return None,
        };
    }

    if state == 109 {
        match token {
            Rbrace => return Some(Reduce(10)),
            _ => return None,
        };
    }

    if state == 110 {
        match token {
            Rbrace => return Some(Reduce(57)),
            Semicolon => return Some(Reduce(57)),
            _ => return None,
        };
    }

    if state == 111 {
        match token {
            IntLit(_) => return Some(Shift(71)),
            Star => return Some(Shift(66)),
            Identifier(_) => return Some(Shift(73)),
            Lparen => return Some(Shift(74)),
            StringLit(_) => return Some(Shift(72)),
            AddMinus(_) => return Some(Shift(68)),
            And => return Some(Shift(67)),
            UnaryOp(_) => return Some(Shift(65)),
            OPERAND => return Some(Goto(70)),
            PRIMARY_EXPR => return Some(Goto(69)),
            RELATIONAL_EXPR => return Some(Goto(186)),
            ADDITIVE_EXPR => return Some(Goto(62)),
            MULTIPLICATIVE_EXPR => return Some(Goto(63)),
            UNARY_EXPR => return Some(Goto(64)),
            _ => return None,
        };
    }

    if state == 112 {
        match token {
            IntLit(_) => return Some(Shift(71)),
            Star => return Some(Shift(66)),
            Identifier(_) => return Some(Shift(73)),
            Lparen => return Some(Shift(74)),
            StringLit(_) => return Some(Shift(72)),
            AddMinus(_) => return Some(Shift(68)),
            And => return Some(Shift(67)),
            UnaryOp(_) => return Some(Shift(65)),
            OPERAND => return Some(Goto(70)),
            PRIMARY_EXPR => return Some(Goto(69)),
            ADDITIVE_EXPR => return Some(Goto(187)),
            MULTIPLICATIVE_EXPR => return Some(Goto(63)),
            UNARY_EXPR => return Some(Goto(64)),
            _ => return None,
        };
    }

    if state == 113 {
        match token {
            IntLit(_) => return Some(Shift(71)),
            Star => return Some(Shift(66)),
            Identifier(_) => return Some(Shift(73)),
            Lparen => return Some(Shift(74)),
            StringLit(_) => return Some(Shift(72)),
            AddMinus(_) => return Some(Shift(68)),
            And => return Some(Shift(67)),
            UnaryOp(_) => return Some(Shift(65)),
            OPERAND => return Some(Goto(70)),
            PRIMARY_EXPR => return Some(Goto(69)),
            MULTIPLICATIVE_EXPR => return Some(Goto(188)),
            UNARY_EXPR => return Some(Goto(64)),
            _ => return None,
        };
    }

    if state == 114 {
        match token {
            IntLit(_) => return Some(Shift(71)),
            Star => return Some(Shift(66)),
            Identifier(_) => return Some(Shift(73)),
            Lparen => return Some(Shift(74)),
            StringLit(_) => return Some(Shift(72)),
            AddMinus(_) => return Some(Shift(68)),
            And => return Some(Shift(67)),
            UnaryOp(_) => return Some(Shift(65)),
            OPERAND => return Some(Goto(70)),
            PRIMARY_EXPR => return Some(Goto(69)),
            MULTIPLICATIVE_EXPR => return Some(Goto(189)),
            UNARY_EXPR => return Some(Goto(64)),
            _ => return None,
        };
    }

    if state == 115 {
        match token {
            IntLit(_) => return Some(Shift(71)),
            Star => return Some(Shift(66)),
            Identifier(_) => return Some(Shift(73)),
            Lparen => return Some(Shift(74)),
            StringLit(_) => return Some(Shift(72)),
            AddMinus(_) => return Some(Shift(68)),
            And => return Some(Shift(67)),
            UnaryOp(_) => return Some(Shift(65)),
            OPERAND => return Some(Goto(70)),
            PRIMARY_EXPR => return Some(Goto(69)),
            UNARY_EXPR => return Some(Goto(190)),
            _ => return None,
        };
    }

    if state == 116 {
        match token {
            IntLit(_) => return Some(Shift(71)),
            Star => return Some(Shift(66)),
            Identifier(_) => return Some(Shift(73)),
            Lparen => return Some(Shift(74)),
            StringLit(_) => return Some(Shift(72)),
            AddMinus(_) => return Some(Shift(68)),
            And => return Some(Shift(67)),
            UnaryOp(_) => return Some(Shift(65)),
            OPERAND => return Some(Goto(70)),
            PRIMARY_EXPR => return Some(Goto(69)),
            UNARY_EXPR => return Some(Goto(191)),
            _ => return None,
        };
    }

    if state == 117 {
        match token {
            IntLit(_) => return Some(Shift(71)),
            Star => return Some(Shift(66)),
            Identifier(_) => return Some(Shift(73)),
            Lparen => return Some(Shift(74)),
            StringLit(_) => return Some(Shift(72)),
            AddMinus(_) => return Some(Shift(68)),
            And => return Some(Shift(67)),
            UnaryOp(_) => return Some(Shift(65)),
            OPERAND => return Some(Goto(70)),
            PRIMARY_EXPR => return Some(Goto(69)),
            UNARY_EXPR => return Some(Goto(192)),
            _ => return None,
        };
    }

    if state == 118 {
        match token {
            Star => return Some(Reduce(44)),
            Rbrace => return Some(Reduce(44)),
            Semicolon => return Some(Reduce(44)),
            LogOp(_) => return Some(Reduce(44)),
            RelOp(_) => return Some(Reduce(44)),
            AddOp(_) => return Some(Reduce(44)),
            AddMinus(_) => return Some(Reduce(44)),
            MulOp(_) => return Some(Reduce(44)),
            And => return Some(Reduce(44)),
            _ => return None,
        };
    }

    if state == 119 {
        match token {
            Star => return Some(Reduce(45)),
            Rbrace => return Some(Reduce(45)),
            Semicolon => return Some(Reduce(45)),
            LogOp(_) => return Some(Reduce(45)),
            RelOp(_) => return Some(Reduce(45)),
            AddOp(_) => return Some(Reduce(45)),
            AddMinus(_) => return Some(Reduce(45)),
            MulOp(_) => return Some(Reduce(45)),
            And => return Some(Reduce(45)),
            _ => return None,
        };
    }

    if state == 120 {
        match token {
            Star => return Some(Reduce(46)),
            Rbrace => return Some(Reduce(46)),
            Semicolon => return Some(Reduce(46)),
            LogOp(_) => return Some(Reduce(46)),
            RelOp(_) => return Some(Reduce(46)),
            AddOp(_) => return Some(Reduce(46)),
            AddMinus(_) => return Some(Reduce(46)),
            MulOp(_) => return Some(Reduce(46)),
            And => return Some(Reduce(46)),
            _ => return None,
        };
    }

    if state == 121 {
        match token {
            Star => return Some(Reduce(47)),
            Rbrace => return Some(Reduce(47)),
            Semicolon => return Some(Reduce(47)),
            LogOp(_) => return Some(Reduce(47)),
            RelOp(_) => return Some(Reduce(47)),
            AddOp(_) => return Some(Reduce(47)),
            AddMinus(_) => return Some(Reduce(47)),
            MulOp(_) => return Some(Reduce(47)),
            And => return Some(Reduce(47)),
            _ => return None,
        };
    }

    if state == 122 {
        match token {
            Lbracket => return Some(Reduce(24)),
            Star => return Some(Reduce(24)),
            Rbrace => return Some(Reduce(24)),
            Semicolon => return Some(Reduce(24)),
            Lparen => return Some(Reduce(24)),
            LogOp(_) => return Some(Reduce(24)),
            RelOp(_) => return Some(Reduce(24)),
            AddOp(_) => return Some(Reduce(24)),
            AddMinus(_) => return Some(Reduce(24)),
            MulOp(_) => return Some(Reduce(24)),
            And => return Some(Reduce(24)),
            _ => return None,
        };
    }

    if state == 123 {
        match token {
            Lbracket => return Some(Reduce(25)),
            Star => return Some(Reduce(25)),
            Rbrace => return Some(Reduce(25)),
            Semicolon => return Some(Reduce(25)),
            Lparen => return Some(Reduce(25)),
            LogOp(_) => return Some(Reduce(25)),
            RelOp(_) => return Some(Reduce(25)),
            AddOp(_) => return Some(Reduce(25)),
            AddMinus(_) => return Some(Reduce(25)),
            MulOp(_) => return Some(Reduce(25)),
            And => return Some(Reduce(25)),
            _ => return None,
        };
    }

    if state == 124 {
        match token {
            IntLit(_) => return Some(Shift(148)),
            Star => return Some(Shift(143)),
            Identifier(_) => return Some(Shift(150)),
            Lparen => return Some(Shift(151)),
            StringLit(_) => return Some(Shift(149)),
            AddMinus(_) => return Some(Shift(145)),
            And => return Some(Shift(144)),
            UnaryOp(_) => return Some(Shift(142)),
            OPERAND => return Some(Goto(147)),
            PRIMARY_EXPR => return Some(Goto(146)),
            EXPRESSION => return Some(Goto(193)),
            LOGICAL_EXPR => return Some(Goto(137)),
            RELATIONAL_EXPR => return Some(Goto(138)),
            ADDITIVE_EXPR => return Some(Goto(139)),
            MULTIPLICATIVE_EXPR => return Some(Goto(140)),
            UNARY_EXPR => return Some(Goto(141)),
            _ => return None,
        };
    }

    if state == 125 {
        match token {
            IntLit(_) => return Some(Shift(165)),
            Star => return Some(Shift(160)),
            Identifier(_) => return Some(Shift(167)),
            Lparen => return Some(Shift(168)),
            Rparen => return Some(Reduce(31)),
            StringLit(_) => return Some(Shift(166)),
            AddMinus(_) => return Some(Shift(162)),
            And => return Some(Shift(161)),
            UnaryOp(_) => return Some(Shift(159)),
            OPERAND => return Some(Goto(164)),
            PRIMARY_EXPR => return Some(Goto(163)),
            EXPRESSION_LIST => return Some(Goto(194)),
            EXPRESSION => return Some(Goto(153)),
            LOGICAL_EXPR => return Some(Goto(154)),
            RELATIONAL_EXPR => return Some(Goto(155)),
            ADDITIVE_EXPR => return Some(Goto(156)),
            MULTIPLICATIVE_EXPR => return Some(Goto(157)),
            UNARY_EXPR => return Some(Goto(158)),
            _ => return None,
        };
    }

    if state == 126 {
        match token {
            Rparen => return Some(Shift(195)),
            _ => return None,
        };
    }

    if state == 127 {
        match token {
            Rparen => return Some(Shift(196)),
            _ => return None,
        };
    }

    if state == 128 {
        match token {
            Rparen => return Some(Shift(197)),
            _ => return None,
        };
    }

    if state == 129 {
        match token {
            Rbrace => return Some(Reduce(33)),
            Semicolon => return Some(Reduce(33)),
            LogOp(_) => return Some(Reduce(33)),
            RelOp(_) => return Some(Shift(78)),
            AssignOp => return Some(Reduce(33)),
            _ => return None,
        };
    }

    if state == 130 {
        match token {
            Rbrace => return Some(Reduce(35)),
            Semicolon => return Some(Reduce(35)),
            LogOp(_) => return Some(Reduce(35)),
            RelOp(_) => return Some(Reduce(35)),
            AddOp(_) => return Some(Shift(79)),
            AddMinus(_) => return Some(Shift(80)),
            AssignOp => return Some(Reduce(35)),
            _ => return None,
        };
    }

    if state == 131 {
        match token {
            Star => return Some(Shift(82)),
            Rbrace => return Some(Reduce(37)),
            Semicolon => return Some(Reduce(37)),
            LogOp(_) => return Some(Reduce(37)),
            RelOp(_) => return Some(Reduce(37)),
            AddOp(_) => return Some(Reduce(37)),
            AddMinus(_) => return Some(Reduce(37)),
            MulOp(_) => return Some(Shift(81)),
            And => return Some(Shift(83)),
            AssignOp => return Some(Reduce(37)),
            _ => return None,
        };
    }

    if state == 132 {
        match token {
            Star => return Some(Shift(82)),
            Rbrace => return Some(Reduce(38)),
            Semicolon => return Some(Reduce(38)),
            LogOp(_) => return Some(Reduce(38)),
            RelOp(_) => return Some(Reduce(38)),
            AddOp(_) => return Some(Reduce(38)),
            AddMinus(_) => return Some(Reduce(38)),
            MulOp(_) => return Some(Shift(81)),
            And => return Some(Shift(83)),
            AssignOp => return Some(Reduce(38)),
            _ => return None,
        };
    }

    if state == 133 {
        match token {
            Star => return Some(Reduce(40)),
            Rbrace => return Some(Reduce(40)),
            Semicolon => return Some(Reduce(40)),
            LogOp(_) => return Some(Reduce(40)),
            RelOp(_) => return Some(Reduce(40)),
            AddOp(_) => return Some(Reduce(40)),
            AddMinus(_) => return Some(Reduce(40)),
            MulOp(_) => return Some(Reduce(40)),
            And => return Some(Reduce(40)),
            AssignOp => return Some(Reduce(40)),
            _ => return None,
        };
    }

    if state == 134 {
        match token {
            Star => return Some(Reduce(41)),
            Rbrace => return Some(Reduce(41)),
            Semicolon => return Some(Reduce(41)),
            LogOp(_) => return Some(Reduce(41)),
            RelOp(_) => return Some(Reduce(41)),
            AddOp(_) => return Some(Reduce(41)),
            AddMinus(_) => return Some(Reduce(41)),
            MulOp(_) => return Some(Reduce(41)),
            And => return Some(Reduce(41)),
            AssignOp => return Some(Reduce(41)),
            _ => return None,
        };
    }

    if state == 135 {
        match token {
            Star => return Some(Reduce(42)),
            Rbrace => return Some(Reduce(42)),
            Semicolon => return Some(Reduce(42)),
            LogOp(_) => return Some(Reduce(42)),
            RelOp(_) => return Some(Reduce(42)),
            AddOp(_) => return Some(Reduce(42)),
            AddMinus(_) => return Some(Reduce(42)),
            MulOp(_) => return Some(Reduce(42)),
            And => return Some(Reduce(42)),
            AssignOp => return Some(Reduce(42)),
            _ => return None,
        };
    }

    if state == 136 {
        match token {
            Rbracket => return Some(Shift(198)),
            _ => return None,
        };
    }

    if state == 137 {
        match token {
            Rbracket => return Some(Reduce(32)),
            LogOp(_) => return Some(Shift(199)),
            _ => return None,
        };
    }

    if state == 138 {
        match token {
            Rbracket => return Some(Reduce(34)),
            LogOp(_) => return Some(Reduce(34)),
            RelOp(_) => return Some(Shift(200)),
            _ => return None,
        };
    }

    if state == 139 {
        match token {
            Rbracket => return Some(Reduce(36)),
            LogOp(_) => return Some(Reduce(36)),
            RelOp(_) => return Some(Reduce(36)),
            AddOp(_) => return Some(Shift(201)),
            AddMinus(_) => return Some(Shift(202)),
            _ => return None,
        };
    }

    if state == 140 {
        match token {
            Rbracket => return Some(Reduce(39)),
            Star => return Some(Shift(204)),
            LogOp(_) => return Some(Reduce(39)),
            RelOp(_) => return Some(Reduce(39)),
            AddOp(_) => return Some(Reduce(39)),
            AddMinus(_) => return Some(Reduce(39)),
            MulOp(_) => return Some(Shift(203)),
            And => return Some(Shift(205)),
            _ => return None,
        };
    }

    if state == 141 {
        match token {
            Rbracket => return Some(Reduce(43)),
            Star => return Some(Reduce(43)),
            LogOp(_) => return Some(Reduce(43)),
            RelOp(_) => return Some(Reduce(43)),
            AddOp(_) => return Some(Reduce(43)),
            AddMinus(_) => return Some(Reduce(43)),
            MulOp(_) => return Some(Reduce(43)),
            And => return Some(Reduce(43)),
            _ => return None,
        };
    }

    if state == 142 {
        match token {
            IntLit(_) => return Some(Shift(148)),
            Star => return Some(Shift(143)),
            Identifier(_) => return Some(Shift(150)),
            Lparen => return Some(Shift(151)),
            StringLit(_) => return Some(Shift(149)),
            AddMinus(_) => return Some(Shift(145)),
            And => return Some(Shift(144)),
            UnaryOp(_) => return Some(Shift(142)),
            OPERAND => return Some(Goto(147)),
            PRIMARY_EXPR => return Some(Goto(146)),
            UNARY_EXPR => return Some(Goto(206)),
            _ => return None,
        };
    }

    if state == 143 {
        match token {
            IntLit(_) => return Some(Shift(148)),
            Star => return Some(Shift(143)),
            Identifier(_) => return Some(Shift(150)),
            Lparen => return Some(Shift(151)),
            StringLit(_) => return Some(Shift(149)),
            AddMinus(_) => return Some(Shift(145)),
            And => return Some(Shift(144)),
            UnaryOp(_) => return Some(Shift(142)),
            OPERAND => return Some(Goto(147)),
            PRIMARY_EXPR => return Some(Goto(146)),
            UNARY_EXPR => return Some(Goto(207)),
            _ => return None,
        };
    }

    if state == 144 {
        match token {
            IntLit(_) => return Some(Shift(148)),
            Star => return Some(Shift(143)),
            Identifier(_) => return Some(Shift(150)),
            Lparen => return Some(Shift(151)),
            StringLit(_) => return Some(Shift(149)),
            AddMinus(_) => return Some(Shift(145)),
            And => return Some(Shift(144)),
            UnaryOp(_) => return Some(Shift(142)),
            OPERAND => return Some(Goto(147)),
            PRIMARY_EXPR => return Some(Goto(146)),
            UNARY_EXPR => return Some(Goto(208)),
            _ => return None,
        };
    }

    if state == 145 {
        match token {
            IntLit(_) => return Some(Shift(148)),
            Star => return Some(Shift(143)),
            Identifier(_) => return Some(Shift(150)),
            Lparen => return Some(Shift(151)),
            StringLit(_) => return Some(Shift(149)),
            AddMinus(_) => return Some(Shift(145)),
            And => return Some(Shift(144)),
            UnaryOp(_) => return Some(Shift(142)),
            OPERAND => return Some(Goto(147)),
            PRIMARY_EXPR => return Some(Goto(146)),
            UNARY_EXPR => return Some(Goto(209)),
            _ => return None,
        };
    }

    if state == 146 {
        match token {
            Lbracket => return Some(Shift(212)),
            Rbracket => return Some(Reduce(48)),
            Star => return Some(Reduce(48)),
            Lparen => return Some(Shift(213)),
            LogOp(_) => return Some(Reduce(48)),
            RelOp(_) => return Some(Reduce(48)),
            AddOp(_) => return Some(Reduce(48)),
            AddMinus(_) => return Some(Reduce(48)),
            MulOp(_) => return Some(Reduce(48)),
            And => return Some(Reduce(48)),
            INDEX => return Some(Goto(210)),
            ARGUMENTS => return Some(Goto(211)),
            _ => return None,
        };
    }

    if state == 147 {
        match token {
            Lbracket => return Some(Reduce(26)),
            Rbracket => return Some(Reduce(26)),
            Star => return Some(Reduce(26)),
            Lparen => return Some(Reduce(26)),
            LogOp(_) => return Some(Reduce(26)),
            RelOp(_) => return Some(Reduce(26)),
            AddOp(_) => return Some(Reduce(26)),
            AddMinus(_) => return Some(Reduce(26)),
            MulOp(_) => return Some(Reduce(26)),
            And => return Some(Reduce(26)),
            _ => return None,
        };
    }

    if state == 148 {
        match token {
            Lbracket => return Some(Reduce(20)),
            Rbracket => return Some(Reduce(20)),
            Star => return Some(Reduce(20)),
            Lparen => return Some(Reduce(20)),
            LogOp(_) => return Some(Reduce(20)),
            RelOp(_) => return Some(Reduce(20)),
            AddOp(_) => return Some(Reduce(20)),
            AddMinus(_) => return Some(Reduce(20)),
            MulOp(_) => return Some(Reduce(20)),
            And => return Some(Reduce(20)),
            _ => return None,
        };
    }

    if state == 149 {
        match token {
            Lbracket => return Some(Reduce(21)),
            Rbracket => return Some(Reduce(21)),
            Star => return Some(Reduce(21)),
            Lparen => return Some(Reduce(21)),
            LogOp(_) => return Some(Reduce(21)),
            RelOp(_) => return Some(Reduce(21)),
            AddOp(_) => return Some(Reduce(21)),
            AddMinus(_) => return Some(Reduce(21)),
            MulOp(_) => return Some(Reduce(21)),
            And => return Some(Reduce(21)),
            _ => return None,
        };
    }

    if state == 150 {
        match token {
            Lbracket => return Some(Reduce(22)),
            Rbracket => return Some(Reduce(22)),
            Star => return Some(Reduce(22)),
            Lparen => return Some(Reduce(22)),
            LogOp(_) => return Some(Reduce(22)),
            RelOp(_) => return Some(Reduce(22)),
            AddOp(_) => return Some(Reduce(22)),
            AddMinus(_) => return Some(Reduce(22)),
            MulOp(_) => return Some(Reduce(22)),
            And => return Some(Reduce(22)),
            _ => return None,
        };
    }

    if state == 151 {
        match token {
            IntLit(_) => return Some(Shift(104)),
            Star => return Some(Shift(99)),
            Identifier(_) => return Some(Shift(106)),
            Lparen => return Some(Shift(107)),
            StringLit(_) => return Some(Shift(105)),
            AddMinus(_) => return Some(Shift(101)),
            And => return Some(Shift(100)),
            UnaryOp(_) => return Some(Shift(98)),
            OPERAND => return Some(Goto(103)),
            PRIMARY_EXPR => return Some(Goto(102)),
            EXPRESSION => return Some(Goto(214)),
            LOGICAL_EXPR => return Some(Goto(93)),
            RELATIONAL_EXPR => return Some(Goto(94)),
            ADDITIVE_EXPR => return Some(Goto(95)),
            MULTIPLICATIVE_EXPR => return Some(Goto(96)),
            UNARY_EXPR => return Some(Goto(97)),
            _ => return None,
        };
    }

    if state == 152 {
        match token {
            Rparen => return Some(Shift(215)),
            _ => return None,
        };
    }

    if state == 153 {
        match token {
            Rparen => return Some(Reduce(30)),
            Comma => return Some(Shift(216)),
            _ => return None,
        };
    }

    if state == 154 {
        match token {
            Rparen => return Some(Reduce(32)),
            Comma => return Some(Reduce(32)),
            LogOp(_) => return Some(Shift(217)),
            _ => return None,
        };
    }

    if state == 155 {
        match token {
            Rparen => return Some(Reduce(34)),
            Comma => return Some(Reduce(34)),
            LogOp(_) => return Some(Reduce(34)),
            RelOp(_) => return Some(Shift(218)),
            _ => return None,
        };
    }

    if state == 156 {
        match token {
            Rparen => return Some(Reduce(36)),
            Comma => return Some(Reduce(36)),
            LogOp(_) => return Some(Reduce(36)),
            RelOp(_) => return Some(Reduce(36)),
            AddOp(_) => return Some(Shift(219)),
            AddMinus(_) => return Some(Shift(220)),
            _ => return None,
        };
    }

    if state == 157 {
        match token {
            Star => return Some(Shift(222)),
            Rparen => return Some(Reduce(39)),
            Comma => return Some(Reduce(39)),
            LogOp(_) => return Some(Reduce(39)),
            RelOp(_) => return Some(Reduce(39)),
            AddOp(_) => return Some(Reduce(39)),
            AddMinus(_) => return Some(Reduce(39)),
            MulOp(_) => return Some(Shift(221)),
            And => return Some(Shift(223)),
            _ => return None,
        };
    }

    if state == 158 {
        match token {
            Star => return Some(Reduce(43)),
            Rparen => return Some(Reduce(43)),
            Comma => return Some(Reduce(43)),
            LogOp(_) => return Some(Reduce(43)),
            RelOp(_) => return Some(Reduce(43)),
            AddOp(_) => return Some(Reduce(43)),
            AddMinus(_) => return Some(Reduce(43)),
            MulOp(_) => return Some(Reduce(43)),
            And => return Some(Reduce(43)),
            _ => return None,
        };
    }

    if state == 159 {
        match token {
            IntLit(_) => return Some(Shift(165)),
            Star => return Some(Shift(160)),
            Identifier(_) => return Some(Shift(167)),
            Lparen => return Some(Shift(168)),
            StringLit(_) => return Some(Shift(166)),
            AddMinus(_) => return Some(Shift(162)),
            And => return Some(Shift(161)),
            UnaryOp(_) => return Some(Shift(159)),
            OPERAND => return Some(Goto(164)),
            PRIMARY_EXPR => return Some(Goto(163)),
            UNARY_EXPR => return Some(Goto(224)),
            _ => return None,
        };
    }

    if state == 160 {
        match token {
            IntLit(_) => return Some(Shift(165)),
            Star => return Some(Shift(160)),
            Identifier(_) => return Some(Shift(167)),
            Lparen => return Some(Shift(168)),
            StringLit(_) => return Some(Shift(166)),
            AddMinus(_) => return Some(Shift(162)),
            And => return Some(Shift(161)),
            UnaryOp(_) => return Some(Shift(159)),
            OPERAND => return Some(Goto(164)),
            PRIMARY_EXPR => return Some(Goto(163)),
            UNARY_EXPR => return Some(Goto(225)),
            _ => return None,
        };
    }

    if state == 161 {
        match token {
            IntLit(_) => return Some(Shift(165)),
            Star => return Some(Shift(160)),
            Identifier(_) => return Some(Shift(167)),
            Lparen => return Some(Shift(168)),
            StringLit(_) => return Some(Shift(166)),
            AddMinus(_) => return Some(Shift(162)),
            And => return Some(Shift(161)),
            UnaryOp(_) => return Some(Shift(159)),
            OPERAND => return Some(Goto(164)),
            PRIMARY_EXPR => return Some(Goto(163)),
            UNARY_EXPR => return Some(Goto(226)),
            _ => return None,
        };
    }

    if state == 162 {
        match token {
            IntLit(_) => return Some(Shift(165)),
            Star => return Some(Shift(160)),
            Identifier(_) => return Some(Shift(167)),
            Lparen => return Some(Shift(168)),
            StringLit(_) => return Some(Shift(166)),
            AddMinus(_) => return Some(Shift(162)),
            And => return Some(Shift(161)),
            UnaryOp(_) => return Some(Shift(159)),
            OPERAND => return Some(Goto(164)),
            PRIMARY_EXPR => return Some(Goto(163)),
            UNARY_EXPR => return Some(Goto(227)),
            _ => return None,
        };
    }

    if state == 163 {
        match token {
            Lbracket => return Some(Shift(230)),
            Star => return Some(Reduce(48)),
            Lparen => return Some(Shift(231)),
            Rparen => return Some(Reduce(48)),
            Comma => return Some(Reduce(48)),
            LogOp(_) => return Some(Reduce(48)),
            RelOp(_) => return Some(Reduce(48)),
            AddOp(_) => return Some(Reduce(48)),
            AddMinus(_) => return Some(Reduce(48)),
            MulOp(_) => return Some(Reduce(48)),
            And => return Some(Reduce(48)),
            INDEX => return Some(Goto(228)),
            ARGUMENTS => return Some(Goto(229)),
            _ => return None,
        };
    }

    if state == 164 {
        match token {
            Lbracket => return Some(Reduce(26)),
            Star => return Some(Reduce(26)),
            Lparen => return Some(Reduce(26)),
            Rparen => return Some(Reduce(26)),
            Comma => return Some(Reduce(26)),
            LogOp(_) => return Some(Reduce(26)),
            RelOp(_) => return Some(Reduce(26)),
            AddOp(_) => return Some(Reduce(26)),
            AddMinus(_) => return Some(Reduce(26)),
            MulOp(_) => return Some(Reduce(26)),
            And => return Some(Reduce(26)),
            _ => return None,
        };
    }

    if state == 165 {
        match token {
            Lbracket => return Some(Reduce(20)),
            Star => return Some(Reduce(20)),
            Lparen => return Some(Reduce(20)),
            Rparen => return Some(Reduce(20)),
            Comma => return Some(Reduce(20)),
            LogOp(_) => return Some(Reduce(20)),
            RelOp(_) => return Some(Reduce(20)),
            AddOp(_) => return Some(Reduce(20)),
            AddMinus(_) => return Some(Reduce(20)),
            MulOp(_) => return Some(Reduce(20)),
            And => return Some(Reduce(20)),
            _ => return None,
        };
    }

    if state == 166 {
        match token {
            Lbracket => return Some(Reduce(21)),
            Star => return Some(Reduce(21)),
            Lparen => return Some(Reduce(21)),
            Rparen => return Some(Reduce(21)),
            Comma => return Some(Reduce(21)),
            LogOp(_) => return Some(Reduce(21)),
            RelOp(_) => return Some(Reduce(21)),
            AddOp(_) => return Some(Reduce(21)),
            AddMinus(_) => return Some(Reduce(21)),
            MulOp(_) => return Some(Reduce(21)),
            And => return Some(Reduce(21)),
            _ => return None,
        };
    }

    if state == 167 {
        match token {
            Lbracket => return Some(Reduce(22)),
            Star => return Some(Reduce(22)),
            Lparen => return Some(Reduce(22)),
            Rparen => return Some(Reduce(22)),
            Comma => return Some(Reduce(22)),
            LogOp(_) => return Some(Reduce(22)),
            RelOp(_) => return Some(Reduce(22)),
            AddOp(_) => return Some(Reduce(22)),
            AddMinus(_) => return Some(Reduce(22)),
            MulOp(_) => return Some(Reduce(22)),
            And => return Some(Reduce(22)),
            _ => return None,
        };
    }

    if state == 168 {
        match token {
            IntLit(_) => return Some(Shift(104)),
            Star => return Some(Shift(99)),
            Identifier(_) => return Some(Shift(106)),
            Lparen => return Some(Shift(107)),
            StringLit(_) => return Some(Shift(105)),
            AddMinus(_) => return Some(Shift(101)),
            And => return Some(Shift(100)),
            UnaryOp(_) => return Some(Shift(98)),
            OPERAND => return Some(Goto(103)),
            PRIMARY_EXPR => return Some(Goto(102)),
            EXPRESSION => return Some(Goto(232)),
            LOGICAL_EXPR => return Some(Goto(93)),
            RELATIONAL_EXPR => return Some(Goto(94)),
            ADDITIVE_EXPR => return Some(Goto(95)),
            MULTIPLICATIVE_EXPR => return Some(Goto(96)),
            UNARY_EXPR => return Some(Goto(97)),
            _ => return None,
        };
    }

    if state == 169 {
        match token {
            Lbracket => return Some(Reduce(23)),
            Star => return Some(Reduce(23)),
            Rbrace => return Some(Reduce(23)),
            Semicolon => return Some(Reduce(23)),
            Lparen => return Some(Reduce(23)),
            LogOp(_) => return Some(Reduce(23)),
            RelOp(_) => return Some(Reduce(23)),
            AddOp(_) => return Some(Reduce(23)),
            AddMinus(_) => return Some(Reduce(23)),
            MulOp(_) => return Some(Reduce(23)),
            And => return Some(Reduce(23)),
            AssignOp => return Some(Reduce(23)),
            _ => return None,
        };
    }

    if state == 170 {
        match token {
            IntLit(_) => return Some(Shift(104)),
            Star => return Some(Shift(99)),
            Identifier(_) => return Some(Shift(106)),
            Lparen => return Some(Shift(107)),
            StringLit(_) => return Some(Shift(105)),
            AddMinus(_) => return Some(Shift(101)),
            And => return Some(Shift(100)),
            UnaryOp(_) => return Some(Shift(98)),
            OPERAND => return Some(Goto(103)),
            PRIMARY_EXPR => return Some(Goto(102)),
            RELATIONAL_EXPR => return Some(Goto(233)),
            ADDITIVE_EXPR => return Some(Goto(95)),
            MULTIPLICATIVE_EXPR => return Some(Goto(96)),
            UNARY_EXPR => return Some(Goto(97)),
            _ => return None,
        };
    }

    if state == 171 {
        match token {
            IntLit(_) => return Some(Shift(104)),
            Star => return Some(Shift(99)),
            Identifier(_) => return Some(Shift(106)),
            Lparen => return Some(Shift(107)),
            StringLit(_) => return Some(Shift(105)),
            AddMinus(_) => return Some(Shift(101)),
            And => return Some(Shift(100)),
            UnaryOp(_) => return Some(Shift(98)),
            OPERAND => return Some(Goto(103)),
            PRIMARY_EXPR => return Some(Goto(102)),
            ADDITIVE_EXPR => return Some(Goto(234)),
            MULTIPLICATIVE_EXPR => return Some(Goto(96)),
            UNARY_EXPR => return Some(Goto(97)),
            _ => return None,
        };
    }

    if state == 172 {
        match token {
            IntLit(_) => return Some(Shift(104)),
            Star => return Some(Shift(99)),
            Identifier(_) => return Some(Shift(106)),
            Lparen => return Some(Shift(107)),
            StringLit(_) => return Some(Shift(105)),
            AddMinus(_) => return Some(Shift(101)),
            And => return Some(Shift(100)),
            UnaryOp(_) => return Some(Shift(98)),
            OPERAND => return Some(Goto(103)),
            PRIMARY_EXPR => return Some(Goto(102)),
            MULTIPLICATIVE_EXPR => return Some(Goto(235)),
            UNARY_EXPR => return Some(Goto(97)),
            _ => return None,
        };
    }

    if state == 173 {
        match token {
            IntLit(_) => return Some(Shift(104)),
            Star => return Some(Shift(99)),
            Identifier(_) => return Some(Shift(106)),
            Lparen => return Some(Shift(107)),
            StringLit(_) => return Some(Shift(105)),
            AddMinus(_) => return Some(Shift(101)),
            And => return Some(Shift(100)),
            UnaryOp(_) => return Some(Shift(98)),
            OPERAND => return Some(Goto(103)),
            PRIMARY_EXPR => return Some(Goto(102)),
            MULTIPLICATIVE_EXPR => return Some(Goto(236)),
            UNARY_EXPR => return Some(Goto(97)),
            _ => return None,
        };
    }

    if state == 174 {
        match token {
            IntLit(_) => return Some(Shift(104)),
            Star => return Some(Shift(99)),
            Identifier(_) => return Some(Shift(106)),
            Lparen => return Some(Shift(107)),
            StringLit(_) => return Some(Shift(105)),
            AddMinus(_) => return Some(Shift(101)),
            And => return Some(Shift(100)),
            UnaryOp(_) => return Some(Shift(98)),
            OPERAND => return Some(Goto(103)),
            PRIMARY_EXPR => return Some(Goto(102)),
            UNARY_EXPR => return Some(Goto(237)),
            _ => return None,
        };
    }

    if state == 175 {
        match token {
            IntLit(_) => return Some(Shift(104)),
            Star => return Some(Shift(99)),
            Identifier(_) => return Some(Shift(106)),
            Lparen => return Some(Shift(107)),
            StringLit(_) => return Some(Shift(105)),
            AddMinus(_) => return Some(Shift(101)),
            And => return Some(Shift(100)),
            UnaryOp(_) => return Some(Shift(98)),
            OPERAND => return Some(Goto(103)),
            PRIMARY_EXPR => return Some(Goto(102)),
            UNARY_EXPR => return Some(Goto(238)),
            _ => return None,
        };
    }

    if state == 176 {
        match token {
            IntLit(_) => return Some(Shift(104)),
            Star => return Some(Shift(99)),
            Identifier(_) => return Some(Shift(106)),
            Lparen => return Some(Shift(107)),
            StringLit(_) => return Some(Shift(105)),
            AddMinus(_) => return Some(Shift(101)),
            And => return Some(Shift(100)),
            UnaryOp(_) => return Some(Shift(98)),
            OPERAND => return Some(Goto(103)),
            PRIMARY_EXPR => return Some(Goto(102)),
            UNARY_EXPR => return Some(Goto(239)),
            _ => return None,
        };
    }

    if state == 177 {
        match token {
            Star => return Some(Reduce(44)),
            Rparen => return Some(Reduce(44)),
            LogOp(_) => return Some(Reduce(44)),
            RelOp(_) => return Some(Reduce(44)),
            AddOp(_) => return Some(Reduce(44)),
            AddMinus(_) => return Some(Reduce(44)),
            MulOp(_) => return Some(Reduce(44)),
            And => return Some(Reduce(44)),
            _ => return None,
        };
    }

    if state == 178 {
        match token {
            Star => return Some(Reduce(45)),
            Rparen => return Some(Reduce(45)),
            LogOp(_) => return Some(Reduce(45)),
            RelOp(_) => return Some(Reduce(45)),
            AddOp(_) => return Some(Reduce(45)),
            AddMinus(_) => return Some(Reduce(45)),
            MulOp(_) => return Some(Reduce(45)),
            And => return Some(Reduce(45)),
            _ => return None,
        };
    }

    if state == 179 {
        match token {
            Star => return Some(Reduce(46)),
            Rparen => return Some(Reduce(46)),
            LogOp(_) => return Some(Reduce(46)),
            RelOp(_) => return Some(Reduce(46)),
            AddOp(_) => return Some(Reduce(46)),
            AddMinus(_) => return Some(Reduce(46)),
            MulOp(_) => return Some(Reduce(46)),
            And => return Some(Reduce(46)),
            _ => return None,
        };
    }

    if state == 180 {
        match token {
            Star => return Some(Reduce(47)),
            Rparen => return Some(Reduce(47)),
            LogOp(_) => return Some(Reduce(47)),
            RelOp(_) => return Some(Reduce(47)),
            AddOp(_) => return Some(Reduce(47)),
            AddMinus(_) => return Some(Reduce(47)),
            MulOp(_) => return Some(Reduce(47)),
            And => return Some(Reduce(47)),
            _ => return None,
        };
    }

    if state == 181 {
        match token {
            Lbracket => return Some(Reduce(24)),
            Star => return Some(Reduce(24)),
            Lparen => return Some(Reduce(24)),
            Rparen => return Some(Reduce(24)),
            LogOp(_) => return Some(Reduce(24)),
            RelOp(_) => return Some(Reduce(24)),
            AddOp(_) => return Some(Reduce(24)),
            AddMinus(_) => return Some(Reduce(24)),
            MulOp(_) => return Some(Reduce(24)),
            And => return Some(Reduce(24)),
            _ => return None,
        };
    }

    if state == 182 {
        match token {
            Lbracket => return Some(Reduce(25)),
            Star => return Some(Reduce(25)),
            Lparen => return Some(Reduce(25)),
            Rparen => return Some(Reduce(25)),
            LogOp(_) => return Some(Reduce(25)),
            RelOp(_) => return Some(Reduce(25)),
            AddOp(_) => return Some(Reduce(25)),
            AddMinus(_) => return Some(Reduce(25)),
            MulOp(_) => return Some(Reduce(25)),
            And => return Some(Reduce(25)),
            _ => return None,
        };
    }

    if state == 183 {
        match token {
            IntLit(_) => return Some(Shift(148)),
            Star => return Some(Shift(143)),
            Identifier(_) => return Some(Shift(150)),
            Lparen => return Some(Shift(151)),
            StringLit(_) => return Some(Shift(149)),
            AddMinus(_) => return Some(Shift(145)),
            And => return Some(Shift(144)),
            UnaryOp(_) => return Some(Shift(142)),
            OPERAND => return Some(Goto(147)),
            PRIMARY_EXPR => return Some(Goto(146)),
            EXPRESSION => return Some(Goto(240)),
            LOGICAL_EXPR => return Some(Goto(137)),
            RELATIONAL_EXPR => return Some(Goto(138)),
            ADDITIVE_EXPR => return Some(Goto(139)),
            MULTIPLICATIVE_EXPR => return Some(Goto(140)),
            UNARY_EXPR => return Some(Goto(141)),
            _ => return None,
        };
    }

    if state == 184 {
        match token {
            IntLit(_) => return Some(Shift(165)),
            Star => return Some(Shift(160)),
            Identifier(_) => return Some(Shift(167)),
            Lparen => return Some(Shift(168)),
            Rparen => return Some(Reduce(31)),
            StringLit(_) => return Some(Shift(166)),
            AddMinus(_) => return Some(Shift(162)),
            And => return Some(Shift(161)),
            UnaryOp(_) => return Some(Shift(159)),
            OPERAND => return Some(Goto(164)),
            PRIMARY_EXPR => return Some(Goto(163)),
            EXPRESSION_LIST => return Some(Goto(241)),
            EXPRESSION => return Some(Goto(153)),
            LOGICAL_EXPR => return Some(Goto(154)),
            RELATIONAL_EXPR => return Some(Goto(155)),
            ADDITIVE_EXPR => return Some(Goto(156)),
            MULTIPLICATIVE_EXPR => return Some(Goto(157)),
            UNARY_EXPR => return Some(Goto(158)),
            _ => return None,
        };
    }

    if state == 185 {
        match token {
            Rparen => return Some(Shift(242)),
            _ => return None,
        };
    }

    if state == 186 {
        match token {
            Rbrace => return Some(Reduce(33)),
            Semicolon => return Some(Reduce(33)),
            LogOp(_) => return Some(Reduce(33)),
            RelOp(_) => return Some(Shift(112)),
            _ => return None,
        };
    }

    if state == 187 {
        match token {
            Rbrace => return Some(Reduce(35)),
            Semicolon => return Some(Reduce(35)),
            LogOp(_) => return Some(Reduce(35)),
            RelOp(_) => return Some(Reduce(35)),
            AddOp(_) => return Some(Shift(113)),
            AddMinus(_) => return Some(Shift(114)),
            _ => return None,
        };
    }

    if state == 188 {
        match token {
            Star => return Some(Shift(116)),
            Rbrace => return Some(Reduce(37)),
            Semicolon => return Some(Reduce(37)),
            LogOp(_) => return Some(Reduce(37)),
            RelOp(_) => return Some(Reduce(37)),
            AddOp(_) => return Some(Reduce(37)),
            AddMinus(_) => return Some(Reduce(37)),
            MulOp(_) => return Some(Shift(115)),
            And => return Some(Shift(117)),
            _ => return None,
        };
    }

    if state == 189 {
        match token {
            Star => return Some(Shift(116)),
            Rbrace => return Some(Reduce(38)),
            Semicolon => return Some(Reduce(38)),
            LogOp(_) => return Some(Reduce(38)),
            RelOp(_) => return Some(Reduce(38)),
            AddOp(_) => return Some(Reduce(38)),
            AddMinus(_) => return Some(Reduce(38)),
            MulOp(_) => return Some(Shift(115)),
            And => return Some(Shift(117)),
            _ => return None,
        };
    }

    if state == 190 {
        match token {
            Star => return Some(Reduce(40)),
            Rbrace => return Some(Reduce(40)),
            Semicolon => return Some(Reduce(40)),
            LogOp(_) => return Some(Reduce(40)),
            RelOp(_) => return Some(Reduce(40)),
            AddOp(_) => return Some(Reduce(40)),
            AddMinus(_) => return Some(Reduce(40)),
            MulOp(_) => return Some(Reduce(40)),
            And => return Some(Reduce(40)),
            _ => return None,
        };
    }

    if state == 191 {
        match token {
            Star => return Some(Reduce(41)),
            Rbrace => return Some(Reduce(41)),
            Semicolon => return Some(Reduce(41)),
            LogOp(_) => return Some(Reduce(41)),
            RelOp(_) => return Some(Reduce(41)),
            AddOp(_) => return Some(Reduce(41)),
            AddMinus(_) => return Some(Reduce(41)),
            MulOp(_) => return Some(Reduce(41)),
            And => return Some(Reduce(41)),
            _ => return None,
        };
    }

    if state == 192 {
        match token {
            Star => return Some(Reduce(42)),
            Rbrace => return Some(Reduce(42)),
            Semicolon => return Some(Reduce(42)),
            LogOp(_) => return Some(Reduce(42)),
            RelOp(_) => return Some(Reduce(42)),
            AddOp(_) => return Some(Reduce(42)),
            AddMinus(_) => return Some(Reduce(42)),
            MulOp(_) => return Some(Reduce(42)),
            And => return Some(Reduce(42)),
            _ => return None,
        };
    }

    if state == 193 {
        match token {
            Rbracket => return Some(Shift(243)),
            _ => return None,
        };
    }

    if state == 194 {
        match token {
            Rparen => return Some(Shift(244)),
            _ => return None,
        };
    }

    if state == 195 {
        match token {
            Lbracket => return Some(Reduce(23)),
            Star => return Some(Reduce(23)),
            Rbrace => return Some(Reduce(23)),
            Semicolon => return Some(Reduce(23)),
            Lparen => return Some(Reduce(23)),
            LogOp(_) => return Some(Reduce(23)),
            RelOp(_) => return Some(Reduce(23)),
            AddOp(_) => return Some(Reduce(23)),
            AddMinus(_) => return Some(Reduce(23)),
            MulOp(_) => return Some(Reduce(23)),
            And => return Some(Reduce(23)),
            _ => return None,
        };
    }

    if state == 196 {
        match token {
            Lbrace => return Some(Shift(246)),
            BLOCK => return Some(Goto(245)),
            _ => return None,
        };
    }

    if state == 197 {
        match token {
            Lbrace => return Some(Shift(246)),
            BLOCK => return Some(Goto(247)),
            _ => return None,
        };
    }

    if state == 198 {
        match token {
            Lbracket => return Some(Reduce(27)),
            Star => return Some(Reduce(27)),
            Rbrace => return Some(Reduce(27)),
            Semicolon => return Some(Reduce(27)),
            Lparen => return Some(Reduce(27)),
            LogOp(_) => return Some(Reduce(27)),
            RelOp(_) => return Some(Reduce(27)),
            AddOp(_) => return Some(Reduce(27)),
            AddMinus(_) => return Some(Reduce(27)),
            MulOp(_) => return Some(Reduce(27)),
            And => return Some(Reduce(27)),
            AssignOp => return Some(Reduce(27)),
            _ => return None,
        };
    }

    if state == 199 {
        match token {
            IntLit(_) => return Some(Shift(148)),
            Star => return Some(Shift(143)),
            Identifier(_) => return Some(Shift(150)),
            Lparen => return Some(Shift(151)),
            StringLit(_) => return Some(Shift(149)),
            AddMinus(_) => return Some(Shift(145)),
            And => return Some(Shift(144)),
            UnaryOp(_) => return Some(Shift(142)),
            OPERAND => return Some(Goto(147)),
            PRIMARY_EXPR => return Some(Goto(146)),
            RELATIONAL_EXPR => return Some(Goto(248)),
            ADDITIVE_EXPR => return Some(Goto(139)),
            MULTIPLICATIVE_EXPR => return Some(Goto(140)),
            UNARY_EXPR => return Some(Goto(141)),
            _ => return None,
        };
    }

    if state == 200 {
        match token {
            IntLit(_) => return Some(Shift(148)),
            Star => return Some(Shift(143)),
            Identifier(_) => return Some(Shift(150)),
            Lparen => return Some(Shift(151)),
            StringLit(_) => return Some(Shift(149)),
            AddMinus(_) => return Some(Shift(145)),
            And => return Some(Shift(144)),
            UnaryOp(_) => return Some(Shift(142)),
            OPERAND => return Some(Goto(147)),
            PRIMARY_EXPR => return Some(Goto(146)),
            ADDITIVE_EXPR => return Some(Goto(249)),
            MULTIPLICATIVE_EXPR => return Some(Goto(140)),
            UNARY_EXPR => return Some(Goto(141)),
            _ => return None,
        };
    }

    if state == 201 {
        match token {
            IntLit(_) => return Some(Shift(148)),
            Star => return Some(Shift(143)),
            Identifier(_) => return Some(Shift(150)),
            Lparen => return Some(Shift(151)),
            StringLit(_) => return Some(Shift(149)),
            AddMinus(_) => return Some(Shift(145)),
            And => return Some(Shift(144)),
            UnaryOp(_) => return Some(Shift(142)),
            OPERAND => return Some(Goto(147)),
            PRIMARY_EXPR => return Some(Goto(146)),
            MULTIPLICATIVE_EXPR => return Some(Goto(250)),
            UNARY_EXPR => return Some(Goto(141)),
            _ => return None,
        };
    }

    if state == 202 {
        match token {
            IntLit(_) => return Some(Shift(148)),
            Star => return Some(Shift(143)),
            Identifier(_) => return Some(Shift(150)),
            Lparen => return Some(Shift(151)),
            StringLit(_) => return Some(Shift(149)),
            AddMinus(_) => return Some(Shift(145)),
            And => return Some(Shift(144)),
            UnaryOp(_) => return Some(Shift(142)),
            OPERAND => return Some(Goto(147)),
            PRIMARY_EXPR => return Some(Goto(146)),
            MULTIPLICATIVE_EXPR => return Some(Goto(251)),
            UNARY_EXPR => return Some(Goto(141)),
            _ => return None,
        };
    }

    if state == 203 {
        match token {
            IntLit(_) => return Some(Shift(148)),
            Star => return Some(Shift(143)),
            Identifier(_) => return Some(Shift(150)),
            Lparen => return Some(Shift(151)),
            StringLit(_) => return Some(Shift(149)),
            AddMinus(_) => return Some(Shift(145)),
            And => return Some(Shift(144)),
            UnaryOp(_) => return Some(Shift(142)),
            OPERAND => return Some(Goto(147)),
            PRIMARY_EXPR => return Some(Goto(146)),
            UNARY_EXPR => return Some(Goto(252)),
            _ => return None,
        };
    }

    if state == 204 {
        match token {
            IntLit(_) => return Some(Shift(148)),
            Star => return Some(Shift(143)),
            Identifier(_) => return Some(Shift(150)),
            Lparen => return Some(Shift(151)),
            StringLit(_) => return Some(Shift(149)),
            AddMinus(_) => return Some(Shift(145)),
            And => return Some(Shift(144)),
            UnaryOp(_) => return Some(Shift(142)),
            OPERAND => return Some(Goto(147)),
            PRIMARY_EXPR => return Some(Goto(146)),
            UNARY_EXPR => return Some(Goto(253)),
            _ => return None,
        };
    }

    if state == 205 {
        match token {
            IntLit(_) => return Some(Shift(148)),
            Star => return Some(Shift(143)),
            Identifier(_) => return Some(Shift(150)),
            Lparen => return Some(Shift(151)),
            StringLit(_) => return Some(Shift(149)),
            AddMinus(_) => return Some(Shift(145)),
            And => return Some(Shift(144)),
            UnaryOp(_) => return Some(Shift(142)),
            OPERAND => return Some(Goto(147)),
            PRIMARY_EXPR => return Some(Goto(146)),
            UNARY_EXPR => return Some(Goto(254)),
            _ => return None,
        };
    }

    if state == 206 {
        match token {
            Rbracket => return Some(Reduce(44)),
            Star => return Some(Reduce(44)),
            LogOp(_) => return Some(Reduce(44)),
            RelOp(_) => return Some(Reduce(44)),
            AddOp(_) => return Some(Reduce(44)),
            AddMinus(_) => return Some(Reduce(44)),
            MulOp(_) => return Some(Reduce(44)),
            And => return Some(Reduce(44)),
            _ => return None,
        };
    }

    if state == 207 {
        match token {
            Rbracket => return Some(Reduce(45)),
            Star => return Some(Reduce(45)),
            LogOp(_) => return Some(Reduce(45)),
            RelOp(_) => return Some(Reduce(45)),
            AddOp(_) => return Some(Reduce(45)),
            AddMinus(_) => return Some(Reduce(45)),
            MulOp(_) => return Some(Reduce(45)),
            And => return Some(Reduce(45)),
            _ => return None,
        };
    }

    if state == 208 {
        match token {
            Rbracket => return Some(Reduce(46)),
            Star => return Some(Reduce(46)),
            LogOp(_) => return Some(Reduce(46)),
            RelOp(_) => return Some(Reduce(46)),
            AddOp(_) => return Some(Reduce(46)),
            AddMinus(_) => return Some(Reduce(46)),
            MulOp(_) => return Some(Reduce(46)),
            And => return Some(Reduce(46)),
            _ => return None,
        };
    }

    if state == 209 {
        match token {
            Rbracket => return Some(Reduce(47)),
            Star => return Some(Reduce(47)),
            LogOp(_) => return Some(Reduce(47)),
            RelOp(_) => return Some(Reduce(47)),
            AddOp(_) => return Some(Reduce(47)),
            AddMinus(_) => return Some(Reduce(47)),
            MulOp(_) => return Some(Reduce(47)),
            And => return Some(Reduce(47)),
            _ => return None,
        };
    }

    if state == 210 {
        match token {
            Lbracket => return Some(Reduce(24)),
            Rbracket => return Some(Reduce(24)),
            Star => return Some(Reduce(24)),
            Lparen => return Some(Reduce(24)),
            LogOp(_) => return Some(Reduce(24)),
            RelOp(_) => return Some(Reduce(24)),
            AddOp(_) => return Some(Reduce(24)),
            AddMinus(_) => return Some(Reduce(24)),
            MulOp(_) => return Some(Reduce(24)),
            And => return Some(Reduce(24)),
            _ => return None,
        };
    }

    if state == 211 {
        match token {
            Lbracket => return Some(Reduce(25)),
            Rbracket => return Some(Reduce(25)),
            Star => return Some(Reduce(25)),
            Lparen => return Some(Reduce(25)),
            LogOp(_) => return Some(Reduce(25)),
            RelOp(_) => return Some(Reduce(25)),
            AddOp(_) => return Some(Reduce(25)),
            AddMinus(_) => return Some(Reduce(25)),
            MulOp(_) => return Some(Reduce(25)),
            And => return Some(Reduce(25)),
            _ => return None,
        };
    }

    if state == 212 {
        match token {
            IntLit(_) => return Some(Shift(148)),
            Star => return Some(Shift(143)),
            Identifier(_) => return Some(Shift(150)),
            Lparen => return Some(Shift(151)),
            StringLit(_) => return Some(Shift(149)),
            AddMinus(_) => return Some(Shift(145)),
            And => return Some(Shift(144)),
            UnaryOp(_) => return Some(Shift(142)),
            OPERAND => return Some(Goto(147)),
            PRIMARY_EXPR => return Some(Goto(146)),
            EXPRESSION => return Some(Goto(255)),
            LOGICAL_EXPR => return Some(Goto(137)),
            RELATIONAL_EXPR => return Some(Goto(138)),
            ADDITIVE_EXPR => return Some(Goto(139)),
            MULTIPLICATIVE_EXPR => return Some(Goto(140)),
            UNARY_EXPR => return Some(Goto(141)),
            _ => return None,
        };
    }

    if state == 213 {
        match token {
            IntLit(_) => return Some(Shift(165)),
            Star => return Some(Shift(160)),
            Identifier(_) => return Some(Shift(167)),
            Lparen => return Some(Shift(168)),
            Rparen => return Some(Reduce(31)),
            StringLit(_) => return Some(Shift(166)),
            AddMinus(_) => return Some(Shift(162)),
            And => return Some(Shift(161)),
            UnaryOp(_) => return Some(Shift(159)),
            OPERAND => return Some(Goto(164)),
            PRIMARY_EXPR => return Some(Goto(163)),
            EXPRESSION_LIST => return Some(Goto(256)),
            EXPRESSION => return Some(Goto(153)),
            LOGICAL_EXPR => return Some(Goto(154)),
            RELATIONAL_EXPR => return Some(Goto(155)),
            ADDITIVE_EXPR => return Some(Goto(156)),
            MULTIPLICATIVE_EXPR => return Some(Goto(157)),
            UNARY_EXPR => return Some(Goto(158)),
            _ => return None,
        };
    }

    if state == 214 {
        match token {
            Rparen => return Some(Shift(257)),
            _ => return None,
        };
    }

    if state == 215 {
        match token {
            Lbracket => return Some(Reduce(28)),
            Star => return Some(Reduce(28)),
            Rbrace => return Some(Reduce(28)),
            Semicolon => return Some(Reduce(28)),
            Lparen => return Some(Reduce(28)),
            LogOp(_) => return Some(Reduce(28)),
            RelOp(_) => return Some(Reduce(28)),
            AddOp(_) => return Some(Reduce(28)),
            AddMinus(_) => return Some(Reduce(28)),
            MulOp(_) => return Some(Reduce(28)),
            And => return Some(Reduce(28)),
            AssignOp => return Some(Reduce(28)),
            _ => return None,
        };
    }

    if state == 216 {
        match token {
            IntLit(_) => return Some(Shift(165)),
            Star => return Some(Shift(160)),
            Identifier(_) => return Some(Shift(167)),
            Lparen => return Some(Shift(168)),
            Rparen => return Some(Reduce(31)),
            StringLit(_) => return Some(Shift(166)),
            AddMinus(_) => return Some(Shift(162)),
            And => return Some(Shift(161)),
            UnaryOp(_) => return Some(Shift(159)),
            OPERAND => return Some(Goto(164)),
            PRIMARY_EXPR => return Some(Goto(163)),
            EXPRESSION_LIST => return Some(Goto(258)),
            EXPRESSION => return Some(Goto(153)),
            LOGICAL_EXPR => return Some(Goto(154)),
            RELATIONAL_EXPR => return Some(Goto(155)),
            ADDITIVE_EXPR => return Some(Goto(156)),
            MULTIPLICATIVE_EXPR => return Some(Goto(157)),
            UNARY_EXPR => return Some(Goto(158)),
            _ => return None,
        };
    }

    if state == 217 {
        match token {
            IntLit(_) => return Some(Shift(165)),
            Star => return Some(Shift(160)),
            Identifier(_) => return Some(Shift(167)),
            Lparen => return Some(Shift(168)),
            StringLit(_) => return Some(Shift(166)),
            AddMinus(_) => return Some(Shift(162)),
            And => return Some(Shift(161)),
            UnaryOp(_) => return Some(Shift(159)),
            OPERAND => return Some(Goto(164)),
            PRIMARY_EXPR => return Some(Goto(163)),
            RELATIONAL_EXPR => return Some(Goto(259)),
            ADDITIVE_EXPR => return Some(Goto(156)),
            MULTIPLICATIVE_EXPR => return Some(Goto(157)),
            UNARY_EXPR => return Some(Goto(158)),
            _ => return None,
        };
    }

    if state == 218 {
        match token {
            IntLit(_) => return Some(Shift(165)),
            Star => return Some(Shift(160)),
            Identifier(_) => return Some(Shift(167)),
            Lparen => return Some(Shift(168)),
            StringLit(_) => return Some(Shift(166)),
            AddMinus(_) => return Some(Shift(162)),
            And => return Some(Shift(161)),
            UnaryOp(_) => return Some(Shift(159)),
            OPERAND => return Some(Goto(164)),
            PRIMARY_EXPR => return Some(Goto(163)),
            ADDITIVE_EXPR => return Some(Goto(260)),
            MULTIPLICATIVE_EXPR => return Some(Goto(157)),
            UNARY_EXPR => return Some(Goto(158)),
            _ => return None,
        };
    }

    if state == 219 {
        match token {
            IntLit(_) => return Some(Shift(165)),
            Star => return Some(Shift(160)),
            Identifier(_) => return Some(Shift(167)),
            Lparen => return Some(Shift(168)),
            StringLit(_) => return Some(Shift(166)),
            AddMinus(_) => return Some(Shift(162)),
            And => return Some(Shift(161)),
            UnaryOp(_) => return Some(Shift(159)),
            OPERAND => return Some(Goto(164)),
            PRIMARY_EXPR => return Some(Goto(163)),
            MULTIPLICATIVE_EXPR => return Some(Goto(261)),
            UNARY_EXPR => return Some(Goto(158)),
            _ => return None,
        };
    }

    if state == 220 {
        match token {
            IntLit(_) => return Some(Shift(165)),
            Star => return Some(Shift(160)),
            Identifier(_) => return Some(Shift(167)),
            Lparen => return Some(Shift(168)),
            StringLit(_) => return Some(Shift(166)),
            AddMinus(_) => return Some(Shift(162)),
            And => return Some(Shift(161)),
            UnaryOp(_) => return Some(Shift(159)),
            OPERAND => return Some(Goto(164)),
            PRIMARY_EXPR => return Some(Goto(163)),
            MULTIPLICATIVE_EXPR => return Some(Goto(262)),
            UNARY_EXPR => return Some(Goto(158)),
            _ => return None,
        };
    }

    if state == 221 {
        match token {
            IntLit(_) => return Some(Shift(165)),
            Star => return Some(Shift(160)),
            Identifier(_) => return Some(Shift(167)),
            Lparen => return Some(Shift(168)),
            StringLit(_) => return Some(Shift(166)),
            AddMinus(_) => return Some(Shift(162)),
            And => return Some(Shift(161)),
            UnaryOp(_) => return Some(Shift(159)),
            OPERAND => return Some(Goto(164)),
            PRIMARY_EXPR => return Some(Goto(163)),
            UNARY_EXPR => return Some(Goto(263)),
            _ => return None,
        };
    }

    if state == 222 {
        match token {
            IntLit(_) => return Some(Shift(165)),
            Star => return Some(Shift(160)),
            Identifier(_) => return Some(Shift(167)),
            Lparen => return Some(Shift(168)),
            StringLit(_) => return Some(Shift(166)),
            AddMinus(_) => return Some(Shift(162)),
            And => return Some(Shift(161)),
            UnaryOp(_) => return Some(Shift(159)),
            OPERAND => return Some(Goto(164)),
            PRIMARY_EXPR => return Some(Goto(163)),
            UNARY_EXPR => return Some(Goto(264)),
            _ => return None,
        };
    }

    if state == 223 {
        match token {
            IntLit(_) => return Some(Shift(165)),
            Star => return Some(Shift(160)),
            Identifier(_) => return Some(Shift(167)),
            Lparen => return Some(Shift(168)),
            StringLit(_) => return Some(Shift(166)),
            AddMinus(_) => return Some(Shift(162)),
            And => return Some(Shift(161)),
            UnaryOp(_) => return Some(Shift(159)),
            OPERAND => return Some(Goto(164)),
            PRIMARY_EXPR => return Some(Goto(163)),
            UNARY_EXPR => return Some(Goto(265)),
            _ => return None,
        };
    }

    if state == 224 {
        match token {
            Star => return Some(Reduce(44)),
            Rparen => return Some(Reduce(44)),
            Comma => return Some(Reduce(44)),
            LogOp(_) => return Some(Reduce(44)),
            RelOp(_) => return Some(Reduce(44)),
            AddOp(_) => return Some(Reduce(44)),
            AddMinus(_) => return Some(Reduce(44)),
            MulOp(_) => return Some(Reduce(44)),
            And => return Some(Reduce(44)),
            _ => return None,
        };
    }

    if state == 225 {
        match token {
            Star => return Some(Reduce(45)),
            Rparen => return Some(Reduce(45)),
            Comma => return Some(Reduce(45)),
            LogOp(_) => return Some(Reduce(45)),
            RelOp(_) => return Some(Reduce(45)),
            AddOp(_) => return Some(Reduce(45)),
            AddMinus(_) => return Some(Reduce(45)),
            MulOp(_) => return Some(Reduce(45)),
            And => return Some(Reduce(45)),
            _ => return None,
        };
    }

    if state == 226 {
        match token {
            Star => return Some(Reduce(46)),
            Rparen => return Some(Reduce(46)),
            Comma => return Some(Reduce(46)),
            LogOp(_) => return Some(Reduce(46)),
            RelOp(_) => return Some(Reduce(46)),
            AddOp(_) => return Some(Reduce(46)),
            AddMinus(_) => return Some(Reduce(46)),
            MulOp(_) => return Some(Reduce(46)),
            And => return Some(Reduce(46)),
            _ => return None,
        };
    }

    if state == 227 {
        match token {
            Star => return Some(Reduce(47)),
            Rparen => return Some(Reduce(47)),
            Comma => return Some(Reduce(47)),
            LogOp(_) => return Some(Reduce(47)),
            RelOp(_) => return Some(Reduce(47)),
            AddOp(_) => return Some(Reduce(47)),
            AddMinus(_) => return Some(Reduce(47)),
            MulOp(_) => return Some(Reduce(47)),
            And => return Some(Reduce(47)),
            _ => return None,
        };
    }

    if state == 228 {
        match token {
            Lbracket => return Some(Reduce(24)),
            Star => return Some(Reduce(24)),
            Lparen => return Some(Reduce(24)),
            Rparen => return Some(Reduce(24)),
            Comma => return Some(Reduce(24)),
            LogOp(_) => return Some(Reduce(24)),
            RelOp(_) => return Some(Reduce(24)),
            AddOp(_) => return Some(Reduce(24)),
            AddMinus(_) => return Some(Reduce(24)),
            MulOp(_) => return Some(Reduce(24)),
            And => return Some(Reduce(24)),
            _ => return None,
        };
    }

    if state == 229 {
        match token {
            Lbracket => return Some(Reduce(25)),
            Star => return Some(Reduce(25)),
            Lparen => return Some(Reduce(25)),
            Rparen => return Some(Reduce(25)),
            Comma => return Some(Reduce(25)),
            LogOp(_) => return Some(Reduce(25)),
            RelOp(_) => return Some(Reduce(25)),
            AddOp(_) => return Some(Reduce(25)),
            AddMinus(_) => return Some(Reduce(25)),
            MulOp(_) => return Some(Reduce(25)),
            And => return Some(Reduce(25)),
            _ => return None,
        };
    }

    if state == 230 {
        match token {
            IntLit(_) => return Some(Shift(148)),
            Star => return Some(Shift(143)),
            Identifier(_) => return Some(Shift(150)),
            Lparen => return Some(Shift(151)),
            StringLit(_) => return Some(Shift(149)),
            AddMinus(_) => return Some(Shift(145)),
            And => return Some(Shift(144)),
            UnaryOp(_) => return Some(Shift(142)),
            OPERAND => return Some(Goto(147)),
            PRIMARY_EXPR => return Some(Goto(146)),
            EXPRESSION => return Some(Goto(266)),
            LOGICAL_EXPR => return Some(Goto(137)),
            RELATIONAL_EXPR => return Some(Goto(138)),
            ADDITIVE_EXPR => return Some(Goto(139)),
            MULTIPLICATIVE_EXPR => return Some(Goto(140)),
            UNARY_EXPR => return Some(Goto(141)),
            _ => return None,
        };
    }

    if state == 231 {
        match token {
            IntLit(_) => return Some(Shift(165)),
            Star => return Some(Shift(160)),
            Identifier(_) => return Some(Shift(167)),
            Lparen => return Some(Shift(168)),
            Rparen => return Some(Reduce(31)),
            StringLit(_) => return Some(Shift(166)),
            AddMinus(_) => return Some(Shift(162)),
            And => return Some(Shift(161)),
            UnaryOp(_) => return Some(Shift(159)),
            OPERAND => return Some(Goto(164)),
            PRIMARY_EXPR => return Some(Goto(163)),
            EXPRESSION_LIST => return Some(Goto(267)),
            EXPRESSION => return Some(Goto(153)),
            LOGICAL_EXPR => return Some(Goto(154)),
            RELATIONAL_EXPR => return Some(Goto(155)),
            ADDITIVE_EXPR => return Some(Goto(156)),
            MULTIPLICATIVE_EXPR => return Some(Goto(157)),
            UNARY_EXPR => return Some(Goto(158)),
            _ => return None,
        };
    }

    if state == 232 {
        match token {
            Rparen => return Some(Shift(268)),
            _ => return None,
        };
    }

    if state == 233 {
        match token {
            Rparen => return Some(Reduce(33)),
            LogOp(_) => return Some(Reduce(33)),
            RelOp(_) => return Some(Shift(171)),
            _ => return None,
        };
    }

    if state == 234 {
        match token {
            Rparen => return Some(Reduce(35)),
            LogOp(_) => return Some(Reduce(35)),
            RelOp(_) => return Some(Reduce(35)),
            AddOp(_) => return Some(Shift(172)),
            AddMinus(_) => return Some(Shift(173)),
            _ => return None,
        };
    }

    if state == 235 {
        match token {
            Star => return Some(Shift(175)),
            Rparen => return Some(Reduce(37)),
            LogOp(_) => return Some(Reduce(37)),
            RelOp(_) => return Some(Reduce(37)),
            AddOp(_) => return Some(Reduce(37)),
            AddMinus(_) => return Some(Reduce(37)),
            MulOp(_) => return Some(Shift(174)),
            And => return Some(Shift(176)),
            _ => return None,
        };
    }

    if state == 236 {
        match token {
            Star => return Some(Shift(175)),
            Rparen => return Some(Reduce(38)),
            LogOp(_) => return Some(Reduce(38)),
            RelOp(_) => return Some(Reduce(38)),
            AddOp(_) => return Some(Reduce(38)),
            AddMinus(_) => return Some(Reduce(38)),
            MulOp(_) => return Some(Shift(174)),
            And => return Some(Shift(176)),
            _ => return None,
        };
    }

    if state == 237 {
        match token {
            Star => return Some(Reduce(40)),
            Rparen => return Some(Reduce(40)),
            LogOp(_) => return Some(Reduce(40)),
            RelOp(_) => return Some(Reduce(40)),
            AddOp(_) => return Some(Reduce(40)),
            AddMinus(_) => return Some(Reduce(40)),
            MulOp(_) => return Some(Reduce(40)),
            And => return Some(Reduce(40)),
            _ => return None,
        };
    }

    if state == 238 {
        match token {
            Star => return Some(Reduce(41)),
            Rparen => return Some(Reduce(41)),
            LogOp(_) => return Some(Reduce(41)),
            RelOp(_) => return Some(Reduce(41)),
            AddOp(_) => return Some(Reduce(41)),
            AddMinus(_) => return Some(Reduce(41)),
            MulOp(_) => return Some(Reduce(41)),
            And => return Some(Reduce(41)),
            _ => return None,
        };
    }

    if state == 239 {
        match token {
            Star => return Some(Reduce(42)),
            Rparen => return Some(Reduce(42)),
            LogOp(_) => return Some(Reduce(42)),
            RelOp(_) => return Some(Reduce(42)),
            AddOp(_) => return Some(Reduce(42)),
            AddMinus(_) => return Some(Reduce(42)),
            MulOp(_) => return Some(Reduce(42)),
            And => return Some(Reduce(42)),
            _ => return None,
        };
    }

    if state == 240 {
        match token {
            Rbracket => return Some(Shift(269)),
            _ => return None,
        };
    }

    if state == 241 {
        match token {
            Rparen => return Some(Shift(270)),
            _ => return None,
        };
    }

    if state == 242 {
        match token {
            Lbracket => return Some(Reduce(23)),
            Star => return Some(Reduce(23)),
            Lparen => return Some(Reduce(23)),
            Rparen => return Some(Reduce(23)),
            LogOp(_) => return Some(Reduce(23)),
            RelOp(_) => return Some(Reduce(23)),
            AddOp(_) => return Some(Reduce(23)),
            AddMinus(_) => return Some(Reduce(23)),
            MulOp(_) => return Some(Reduce(23)),
            And => return Some(Reduce(23)),
            _ => return None,
        };
    }

    if state == 243 {
        match token {
            Lbracket => return Some(Reduce(27)),
            Star => return Some(Reduce(27)),
            Rbrace => return Some(Reduce(27)),
            Semicolon => return Some(Reduce(27)),
            Lparen => return Some(Reduce(27)),
            LogOp(_) => return Some(Reduce(27)),
            RelOp(_) => return Some(Reduce(27)),
            AddOp(_) => return Some(Reduce(27)),
            AddMinus(_) => return Some(Reduce(27)),
            MulOp(_) => return Some(Reduce(27)),
            And => return Some(Reduce(27)),
            _ => return None,
        };
    }

    if state == 244 {
        match token {
            Lbracket => return Some(Reduce(28)),
            Star => return Some(Reduce(28)),
            Rbrace => return Some(Reduce(28)),
            Semicolon => return Some(Reduce(28)),
            Lparen => return Some(Reduce(28)),
            LogOp(_) => return Some(Reduce(28)),
            RelOp(_) => return Some(Reduce(28)),
            AddOp(_) => return Some(Reduce(28)),
            AddMinus(_) => return Some(Reduce(28)),
            MulOp(_) => return Some(Reduce(28)),
            And => return Some(Reduce(28)),
            _ => return None,
        };
    }

    if state == 245 {
        match token {
            Rbrace => return Some(Reduce(58)),
            Semicolon => return Some(Reduce(58)),
            _ => return None,
        };
    }

    if state == 246 {
        match token {
            Int => return Some(Shift(4)),
            Void => return Some(Shift(5)),
            IntLit(_) => return Some(Shift(48)),
            Star => return Some(Shift(43)),
            Rbrace => return Some(Reduce(12)),
            Identifier(_) => return Some(Shift(50)),
            Lparen => return Some(Shift(51)),
            StringLit(_) => return Some(Shift(49)),
            AddMinus(_) => return Some(Shift(45)),
            And => return Some(Shift(44)),
            UnaryOp(_) => return Some(Shift(42)),
            If => return Some(Shift(35)),
            While => return Some(Shift(36)),
            Return => return Some(Shift(32)),
            Break => return Some(Shift(33)),
            Continue => return Some(Shift(34)),
            TYPE => return Some(Goto(31)),
            ARRAY_TYPE => return Some(Goto(6)),
            POINTER_TYPE => return Some(Goto(7)),
            STATEMENT_LIST => return Some(Goto(271)),
            VAR_DECL => return Some(Goto(24)),
            OPERAND => return Some(Goto(47)),
            PRIMARY_EXPR => return Some(Goto(46)),
            EXPRESSION => return Some(Goto(30)),
            LOGICAL_EXPR => return Some(Goto(37)),
            RELATIONAL_EXPR => return Some(Goto(38)),
            ADDITIVE_EXPR => return Some(Goto(39)),
            MULTIPLICATIVE_EXPR => return Some(Goto(40)),
            UNARY_EXPR => return Some(Goto(41)),
            STATEMENT => return Some(Goto(22)),
            ASSIGNMENT => return Some(Goto(23)),
            IF_STMT => return Some(Goto(28)),
            WHILE_STMT => return Some(Goto(29)),
            RETURN_STMT => return Some(Goto(25)),
            BREAK_STMT => return Some(Goto(26)),
            CONTINUE_STMT => return Some(Goto(27)),
            _ => return None,
        };
    }

    if state == 247 {
        match token {
            Rbrace => return Some(Reduce(59)),
            Semicolon => return Some(Reduce(59)),
            _ => return None,
        };
    }

    if state == 248 {
        match token {
            Rbracket => return Some(Reduce(33)),
            LogOp(_) => return Some(Reduce(33)),
            RelOp(_) => return Some(Shift(200)),
            _ => return None,
        };
    }

    if state == 249 {
        match token {
            Rbracket => return Some(Reduce(35)),
            LogOp(_) => return Some(Reduce(35)),
            RelOp(_) => return Some(Reduce(35)),
            AddOp(_) => return Some(Shift(201)),
            AddMinus(_) => return Some(Shift(202)),
            _ => return None,
        };
    }

    if state == 250 {
        match token {
            Rbracket => return Some(Reduce(37)),
            Star => return Some(Shift(204)),
            LogOp(_) => return Some(Reduce(37)),
            RelOp(_) => return Some(Reduce(37)),
            AddOp(_) => return Some(Reduce(37)),
            AddMinus(_) => return Some(Reduce(37)),
            MulOp(_) => return Some(Shift(203)),
            And => return Some(Shift(205)),
            _ => return None,
        };
    }

    if state == 251 {
        match token {
            Rbracket => return Some(Reduce(38)),
            Star => return Some(Shift(204)),
            LogOp(_) => return Some(Reduce(38)),
            RelOp(_) => return Some(Reduce(38)),
            AddOp(_) => return Some(Reduce(38)),
            AddMinus(_) => return Some(Reduce(38)),
            MulOp(_) => return Some(Shift(203)),
            And => return Some(Shift(205)),
            _ => return None,
        };
    }

    if state == 252 {
        match token {
            Rbracket => return Some(Reduce(40)),
            Star => return Some(Reduce(40)),
            LogOp(_) => return Some(Reduce(40)),
            RelOp(_) => return Some(Reduce(40)),
            AddOp(_) => return Some(Reduce(40)),
            AddMinus(_) => return Some(Reduce(40)),
            MulOp(_) => return Some(Reduce(40)),
            And => return Some(Reduce(40)),
            _ => return None,
        };
    }

    if state == 253 {
        match token {
            Rbracket => return Some(Reduce(41)),
            Star => return Some(Reduce(41)),
            LogOp(_) => return Some(Reduce(41)),
            RelOp(_) => return Some(Reduce(41)),
            AddOp(_) => return Some(Reduce(41)),
            AddMinus(_) => return Some(Reduce(41)),
            MulOp(_) => return Some(Reduce(41)),
            And => return Some(Reduce(41)),
            _ => return None,
        };
    }

    if state == 254 {
        match token {
            Rbracket => return Some(Reduce(42)),
            Star => return Some(Reduce(42)),
            LogOp(_) => return Some(Reduce(42)),
            RelOp(_) => return Some(Reduce(42)),
            AddOp(_) => return Some(Reduce(42)),
            AddMinus(_) => return Some(Reduce(42)),
            MulOp(_) => return Some(Reduce(42)),
            And => return Some(Reduce(42)),
            _ => return None,
        };
    }

    if state == 255 {
        match token {
            Rbracket => return Some(Shift(272)),
            _ => return None,
        };
    }

    if state == 256 {
        match token {
            Rparen => return Some(Shift(273)),
            _ => return None,
        };
    }

    if state == 257 {
        match token {
            Lbracket => return Some(Reduce(23)),
            Rbracket => return Some(Reduce(23)),
            Star => return Some(Reduce(23)),
            Lparen => return Some(Reduce(23)),
            LogOp(_) => return Some(Reduce(23)),
            RelOp(_) => return Some(Reduce(23)),
            AddOp(_) => return Some(Reduce(23)),
            AddMinus(_) => return Some(Reduce(23)),
            MulOp(_) => return Some(Reduce(23)),
            And => return Some(Reduce(23)),
            _ => return None,
        };
    }

    if state == 258 {
        match token {
            Rparen => return Some(Reduce(29)),
            _ => return None,
        };
    }

    if state == 259 {
        match token {
            Rparen => return Some(Reduce(33)),
            Comma => return Some(Reduce(33)),
            LogOp(_) => return Some(Reduce(33)),
            RelOp(_) => return Some(Shift(218)),
            _ => return None,
        };
    }

    if state == 260 {
        match token {
            Rparen => return Some(Reduce(35)),
            Comma => return Some(Reduce(35)),
            LogOp(_) => return Some(Reduce(35)),
            RelOp(_) => return Some(Reduce(35)),
            AddOp(_) => return Some(Shift(219)),
            AddMinus(_) => return Some(Shift(220)),
            _ => return None,
        };
    }

    if state == 261 {
        match token {
            Star => return Some(Shift(222)),
            Rparen => return Some(Reduce(37)),
            Comma => return Some(Reduce(37)),
            LogOp(_) => return Some(Reduce(37)),
            RelOp(_) => return Some(Reduce(37)),
            AddOp(_) => return Some(Reduce(37)),
            AddMinus(_) => return Some(Reduce(37)),
            MulOp(_) => return Some(Shift(221)),
            And => return Some(Shift(223)),
            _ => return None,
        };
    }

    if state == 262 {
        match token {
            Star => return Some(Shift(222)),
            Rparen => return Some(Reduce(38)),
            Comma => return Some(Reduce(38)),
            LogOp(_) => return Some(Reduce(38)),
            RelOp(_) => return Some(Reduce(38)),
            AddOp(_) => return Some(Reduce(38)),
            AddMinus(_) => return Some(Reduce(38)),
            MulOp(_) => return Some(Shift(221)),
            And => return Some(Shift(223)),
            _ => return None,
        };
    }

    if state == 263 {
        match token {
            Star => return Some(Reduce(40)),
            Rparen => return Some(Reduce(40)),
            Comma => return Some(Reduce(40)),
            LogOp(_) => return Some(Reduce(40)),
            RelOp(_) => return Some(Reduce(40)),
            AddOp(_) => return Some(Reduce(40)),
            AddMinus(_) => return Some(Reduce(40)),
            MulOp(_) => return Some(Reduce(40)),
            And => return Some(Reduce(40)),
            _ => return None,
        };
    }

    if state == 264 {
        match token {
            Star => return Some(Reduce(41)),
            Rparen => return Some(Reduce(41)),
            Comma => return Some(Reduce(41)),
            LogOp(_) => return Some(Reduce(41)),
            RelOp(_) => return Some(Reduce(41)),
            AddOp(_) => return Some(Reduce(41)),
            AddMinus(_) => return Some(Reduce(41)),
            MulOp(_) => return Some(Reduce(41)),
            And => return Some(Reduce(41)),
            _ => return None,
        };
    }

    if state == 265 {
        match token {
            Star => return Some(Reduce(42)),
            Rparen => return Some(Reduce(42)),
            Comma => return Some(Reduce(42)),
            LogOp(_) => return Some(Reduce(42)),
            RelOp(_) => return Some(Reduce(42)),
            AddOp(_) => return Some(Reduce(42)),
            AddMinus(_) => return Some(Reduce(42)),
            MulOp(_) => return Some(Reduce(42)),
            And => return Some(Reduce(42)),
            _ => return None,
        };
    }

    if state == 266 {
        match token {
            Rbracket => return Some(Shift(274)),
            _ => return None,
        };
    }

    if state == 267 {
        match token {
            Rparen => return Some(Shift(275)),
            _ => return None,
        };
    }

    if state == 268 {
        match token {
            Lbracket => return Some(Reduce(23)),
            Star => return Some(Reduce(23)),
            Lparen => return Some(Reduce(23)),
            Rparen => return Some(Reduce(23)),
            Comma => return Some(Reduce(23)),
            LogOp(_) => return Some(Reduce(23)),
            RelOp(_) => return Some(Reduce(23)),
            AddOp(_) => return Some(Reduce(23)),
            AddMinus(_) => return Some(Reduce(23)),
            MulOp(_) => return Some(Reduce(23)),
            And => return Some(Reduce(23)),
            _ => return None,
        };
    }

    if state == 269 {
        match token {
            Lbracket => return Some(Reduce(27)),
            Star => return Some(Reduce(27)),
            Lparen => return Some(Reduce(27)),
            Rparen => return Some(Reduce(27)),
            LogOp(_) => return Some(Reduce(27)),
            RelOp(_) => return Some(Reduce(27)),
            AddOp(_) => return Some(Reduce(27)),
            AddMinus(_) => return Some(Reduce(27)),
            MulOp(_) => return Some(Reduce(27)),
            And => return Some(Reduce(27)),
            _ => return None,
        };
    }

    if state == 270 {
        match token {
            Lbracket => return Some(Reduce(28)),
            Star => return Some(Reduce(28)),
            Lparen => return Some(Reduce(28)),
            Rparen => return Some(Reduce(28)),
            LogOp(_) => return Some(Reduce(28)),
            RelOp(_) => return Some(Reduce(28)),
            AddOp(_) => return Some(Reduce(28)),
            AddMinus(_) => return Some(Reduce(28)),
            MulOp(_) => return Some(Reduce(28)),
            And => return Some(Reduce(28)),
            _ => return None,
        };
    }

    if state == 271 {
        match token {
            Rbrace => return Some(Shift(276)),
            _ => return None,
        };
    }

    if state == 272 {
        match token {
            Lbracket => return Some(Reduce(27)),
            Rbracket => return Some(Reduce(27)),
            Star => return Some(Reduce(27)),
            Lparen => return Some(Reduce(27)),
            LogOp(_) => return Some(Reduce(27)),
            RelOp(_) => return Some(Reduce(27)),
            AddOp(_) => return Some(Reduce(27)),
            AddMinus(_) => return Some(Reduce(27)),
            MulOp(_) => return Some(Reduce(27)),
            And => return Some(Reduce(27)),
            _ => return None,
        };
    }

    if state == 273 {
        match token {
            Lbracket => return Some(Reduce(28)),
            Rbracket => return Some(Reduce(28)),
            Star => return Some(Reduce(28)),
            Lparen => return Some(Reduce(28)),
            LogOp(_) => return Some(Reduce(28)),
            RelOp(_) => return Some(Reduce(28)),
            AddOp(_) => return Some(Reduce(28)),
            AddMinus(_) => return Some(Reduce(28)),
            MulOp(_) => return Some(Reduce(28)),
            And => return Some(Reduce(28)),
            _ => return None,
        };
    }

    if state == 274 {
        match token {
            Lbracket => return Some(Reduce(27)),
            Star => return Some(Reduce(27)),
            Lparen => return Some(Reduce(27)),
            Rparen => return Some(Reduce(27)),
            Comma => return Some(Reduce(27)),
            LogOp(_) => return Some(Reduce(27)),
            RelOp(_) => return Some(Reduce(27)),
            AddOp(_) => return Some(Reduce(27)),
            AddMinus(_) => return Some(Reduce(27)),
            MulOp(_) => return Some(Reduce(27)),
            And => return Some(Reduce(27)),
            _ => return None,
        };
    }

    if state == 275 {
        match token {
            Lbracket => return Some(Reduce(28)),
            Star => return Some(Reduce(28)),
            Lparen => return Some(Reduce(28)),
            Rparen => return Some(Reduce(28)),
            Comma => return Some(Reduce(28)),
            LogOp(_) => return Some(Reduce(28)),
            RelOp(_) => return Some(Reduce(28)),
            AddOp(_) => return Some(Reduce(28)),
            AddMinus(_) => return Some(Reduce(28)),
            MulOp(_) => return Some(Reduce(28)),
            And => return Some(Reduce(28)),
            _ => return None,
        };
    }

    if state == 276 {
        match token {
            Rbrace => return Some(Reduce(9)),
            Semicolon => return Some(Reduce(9)),
            _ => return None,
        };
    }
