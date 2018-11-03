# Stabacus

This is stack based calculator to be run in a terminal. See the mockup for a
rough idea of what the interface looks like (obviously subject to change).

## Operation

Right now you can enter integers or one of the commands listed below. Currently
you can only enter a single token at a time delimited by newlines. More flexible
input is on the TODO list.

## Commands

These are the currently implemented commands:
- binary operations: +, *, -, /, %
- nary operations: sum and prod
- control commands: pop and q

The nary operations work by taking the top element of the stack to be the
desired arity, and then applying the corresponding operation the the following n
elements of the stack.

pop discards the top element of the stack, and q quits.

## Planned Features

See the idea document for a full list. Briefly, floating point numbers, multiple
stacks, user defined commands, and a reasonably comprehensive starting library
of builtin commands.

## License

Stabacus is licensed under GPLv3.0 or later; see COPYING for details.

