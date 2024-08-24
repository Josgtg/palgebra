# Propositional Logic Evaluator

Write propositional logic expressions to check errors and evaluate them.

usage: plogic \<file_name>  
If no file is given, an interactive session will start.

Symbols used are:  
and: &  
or: |  
not: !  
if and only if: ~  
if, then: >

1 and 0 are interpreted as literal simple propositions as well as "false" and "true".

An expression like "true & 0" is correct, and returns false.

An alpabetical letter is interpreted as a variable simple proposition. An expression for its true and false variants will be evaluated.

An expression such as "p > q" will return the evaluation of every possible value of p and q.
