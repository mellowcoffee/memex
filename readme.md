## memex

`memex` is a minimal personal wiki that serves pages rendered from markdown,
with structure appended through the concept of parent and children pages, and
through tracking backlinks using a graph structure

###### roadmap

the current management of the wiki graph through a `HashMap` is ugly and
inefficient, sqlite will be introduced in its place.

###### disclaimer

this software is being actively worked on, most concepts and solutions within
the codebase are provisional and likely inefficient. it is riddled with
`.clone()` and ugly solutions. this is expected to change during development.
