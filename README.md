```
This is an RPN (Reverse Polish Notation) calculator program. This works by pushing numbers to a stack, and then completing operations on the pushed numbers in the order that they were added to the stack.

Example: 4 [enter] 3 [enter] * [enter]
Will display 12 on the Current Stack (i.e. 4 * 3 = 12)

Many more math functions are available:

[Command]            [Function]
q                    quit calculator (ctrl-c also works)
+                    add the last two numbers on the stack
-                    subtract the last two numbers on the stack
* or x               multiply the last two numbers on the stack
/                    divide the last two numbers on the stack
%                    output the remainder of the division of the last two numbers on the stack (modulus)
clear or c           clear the stack of all values
del or d             remove the last number pushed to the stack (this can be repeated multiple times)
floor                take the last two numbers from the stack, and push the lower number back onto the stack
ceil                 take the last two numbers from the stack, and push the higher number back onto the stack
round                round the last number on the stack (follows conventional rules, 0.5 rounds up)
abs                  change the last number on the stack to it's absolute value
pow                  raise the number on the stack prior to the last input to the power of the last number input
                         Example: 3 [enter] 2 [enter] pow [enter] Output: 9
sqrt or v            change the last number on the stack to it's square root
cbrt or v3           change the last number on the stack to it's cube root
ln                   change the last number on the stack to it's natural log
log                  change the last number on the stack to it's base10 log
sin                  change the last number on the stack to it's sine value (radians)
asin, sinh               the same thing, but arc and hyperbolic instead
cos                  change the last number on the stack to it's cosine value (radians)
acos, cosh               the same thing, but arc and hyperbolic instead
tan                  change the last number on the stack to it's tangent value (radians)
atan, tanh               the same thing, but arc and hyperbolic instead
me                   pushes the machine epsilon value onto the stack
pi                   pushes pi onto the stack
tau                  pushes tau on to the stack
help or ?            print this help text
```
