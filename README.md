# Source2Documents

This is a library for parsing source code into a document model
for use in other Src2Crs projects.

The idea is to define tasks, exams, examples, or similar things that can be part of a programming course in proper source code files
that can be compiled and tested using a normal development toolchain.

E.g. a file defining a task would include the task description, hints and a solution, or an example for use in slides might mark the code to be shown, or possibly define slide overlays.
This is done using special comments that define tags or special sections of code.
