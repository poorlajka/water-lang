# Functions

~~~ ruby
check_age = (name, age) =>
    if age < 18
        print("Sorry {name}, you have to be atlest 18 to buy alcohol.")
    else
        print("Sure {name}, what would you like?")

check_age("Mclovin", 32)
~~~

~~~ ruby
check_age = (name: str, age: int) => none
    if age < 18
        print("Sorry {name}, you have to be atlest 18 to buy alcohol.")
    else
        print("Sure {name}, what would you like?")

check_age("Mclovin", 32)
check_age("Mclovin", () => if 1 < 2 then 32 else 5)
~~~