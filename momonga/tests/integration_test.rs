extern crate momonga;
use crate::momonga::*;

#[test]
fn comment_is_ignored() {
    let tests = [
        (
            r#"
            /*
               // Line comment in block comment is allowed like this.
               But block comment in block comment is not allowed.
            */
            // comment // comment
            //// comment
            "#,
            None,
        ),
        (
            r#"
            var x;
            if(true) {
                x /* comment */ = /* comment */ 1; // comment

                // x = 2;

                /*
                x = 3;
                */
            }
            /* x = 4; */
            x; /* comment */
            "#,
            Some("1".to_string()),
        ),
    ];
    for (src, expected) in tests {
        assert_eq!(interpret(src), expected, "Failed in test case: {}", src);
    }
}

#[test]
fn literal_expression_is_interpreted_correctly() {
    let tests = [
        // Boolean
        (
            r#"
            true;
            "#,
            Some("true".to_string()),
        ),
        (
            r#"
            false;
            "#,
            Some("false".to_string()),
        ),
        // Integer
        (
            r#"
            0;
            "#,
            Some("0".to_string()),
        ),
        (
            r#"
            9223372036854775807;  // Max value of Integer type
            "#,
            Some(std::i64::MAX.to_string()),
        ),
        // String
        (
            r#"
            "foo";
            "#,
            Some("foo".to_string()),
        ),
        // Array
        (
            r#"
            [

            ];
            "#,
            Some("[]".to_string()),
        ),
        (
            r#"
            [1, 2, 3];
            "#,
            Some("[1, 2, 3]".to_string()),
        ),
        // None
        (
            r#"
            none;
            "#,
            Some("none".to_string()),
        ),
    ];
    for (src, expected) in tests {
        assert_eq!(interpret(src), expected, "Failed in test case: {}", src);
    }
}

#[test]
fn prefix_operator_is_interpreted_correctly() {
    let tests = [
        // Operator: !
        (
            r#"
            !true;
        "#,
            Some("false".to_string()),
        ),
        (
            r#"
            !false;
        "#,
            Some("true".to_string()),
        ),
        (
            r#"
            !!true;
            "#,
            Some("true".to_string()),
        ),
        (
            r#"
            !!!true;
            "#,
            Some("false".to_string()),
        ),
        //Operator: + -
        (
            r#"
            -9223372036854775808; // Min value of Integer type
        "#,
            Some(i64::MIN.to_string()),
        ),
        (
            r#"
            -9223372036854775809; // Min - 1
        "#,
            Some("Out of range error".to_string()),
        ),
        // (
        //     r#"
        //     -92233720368547758010; // Min - 2
        // "#,
        //     Some("Out of range error".to_string()),
        // ),
        (
            r#"
            +-9223372036854775808;  // Attempt to apply + operator to the min of Integer type
        "#,
            Some(i64::MIN.to_string()),
        ),
        (
            r#"
            +9223372036854775807;  // Attmept to apply + oeraptor to the max value of Integer type
            "#,
            Some(std::i64::MAX.to_string()),
        ),
        (
            r#"
            -+9223372036854775808;  // Attempt to apply - operator to the max of Integer type
        "#,
            Some("Out of range error".to_string()),
        ),
        (
            r#"
            --9223372036854775808;  // Attempt to apply - operator to the min of Integer type
        "#,
            Some("Out of range error".to_string()),
        ),
        (
            r#"
            +0;
        "#,
            Some("0".to_string()),
        ),
        (
            r#"
            -0;
        "#,
            Some("0".to_string()),
        ),
        (
            r#"
            +1;
        "#,
            Some("1".to_string()),
        ),
        (
            r#"
            -1;
        "#,
            Some("-1".to_string()),
        ),
        (
            r#"
            ++1;
        "#,
            Some("1".to_string()),
        ),
        (
            r#"
            --1;
        "#,
            Some("1".to_string()),
        ),
        (
            r#"
            -+1;
        "#,
            Some("-1".to_string()),
        ),
        (
            r#"
            +-1;
        "#,
            Some("-1".to_string()),
        ),
        (
            r#"
            + "foo";
        "#,
            Some("Type error".to_string()),
        ),
        (
            r#"
            - true;
        "#,
            Some("Type error".to_string()),
        ),
    ];

    for (src, expected) in tests {
        assert_eq!(interpret(src), expected, "Failed in test case: {}", src);
    }
}

