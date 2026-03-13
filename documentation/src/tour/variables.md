# Expressions and Variables

An expression in Water is a statement which can be evaluated. 
This could for example be a constant like the number 3.14 or the word "Cow", or the result of some mathematical operation such as 1 + 3 or 5 < 5.

The value of an expression can be assigned to a variable, by writing the name of the variable followed by
an equal sign and then the expression:

~~~ts
my_var = 5
~~~

## Primitive Types

Every expression (and thus variable) has a type. When assigning the result of an expression to a variable like in the previous example, type will automatically be inferred based on the expression. 
The type of a variable or expression may be explicitly specified using the ":" operator:

~~~ts
my_var1: float = 5

my_var2 = 5: float
~~~

* int
* float
* str
* boolean

## Operators 

Arithmetic operators:

* \+
* \-
* /
* %

Logical operators:

* ==
* !=
* <
* <=
* \>
* \>=
* not
* and
* or

## Other Types of Expressions 

Most constructs in Water are expressions, and can thus may be assigned to variable and passed as arguments to functions.

* assignments
* conditionals
* functions
* types
* matches
* loops
