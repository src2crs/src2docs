# Examples and Test Data

This module contains examples for source files that can be parsed by the library.
It adds functionality to the library to create `SourceCode` instances from these files.
It also includes tests to ensure that the parsing works correctly for the examples.

## Adding Examples

When adding examples, take the following into account:

- The examples should be meaningful for the human reader.
- They are meant to be exported from the `Src2Docs` tool
  in order to demonstrate different aspects and use cases of the library.
- Examples may also be useful as a template to start new source documents from.
  E.g. templates for tasks, demos, exams, ...
- Examples are meant for the grand perspective, they should be complete files
  as they might appear in production.
- While examples may be used in integration tests, they are not meant to
  model every aspect of the library. Unit tests should use their own examples.
  
## Notes on the Files

The files are plain text files with `.txt` appended after their usual extensions.
This is to avoid them being treated as Go source files by editors and tools.
Thus, they are not meant to be treated as valid source files
and they should not directly be used for testing.
Any tests or demos should use the `SourceCode` instances created from these files,
either directly or after writing them to the file system.