#[test]
fn infix_operator_is_interpreted_correctly() {
    let tests = [
        // Operator: + - * / %
        (
            r#"
            2 + 3;
        "#,
            Some("5".to_string()),
        ),
        (
            r#"
            2 - 3;
        "#,
            Some("-1".to_string()),
        ),
        (
            r#"
            2 * 3;
        "#,
            Some("6".to_string()),
        ),
        (
            r#"
            2 / 3;
        "#,
            Some("0".to_string()),
        ),
        (
            r#"
            2 / 0;
        "#,
            Some("Zero division error".to_string()),
        ),
        (
            r#"
            2 % 3;
        "#,
            Some("2".to_string()),
        ),
        (
            r#"
            2 % 0;
        "#,
            Some("Zero division error".to_string()),
        ),
        (
            r#"
            0 + 0 - 0 * 0;
        "#,
            Some("0".to_string()),
        ),
        (
            r#"
            2 + 3 + 5;
        "#,
            Some("10".to_string()),
        ),
        (
            r#"
            2 + 3 - 5;
        "#,
            Some("0".to_string()),
        ),
        (
            r#"
            2 + 3 * 5;
        "#,
            Some("17".to_string()),
        ),
        (
            r#"
            2 + 3 / 5;
        "#,
            Some("2".to_string()),
        ),
        (
            r#"
            2 + 3 % 5;
        "#,
            Some("5".to_string()),
        ),
        (
            r#"
            2 - 3 + 5;
        "#,
            Some("4".to_string()),
        ),
        (
            r#"
            2 - 3 - 5;
        "#,
            Some("-6".to_string()),
        ),
        (
            r#"
            2 - 3 * 5;
        "#,
            Some("-13".to_string()),
        ),
        (
            r#"
            2 - 3 / 5;
        "#,
            Some("2".to_string()),
        ),
        (
            r#"
            2 - 3 % 5;
        "#,
            Some("-1".to_string()),
        ),
        (
            r#"
            2 * 3 + 5;
        "#,
            Some("11".to_string()),
        ),
        (
            r#"
            2 * 3 - 5;
        "#,
            Some("1".to_string()),
        ),
        (
            r#"
            2 * 3 * 5;
        "#,
            Some("30".to_string()),
        ),
        (
            r#"
            2 * 3 / 5;
        "#,
            Some("1".to_string()),
        ),
        (
            r#"
            2 * 3 % 5;
        "#,
            Some("1".to_string()),
        ),
        (
            r#"
            2 / 3 + 5;
        "#,
            Some("5".to_string()),
        ),
        (
            r#"
            2 / 3 - 5;
        "#,
            Some("-5".to_string()),
        ),
        (
            r#"
            2 / 3 * 5;
        "#,
            Some("0".to_string()),
        ),
        (
            r#"
            2 / 3 / 5;
        "#,
            Some("0".to_string()),
        ),
        (
            r#"
            2 / 3 % 5;
        "#,
            Some("0".to_string()),
        ),
        (
            r#"
            2 % 3 + 5;
        "#,
            Some("7".to_string()),
        ),
        (
            r#"
            2 % 3 - 5;
        "#,
            Some("-3".to_string()),
        ),
        (
            r#"
            2 % 3 * 5;
        "#,
            Some("10".to_string()),
        ),
        (
            r#"
            2 % 3 / 5;
            "#,
            Some("0".to_string()),
        ),
        (
            r#"
            2 % 3 % 5;
            "#,
            Some("2".to_string()),
        ),
        (
            r#"
            1 * + -2;
        "#,
            Some("-2".to_string()),
        ),
        (
            r#"
            1 + * -2; // * is not prefix operator
        "#,
            Some("Syntax error".to_string()),
        ),
        (
            r#"
            "Hello" + "," + " " + "World" + "!";
            "#,
            Some("Hello, World!".to_string()),
        ),
        (
            r#"
            "foo" - "bar";
        "#,
            Some("Type error".to_string()),
        ),
        (
            r#"
            "foo" * "bar";
        "#,
            Some("Type error".to_string()),
        ),
        // Operator: && ||
        (
            r#"
            true && true;
        "#,
            Some("true".to_string()),
        ),
        (
            r#"
            false && false;
        "#,
            Some("false".to_string()),
        ),
        (
            r#"
            true && false;
        "#,
            Some("false".to_string()),
        ),
        (
            r#"
            false || true;
        "#,
            Some("true".to_string()),
        ),
        (
            r#"
            true || true;
        "#,
            Some("true".to_string()),
        ),
        (
            r#"
            false || false;
        "#,
            Some("false".to_string()),
        ),
        (
            r#"
            true || false;
        "#,
            Some("true".to_string()),
        ),
        // Operator: == !-
        (
            r#"
            true == true;
            "#,
            Some("true".to_string()),
        ),
        (
            r#"
            true == false;
            "#,
            Some("false".to_string()),
        ),
        (
            r#"
            0 == 0;
            "#,
            Some("true".to_string()),
        ),
        (
            r#"
            0 == 1;
            "#,
            Some("false".to_string()),
        ),
        (
            r#"
            "foo" == "foo";
            "#,
            Some("true".to_string()),
        ),
        (
            r#"
            "foo" == "bar";
            "#,
            Some("false".to_string()),
        ),
        (
            r#"
            [1,2,3] == [1,2,3];
            "#,
            Some("true".to_string()),
        ),
        (
            r#"
            [1,2,3] == [4,5,6];
            "#,
            Some("false".to_string()),
        ),
        // TODO:
        // (
        //     r#"
        //     none == none;
        //     "#,
        //     Some("true".to_string()),
        // ),
        (
            r#"
            true != true;
            "#,
            Some("false".to_string()),
        ),
        (
            r#"
            true != false;
            "#,
            Some("true".to_string()),
        ),
        (
            r#"
            0 != 0;
            "#,
            Some("false".to_string()),
        ),
        (
            r#"
            0 != 1;
            "#,
            Some("true".to_string()),
        ),
        (
            r#"
            "foo" != "foo";
            "#,
            Some("false".to_string()),
        ),
        (
            r#"
            "foo" != "bar";
            "#,
            Some("true".to_string()),
        ),
        (
            r#"
            [1,2,3] != [1,2,3];
            "#,
            Some("false".to_string()),
        ),
        (
            r#"
            [1,2,3] != [4,5,6];
            "#,
            Some("true".to_string()),
        ),
        // TODO:
        // (
        //     r#"
        //     none != none;
        //     "#,
        //     Some("false".to_string()),
        // ),
        // Operator: > >= < <=
        (
            r#"
            0 > -1;
            "#,
            Some("true".to_string()),
        ),
        (
            r#"
            0 > 0;
            "#,
            Some("false".to_string()),
        ),
        (
            r#"
            0 > 1;
            "#,
            Some("false".to_string()),
        ),
        (
            r#"
            0 >= -1;
            "#,
            Some("true".to_string()),
        ),
        (
            r#"
            0 >= 0;
            "#,
            Some("true".to_string()),
        ),
        (
            r#"
            0 >= 1;
            "#,
            Some("false".to_string()),
        ),
        (
            r#"
            -1 < 0;
            "#,
            Some("true".to_string()),
        ),
        (
            r#"
            0 < 0;
            "#,
            Some("false".to_string()),
        ),
        (
            r#"
            1 < 0;
            "#,
            Some("false".to_string()),
        ),
        (
            r#"
            -1 <= 0;
            "#,
            Some("true".to_string()),
        ),
        (
            r#"
            0 <= 0;
            "#,
            Some("true".to_string()),
        ),
        (
            r#"
            1 <= 0;
            "#,
            Some("false".to_string()),
        ),
        // Type
        (
            r#"
            0 + true;
            "#,
            Some("Type error".to_string()),
        ),
        (
            r#"
            "foo" - 1;
            "#,
            Some("Type error".to_string()),
        ),
        (
            r#"
            0 * [1, 2, 3];
            "#,
            Some("Type error".to_string()),
        ),
        (
            r#"
            none / 1;
            "#,
            Some("Type error".to_string()),
        ),
        (
            r#"
            0 % "foo";
            "#,
            Some("Type error".to_string()),
        ),
        (
            r#"
            "foo" + true;
            "#,
            Some("Type error".to_string()),
        ),
        (
            r#"
            0 + "foo";
            "#,
            Some("Type error".to_string()),
        ),
        (
            r#"
            1 == "foo";
            "#,
            Some("Type error".to_string()),
        ),
        (
            r#"
            "foo" != [1, 2, 3];
            "#,
            Some("Type error".to_string()),
        ),
        (
            r#"
            0 > [1 ,2 ,3];
            "#,
            Some("Type error".to_string()),
        ),
        (
            r#"
            none >= 1;
            "#,
            Some("Type error".to_string()),
        ),
        (
            r#"
            0 < true;
            "#,
            Some("Type error".to_string()),
        ),
        (
            r#"
            "foo" <= 1;
            "#,
            Some("Type error".to_string()),
        ),
        (
            r#"
            1 && true;
            "#,
            Some("Type error".to_string()),
        ),
        (
            r#"
            true && 1;
            "#,
            Some("Type error".to_string()),
        ),
        // (
        //     r#"
        //     false && "foo";
        //     "#,
        //     Some("Type error".to_string()),
        // ),
        (
            r#"
            1 || true;
            "#,
            Some("Type error".to_string()),
        ),
        (
            r#"
            false || 1;
            "#,
            Some("Type error".to_string()),
        ),
        // (
        //     r#"
        //     true || "foo";
        //     "#,
        //     Some("Type error".to_string()),
        // ),
        (
            r#"
            1 = x;
            "#,
            Some("Type error".to_string()), // TODO: This should be syntax error
        ),
    ];
    for (src, expected) in tests {
        assert_eq!(interpret(src), expected, "Failed in test case: {}", src);
    }
}

