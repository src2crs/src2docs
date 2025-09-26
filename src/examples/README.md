# Examples and Test Data

This module contains examples for source files that can be parsed by the library.
It adds functionality to the library to create `SourceCode` instances from these files.
It also includes tests to ensure that the parsing works correctly for the examples.

The files are plain text files with `.txt` appended after their usual extensions.
This is to avoid them being treated as Go source files by editors and tools.
Thus, they are not meant to be treated as valid source files
and they should not directly be used for testing.
Any tests or demos should use the `SourceCode` instances created from these files,
either directly or after writing them to the file system.
