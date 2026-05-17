use indoc::indoc;
use water::run_capturing;

// -- arithmetic --

#[test]
fn integer_addition() {
    assert_eq!(run_capturing("print(1 + 2)"), "3\n");
}

#[test]
fn integer_subtraction() {
    assert_eq!(run_capturing("print(5 - 3)"), "2\n");
}

#[test]
fn integer_multiplication() {
    assert_eq!(run_capturing("print(3 * 4)"), "12\n");
}

#[test]
fn integer_division() {
    assert_eq!(run_capturing("print(10 / 2)"), "5\n");
}

#[test]
fn integer_modulo() {
    assert_eq!(run_capturing("print(7 % 3)"), "1\n");
}

// -- comparisons --

#[test]
fn greater_than() {
    assert_eq!(run_capturing("print(5 > 3)"), "true\n");
}

#[test]
fn less_than_or_equal() {
    assert_eq!(run_capturing("print(3 <= 3)"), "true\n");
}

#[test]
fn not_equal() {
    assert_eq!(run_capturing("print(1 != 2)"), "true\n");
}

// -- variables --

#[test]
fn variable_assignment() {
    assert_eq!(run_capturing("x = 42\nprint(x)"), "42\n");
}

#[test]
fn variable_reassignment() {
    assert_eq!(run_capturing("x = 1\nx = 2\nprint(x)"), "2\n");
}

// -- strings --

#[test]
fn print_string() {
    assert_eq!(run_capturing(r#"print("hello")"#), "hello\n");
}

// -- conditionals --

#[test]
fn if_true_branch() {
    assert_eq!(run_capturing(indoc! {"
        x = 1
        if x == 1
            print(x)
    "}), "1\n");
}

#[test]
fn if_else_true() {
    assert_eq!(run_capturing(indoc! {"
        x = 1
        if x == 1
            print(1)
        else
            print(0)
    "}), "1\n");
}

#[test]
fn if_else_false() {
    assert_eq!(run_capturing(indoc! {"
        x = 0
        if x == 1
            print(1)
        else
            print(0)
    "}), "0\n");
}

// -- while loops --

#[test]
fn while_counts_up() {
    assert_eq!(run_capturing(indoc! {"
        i = 0
        while i < 3
            print(i)
            i = i + 1
    "}), "0\n1\n2\n");
}

#[test]
fn while_skipped_when_false() {
    assert_eq!(run_capturing(indoc! {"
        i = 5
        while i < 3
            print(i)
    "}), "");
}

#[test]
fn nested_loops_break_inner_only() {
    assert_eq!(run_capturing(indoc! {"
        i = 0
        while i < 3
            j = 0
            while j < 3
                if j == 1
                    break
                print(j)
                j = j + 1
            i = i + 1
        print(i)
    "}), "0\n0\n0\n3\n");
}

// -- break --

#[test]
fn break_exits_loop() {
    assert_eq!(run_capturing(indoc! {"
        i = 0
        while i < 5
            if i == 2
                break
            print(i)
            i = i + 1
    "}), "0\n1\n");
}

// -- continue --

#[test]
fn continue_skips_iteration() {
    assert_eq!(run_capturing(indoc! {"
        i = 0
        while i < 4
            i = i + 1
            if i == 2
                continue
            print(i)
    "}), "1\n3\n4\n");
}

// -- arrays --

#[test]
fn array_read_write() {
    assert_eq!(run_capturing(indoc! {"
        a = [10, 20, 30]
        print(a[0])
        print(a[2])
    "}), "10\n30\n");
}

// -- functions --

#[test]
fn function_no_args() {
    assert_eq!(run_capturing(indoc! {"
        greet = () => 42
        print(greet())
    "}), "42\n");
}

#[test]
fn function_two_args() {
    assert_eq!(run_capturing(indoc! {"
        add = (a, b) => a + b
        print(add(3, 4))
    "}), "7\n");
}

#[test]
fn function_three_args() {
    assert_eq!(run_capturing(indoc! {"
        sum3 = (a, b, c) => a + b + c
        print(sum3(1, 2, 3))
    "}), "6\n");
}

#[test]
fn function_calls_function() {
    assert_eq!(run_capturing(indoc! {"
        double = (x) => x * 2
        quad = (x) => double(double(x))
        print(quad(3))
    "}), "12\n");
}

#[test]
fn recursive_function() {
    assert_eq!(run_capturing(indoc! {"
        fact = (n) =>
            if n == 0
                return 1
            return n * fact(n - 1)
        print(fact(5))
    "}), "120\n");
}