#[test]
fn postfix_operator_is_interpreted_correctly() {
    let tests = [
        // Operator: []
        (
            r#"
            [1, [2, 22], 3][1];
            "#,
            Some("[2, 22]".to_string()),
        ),
        (
            r#"
            [1, [2, 22], 3][1][1];
            "#,
            Some("22".to_string()),
        ),
        (
            r#"
            [1, [2, [22, 222]], 3][1][1][1];
            "#,
            Some("222".to_string()),
        ),
        (
            r#"
            ["foo", "bar", "baz"][2];
            "#,
            Some("baz".to_string()),
        ),
        (
            r#"
            [1, 2, 3][-1];
            "#,
            Some("Index error".to_string()),
        ),
        (
            r#"
            [1, 2, 3][3];
            "#,
            Some("Index error".to_string()),
        ),
        (
            r#"
            1[1];
            "#,
            Some("Type error".to_string()),
        ),
        // Operator: ()
        (
            r#"
            func arr() {return [1, 2, 3]; }
            arr()[2];
            "#,
            Some("3".to_string()),
        ),
        (
            r#"
            true();
            "#,
            Some("Type error".to_string()),
        ),
        (
            r#"
            var x = true;
            x();
            "#,
            Some("Type error".to_string()),
        ),
    ];
    for (src, expected) in tests {
        assert_eq!(interpret(src), expected, "Failed in test case: {}", src);
    }
}

