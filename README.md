# packs
A [PackStream](https://7687.org/packstream/packstream-specification-1.html) implementation written in Rust.

| Latest Version | Supported Version |
|------------------------------------|
|    1           |   1               |

## Overview
PackStream is a streamable binary format for a range of data types,
used by the [bolt protocol](https://7687.org/#bolt) – the protocol of
the graph database neo4j. 
It is meant to be space efficient. It supports 

- booleans,
- strings,
- integers,
- floats,
- lists,
- dictionaries
- byte arrays

and structures built up out of these primitive types, up to 15 fields.

This library 
