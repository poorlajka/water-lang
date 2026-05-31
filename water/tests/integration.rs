use indoc::indoc;
use std::fs;
use water::{run_capturing, run_capturing_with_dir};

fn write_module(dir: &tempfile::TempDir, name: &str, source: &str) {
    fs::write(dir.path().join(format!("{}.water", name)), source).unwrap();
}

// -- arithmetic --

#[test]
fn integer_addition() {
    assert_eq!(run_capturing("println(1 + 2)"), "3\n");
}

#[test]
fn integer_subtraction() {
    assert_eq!(run_capturing("println(5 - 3)"), "2\n");
}

#[test]
fn integer_multiplication() {
    assert_eq!(run_capturing("println(3 * 4)"), "12\n");
}

#[test]
fn integer_division() {
    assert_eq!(run_capturing("println(10 / 2)"), "5\n");
}

#[test]
fn integer_modulo() {
    assert_eq!(run_capturing("println(7 % 3)"), "1\n");
}

// -- comparisons --

#[test]
fn greater_than() {
    assert_eq!(run_capturing("println(5 > 3)"), "true\n");
}

#[test]
fn less_than_or_equal() {
    assert_eq!(run_capturing("println(3 <= 3)"), "true\n");
}

#[test]
fn not_equal() {
    assert_eq!(run_capturing("println(1 != 2)"), "true\n");
}

// -- variables --

#[test]
fn variable_assignment() {
    assert_eq!(run_capturing("x = 42\nprintln(x)"), "42\n");
}

#[test]
fn variable_reassignment() {
    assert_eq!(run_capturing("x = 1\nx = 2\nprintln(x)"), "2\n");
}

// -- compound assignment --

#[test]
fn plus_eq() {
    assert_eq!(run_capturing("x = 10\nx += 5\nprintln(x)"), "15\n");
}

#[test]
fn minus_eq() {
    assert_eq!(run_capturing("x = 10\nx -= 3\nprintln(x)"), "7\n");
}

#[test]
fn star_eq() {
    assert_eq!(run_capturing("x = 4\nx *= 3\nprintln(x)"), "12\n");
}

#[test]
fn slash_eq() {
    assert_eq!(run_capturing("x = 12\nx /= 4\nprintln(x)"), "3\n");
}

#[test]
fn percent_eq() {
    assert_eq!(run_capturing("x = 7\nx %= 3\nprintln(x)"), "1\n");
}

// -- strings --