#[test]
fn operator_precedence_is_interpreted_correctly() {
    let tests = [
        // Prefix precedes infix
        (
            r#"
            -1 + 2;
        "#,
            Some("1".to_string()),
        ),
        (
            r#"
            -+1 - 2;
        "#,
            Some("-3".to_string()),
        ),
        // Postfix preceds prefix
        // TODO: Fix the syntax error
        // (
        //     r#"
        //     -[1, 2, 3][0]
        // "#,
        //     Some("-1".to_string()),
        // ),
        (
            r#"
            func foo() { return 1; }
            -foo();

        "#,
            Some("-1".to_string()),
        ),
        // Parenthesis precedes
        (
            r#"
            (2 + 3) * 5;
        "#,
            Some("25".to_string()),
        ),
        (
            r#"
            (2 + 3) / 5;
        "#,
            Some("1".to_string()),
        ),
        (
            r#"
            (2 + 3) % 5;
        "#,
            Some("0".to_string()),
        ),
        (
            r#"
            (2 - 3) * 5;
        "#,
            Some("-5".to_string()),
        ),
        (
            r#"
            (2 - 3) / 5;
        "#,
            Some("0".to_string()),
        ),
        (
            r#"
            (2 - 3) % 5;
        "#,
            Some("-1".to_string()),
        ),
        (
            r#"
            5 - 3 == 2;
            "#,
            Some("true".to_string()),
        ),
        (
            r#"
            2 == 5 - 3;
            "#,
            Some("true".to_string()),
        ),
    ];

    for (src, expected) in tests {
        assert_eq!(interpret(src), expected, "Failed in test case: {}", src);
    }
}

