# Putt

A **work in progress** stack-based, [golfing](https://en.wikipedia.org/wiki/Code_golf) language.

## Goals
- Easy to type (no copy-pasting unicode, **ever!**)
- Turing-completeness
- Use minimal resources

## Example code 


```putt
"Hello, World!".
```
```
Hello, World!
```

```putt
"WeCanDoMath",,"Times",10 8*,"Division",5 80/,"Addition",20 10+,"Subtraction",20 10-,"Modulo",10 20%,
```

```
WeCanDoMath

Times
80
Division
16
Addition
30
Subtraction
-10
Modulo
0
```

## TODO:
- [x] Fix strings (~~right now~~ was Alphanumeric only) 
- [ ] Compressable strings (save those bytes)
- [ ] Add floating point operation
- [ ] if (`?`), else (`|`), loop(`@`), break (`;`)
- [ ] Arrays
- [ ] Std input (Std output is done)
- [ ] A crap-load of functions


## Token reference 

### Implemented

|Symbol|Name|Pops|Description|
|------|----|-----|-----------------------------------|
|`,`|println|`pop(a)`|prints `a` to `stdout` (w/ newline)|
|`.`|print|`pop(a)`|prints `a` to `stdout` (w/o newline)|
|`1` (and other digits) |int|`push(a)`|pushes `1` to stack|
|`CMD`|int|`N/A`|converts roman numeral to hindu (`CMD >> 1400`) ans pushes to stack|
|`"string literal"`|str|`push(a)`|pushes `"string literal"` to stack|
|`\`compressed string literal\``|cmp_str|`push(a)`|decompresses & pushes `"string literal"` to stack|
|`cmp`|compress|`pop(a)`|pops `a` then compresses then pushes `cmp(a)` to stack|
|`+`|add|`pop(a,b)`|pops `a` then `b` then pushes `a+b` to stack|
|`-`|sub|`pop(a,b)`|pops `a` then `b` then pushes `a-b` to stack|
|`/`|div|`pop(a,b)`|pops `a` then `b` then pushes `a/b` to stack|
|`*`|multi|`pop(a,b)`|pops `a` then `b` then pushes `a*b` to stack|
|`%`|mod|`pop(a,b)`|pops `a` then `b` then pushes `a%b` to stack|
|`!`|factorial|`pop(a)`|pops `a` then pushes `a!` to stack|


### Planned

|Symbol|Name|Pops|Description|
|------|----|-----|-----------------------------------|
|`?`|if|`pop(a, b, c)`|pops `a` (expression) then `b` and/or `c` (execs based on `a`)|
|`\|`|else|`pop(a, b, c)`| pops `a` (expression) then `b` and/or `c` (execs based on `a`)|
|`@`|loop|`N/A`| loops until break (`;`))|
|`;`|break|`N/A`| breaks out of active loop)|