#[test]
fn print_string() {
    assert_eq!(run_capturing(r#"println("hello")"#), "hello\n");
}

// -- conditionals --

#[test]
fn if_true_branch() {
    assert_eq!(run_capturing(indoc! {"
        x = 1
        if x == 1
            println(x)
    "}), "1\n");
}

#[test]
fn if_else_true() {
    assert_eq!(run_capturing(indoc! {"
        x = 1
        if x == 1
            println(1)
        else
            println(0)
    "}), "1\n");
}

#[test]
fn if_else_false() {
    assert_eq!(run_capturing(indoc! {"
        x = 0
        if x == 1
            println(1)
        else
            println(0)
    "}), "0\n");
}

// -- while loops --

#[test]
fn while_counts_up() {
    assert_eq!(run_capturing(indoc! {"
        i = 0
        while i < 3
            println(i)
            i = i + 1
    "}), "0\n1\n2\n");
}

#[test]
fn while_skipped_when_false() {
    assert_eq!(run_capturing(indoc! {"
        i = 5
        while i < 3
            println(i)
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
                println(j)
                j = j + 1
            i = i + 1
        println(i)
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
            println(i)
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
            println(i)
    "}), "1\n3\n4\n");
}

// -- power --

#[test]
fn power_basic() {
    assert_eq!(run_capturing("println(2 ** 10)"), "1024\n");
}

#[test]
fn power_zero_exponent() {
    assert_eq!(run_capturing("println(5 ** 0)"), "1\n");
}

#[test]
fn power_right_associative() {
    assert_eq!(run_capturing("println(2 ** 3 ** 2)"), "512\n");
}

// -- booleans --

#[test]
fn bool_literal_true() {
    assert_eq!(run_capturing("println(true)"), "true\n");
}

#[test]
fn bool_literal_false() {
    assert_eq!(run_capturing("println(false)"), "false\n");
}

// -- logical not --

#[test]
fn not_true() {
    assert_eq!(run_capturing("println(!true)"), "false\n");
}

#[test]
fn not_false() {
    assert_eq!(run_capturing("println(!false)"), "true\n");
}

#[test]
fn not_keyword() {
    assert_eq!(run_capturing("println(not true)"), "false\n");
}

// -- unary negation --

#[test]
fn unary_negation() {
    assert_eq!(run_capturing("println(-5)"), "-5\n");
}

#[test]
fn unary_negation_variable() {
    assert_eq!(run_capturing("x = 3\nprintln(-x)"), "-3\n");
}

// -- logical and --

#[test]
fn and_true_true() {
    assert_eq!(run_capturing("println(true and true)"), "true\n");
}

#[test]
fn and_true_false() {
    assert_eq!(run_capturing("println(true and false)"), "false\n");
}

#[test]
fn and_false_true() {
    assert_eq!(run_capturing("println(false and true)"), "false\n");
}

#[test]
fn and_short_circuits() {
    assert_eq!(run_capturing(indoc! {"
        x = 0
        false and (x = 1)
        println(x)
    "}), "0\n");
}

// -- logical or --

#[test]
fn or_false_false() {
    assert_eq!(run_capturing("println(false or false)"), "false\n");
}

#[test]
fn or_false_true() {
    assert_eq!(run_capturing("println(false or true)"), "true\n");
}

#[test]
fn or_true_false() {
    assert_eq!(run_capturing("println(true or false)"), "true\n");
}

#[test]
fn or_short_circuits() {
    assert_eq!(run_capturing(indoc! {"
        x = 0
        true or (x = 1)
        println(x)
    "}), "0\n");
}

// -- arrays --

#[test]
fn array_read_write() {
    assert_eq!(run_capturing(indoc! {"
        a = [10, 20, 30]
        println(a[0])
        println(a[2])
    "}), "10\n30\n");
}

#[test]
fn array_index_assignment() {
    assert_eq!(run_capturing(indoc! {"
        a = [1, 2, 3]
        a[1] = 99
        println(a[0])
        println(a[1])
        println(a[2])
    "}), "1\n99\n3\n");
}

#[test]
fn array_index_assignment_variable_index() {
    assert_eq!(run_capturing(indoc! {"
        a = [0, 0, 0]
        i = 2
        a[i] = 42
        println(a[i])
    "}), "42\n");
}

#[test]
fn array_multiline_literal() {
    assert_eq!(run_capturing(indoc! {"
        a = [1, 2,
             3, 4]
        println(a[0])
        println(a[2])
    "}), "1\n3\n");
}

#[test]
fn array_2d_read_write() {
    assert_eq!(run_capturing(indoc! {"
        grid = [[1, 2], [3, 4]]
        println(grid[0][1])
        grid[1][0] = 99
        println(grid[1][0])
        println(grid[1][1])
    "}), "2\n99\n4\n");
}

#[test]
fn array_2d_multiline() {
    assert_eq!(run_capturing(indoc! {"
        grid = [
            [1, 2, 3],
            [4, 5, 6]
        ]
        println(grid[0][2])
        println(grid[1][1])
    "}), "3\n5\n");
}

#[test]
fn array_3d_read_write() {
    assert_eq!(run_capturing(indoc! {"
        cube = [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]
        println(cube[0][1][0])
        cube[1][0][1] = 99
        println(cube[1][0][1])
    "}), "3\n99\n");
}

// -- functions --

#[test]
fn function_no_args() {
    assert_eq!(run_capturing(indoc! {"
        greet = () => 42
        println(greet())
    "}), "42\n");
}

#[test]
fn function_two_args() {
    assert_eq!(run_capturing(indoc! {"
        add = (a, b) => a + b
        println(add(3, 4))
    "}), "7\n");
}

#[test]
fn function_three_args() {
    assert_eq!(run_capturing(indoc! {"
        sum3 = (a, b, c) => a + b + c
        println(sum3(1, 2, 3))
    "}), "6\n");
}

#[test]
fn function_calls_function() {
    assert_eq!(run_capturing(indoc! {"
        double = (x) => x * 2
        quad = (x) => double(double(x))
        println(quad(3))
    "}), "12\n");
}

#[test]
fn recursive_function() {
    assert_eq!(run_capturing(indoc! {"
        fact = (n) =>
            if n == 0
                return 1
            return n * fact(n - 1)
        println(fact(5))
    "}), "120\n");
}

// -- imports --

#[test]
fn import_from_single() {
    let dir = tempfile::tempdir().unwrap();
    write_module(&dir, "math", "add = (a, b) => a + b");
    assert_eq!(
        run_capturing_with_dir("from math import add\nprintln(add(3, 4))", Some(dir.path())),
        "7\n"
    );
}

#[test]
fn import_from_alias() {
    let dir = tempfile::tempdir().unwrap();
    write_module(&dir, "math", "add = (a, b) => a + b");
    assert_eq!(
        run_capturing_with_dir("from math import add as plus\nprintln(plus(10, 5))", Some(dir.path())),
        "15\n"
    );
}

#[test]
fn import_from_multiple() {
    let dir = tempfile::tempdir().unwrap();
    write_module(&dir, "math", indoc! {"
        add = (a, b) => a + b
        mul = (a, b) => a * b
    "});
    assert_eq!(
        run_capturing_with_dir(indoc! {"
            from math import add, mul
            println(add(2, 3))
            println(mul(2, 3))
        "}, Some(dir.path())),
        "5\n6\n"
    );
}

#[test]
fn import_module_dot_access() {
    let dir = tempfile::tempdir().unwrap();
    write_module(&dir, "math", "square = (x) => x * x");
    assert_eq!(
        run_capturing_with_dir("import math\nprintln(math.square(5))", Some(dir.path())),
        "25\n"
    );
}

#[test]
fn import_module_with_alias() {
    let dir = tempfile::tempdir().unwrap();
    write_module(&dir, "math", "double = (x) => x * 2");
    assert_eq!(
        run_capturing_with_dir("import math as m\nprintln(m.double(6))", Some(dir.path())),
        "12\n"
    );
}

#[test]
fn import_transitive() {
    let dir = tempfile::tempdir().unwrap();
    write_module(&dir, "base", "inc = (x) => x + 1");
    write_module(&dir, "utils", "from base import inc\ndouble_inc = (x) => inc(inc(x))");
    assert_eq!(
        run_capturing_with_dir("from utils import double_inc\nprintln(double_inc(5))", Some(dir.path())),
        "7\n"
    );
}