#[test]
fn variable_is_daclared_and_assigned_value_correctly() {
    let tests = [
        // Declaration and assign operator
        (
            r#"
            var x = 1; // Declare with initialization
            x;
            "#,
            Some("1".to_string()),
        ),
        (
            r#"
            var x; // Declare without initialization
            x;
            "#,
            Some("none".to_string()),
        ),
        (
            r#"
            var x;
            var y;
            x = y = 1; // Right associative
            x;
            "#,
            Some("1".to_string()),
        ),
        (
            r#"
            var x;
            var y;
            x = y = 1; // Right associative
            y;
            "#,
            Some("1".to_string()),
        ),
        (
            r#"
            var x = 1;
            var x = 2; // Redeclaration
            x;
            "#,
            Some("2".to_string()),
        ),
        (
            r#"
            var x = 1;
            x = 2; // Reassignment
            x;
            "#,
            Some("2".to_string()),
        ),
        // Name error
        (
            r#"
            x;
            "#,
            Some("Name error".to_string()),
        ),
        (
            r#"
            foo();
            "#,
            Some("Name error".to_string()),
        ),
        (
            r#"
            x = 1;
            "#,
            Some("Name error".to_string()),
        ),
        // Block scopes
        (
            r#"
            var x = 1;
            {
                x = 2;
            }
            x;
            "#,
            Some("2".to_string()),
        ),
        (
            r#"
            var x = 1;
            {
                var x;
                x = 2;
            }
            x;
            "#,
            Some("1".to_string()),
        ),
        (
            r#"
            var x = 1;
            {
                var x;
                x = 2;
                {
                    x = 3;
                }
            }
            x;
            "#,
            Some("1".to_string()),
        ),
        (
            r#"
            var x = 1;
            {
                var x;
                x = 2;
                {
                    x = 3;
                }
                x;
            }
            "#,
            Some("3".to_string()),
        ),
    ];

    for (src, expected) in tests {
        assert_eq!(interpret(src), expected, "Failed in test case: {}", src);
    }
}

#[test]
fn function_is_declared_and_called_correctly() {
    let tests = [
        // Declaration
        (
            r#"
            func add(x, y) {
                return x + y;
            }
            add(1, 2);
            "#,
            Some("3".to_string()),
        ),
        (
            r#"
            func add(x, y) {
                return x + y;
                return 100; // Not executed
            }
            add(1, 2);
            "#,
            Some("3".to_string()),
        ),
        // Free variable resolution
        (
            r#"
            var y = 1;
            func add(x) {
                return x + y; // y is free variable and lexically resolved
            }
            add(2);
            "#,
            Some("3".to_string()),
        ),
        // Inner functions
        (
            r#"
            func outer(x, y) {
                func inner(x, y) {
                    return x + y;
                }
                return inner(x, y);
            }
            outer(1, 2);
            "#,
            Some("3".to_string()),
        ),
        // Recursive function
        (
            r#"
            func factorial(n) {
                if (n == 0) {
                    return 1;
                }
                return n * factorial(n - 1);
            }
            factorial(5);
            "#,
            Some("120".to_string()),
        ),
        // Function is not an expression
        (
            r#"
            func foo() {return ; }
            foo;
            "#,
            Some("Invalid expression error".to_string()),
        ),
    ];

    for (src, expected) in tests {
        assert_eq!(interpret(src), expected, "Failed in test case: {}", src);
    }
}

