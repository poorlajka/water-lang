# Scopes

The first time an expression is assigned to a variable it gets declared for the current scope.
Aditional assignments to the variable within that scope will mutate the variable.
If you wish to create a variable by the same name for the current scope, i.e redeclare a variable
you can use the wallrus operator ":=". 


~~~ js
global = 1

my_func = () => {
    global += 1 // This will mutate since the variable was declared in the global scope

    global := 0 // Redeclaring the variable untill the end of the function 
    global += 50000
    print(global) // This will print 50000
}

my_func()
print(global) // This will print 2
~~~

This can also be used for function overloading.