#[test]
fn if_statement_controls_flow_correctly() {
    let tests = [
        (
            r#"
            var x = 0;
            if(true) {
                x = 1;
            }
            x;
            "#,
            Some("1".to_string()),
        ),
        (
            r#"
            var x = 0;
            if(false) {
                x = 1;
            }
            x;
            "#,
            Some("0".to_string()),
        ),
        (
            r#"
            var x = 0;
            if(true) {
                x = 1;
                if (true) {
                    x = 2;
                }
            }
            x;
            "#,
            Some("2".to_string()),
        ),
        (
            r#"
            var x = 0;
            if(true) {
                x = 1;
            } else {
                x = 2;
            }
            x;
            "#,
            Some("1".to_string()),
        ),
        (
            r#"
            var x = 0;
            if(false) {
                x = 1;
            } else {
                x = 2;
            }
            x;
            "#,
            Some("2".to_string()),
        ),
        (
            r#"
            var x = 0;
            if(true) {
                x = 1;
            } else if(true) {
                x = 2;
            }
            x;
            "#,
            Some("1".to_string()),
        ),
        (
            r#"
            var x = 0;
            if(false) {
                x = 1;
            } else if(true) {
                x = 2;
            }
            x;
            "#,
            Some("2".to_string()),
        ),
        (
            r#"
            var x = 0;
            if(false) {
                x = 1;
            } else if(false) {
                x = 2;
            }
            x;
            "#,
            Some("0".to_string()),
        ),
    ];

    for (src, expected) in tests {
        assert_eq!(interpret(src), expected, "Failed in test case: {}", src);
    }
}

#[test]
fn for_statement_controls_flow_correctly() {
    let tests = [
        (
            r#"
            var i;
            for (i = 0; i < 10; i = i + 1) {
                if (i == 3 ) {
                    break;
                    i = 100; // Not executed
                }
            }
            i;
            "#,
            Some("3".to_string()),
        ),
        (
            r#"
            var i;
            for (i = 0; i < 10; i = i + 1) {
                if (i == 3 ) {
                    continue;
                    i = 100; // Not executed
                }
            }
            i;
            "#,
            Some("10".to_string()),
        ),
    ];

    for (src, expected) in tests {
        assert_eq!(interpret(src), expected, "Failed in test case: {}", src);
    }
}

#[test]
fn builtin_function_len_works() {
    let tests = [
        (
            r#"
            var arr = [1, 2, 3, "foo", [true]];
            len(arr);
            "#,
            Some("5".to_string()),
        ),
        (
            r#"
            len("Hello, World!");
            "#,
            Some("13".to_string()),
        ),
        // Error case
        (
            r#"
            len();
            "#,
            Some("Argument error".to_string()),
        ),
        (
            r#"
            len([1, 2, 3], [4, 5, 6]);

            "#,
            Some("Argument error".to_string()),
        ),
        (
            r#"
            len(1);
            "#,
            Some("Type error".to_string()),
        ),
    ];

    for (src, expected) in tests {
        assert_eq!(interpret(src), expected, "Failed in test case: {}", src);
    }
}

#[test]
fn builtin_function_push_works() {
    let tests = [
        (
            r#"
            push([1, 2, 3], 4);
            "#,
            Some("[1, 2, 3, 4]".to_string()),
        ),
        // Error case
        (
            r#"
            push();
            "#,
            Some("Argument error".to_string()),
        ),
        (
            r#"
            push([1, 2, 3]);
            "#,
            Some("Argument error".to_string()),
        ),
        (
            r#"
            push("not array", 1);
            "#,
            Some("Type error".to_string()),
        ),
    ];

    for (src, expected) in tests {
        assert_eq!(interpret(src), expected, "Failed in test case: {}", src);
    }
}

#[test]
fn builtin_function_pop_works() {
    let tests = [
        (
            r#"
            pop([1, 2, 3]);
            "#,
            Some("3".to_string()),
        ),
        (
            r#"
            var arr = [1, 2, 3];
            pop(arr);
            pop(arr);
            "#,
            Some("2".to_string()),
        ),
        (
            r#"
            var arr = [1, 2, 3];
            pop(arr);
            pop(arr);
            pop(arr);
            "#,
            Some("1".to_string()),
        ),
        // Error case
        (
            r#"
            var arr = [1, 2, 3];
            pop(arr);
            pop(arr);
            pop(arr);
            pop(arr);
            "#,
            Some("Index error".to_string()),
        ),
        (
            r#"
            pop("not array");
            "#,
            Some("Type error".to_string()),
        ),
    ];

    for (src, expected) in tests {
        assert_eq!(interpret(src), expected, "Failed in test case: {}", src);
    }
}

#[test]
fn generate_type_error() {
    let tests = [];

    for (src, expected) in tests {
        assert_eq!(interpret(src), expected, "Failed in test case: {}", src);
    }
}
