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
- [x] Compressable strings (save those bytes) (Kinda broken rn)
- [X] Add floating point operation
- [ ] if (`?`), else (`|`), loop(`@`), break (`;`)
- [ ] Arrays
- [ ] Std input (Std output is done)
- [ ] A crap-load of high level functions


## Token reference 

### Implemented

|Symbol|Name|Pops|Description|
|------|----|-----|-----------------------------------|
|`,`|println|`pop(a)`|prints `a` to `stdout` (w/ newline)|
|`P`|print|`pop(a)`|prints `a` to `stdout` (w/o newline)|
|`1` (and other digits) |int|`push(a)`|pushes `1` to stack|
|`CMD`|int|`N/A`|converts roman numeral to hindu (`CMD >> 1400`) ans pushes to stack|
|`"string literal"`|str|`push(a)`|pushes `"string literal"` to stack|
|\`compressed string literal\`|cmp_str|`push(a)`|decompresses & pushes `"string literal"` to stack|
|`cmp`|compress|`pop(a)`|pops `a` then compresses then pushes `cmp(a)` to stack|
|`+`|add|`pop(a,b)`|pops `a` then `b` then pushes `a+b` to stack|
|`-`|sub|`pop(a,b)`|pops `a` then `b` then pushes `a-b` to stack|
|`/`|div|`pop(a,b)`|pops `a` then `b` then pushes `a/b` to stack|
|`*`|multi|`pop(a,b)`|pops `a` then `b` then pushes `a*b` to stack|
|`%`|mod|`pop(a,b)`|pops `a` then `b` then pushes `a%b` to stack|
|`!`|factorial|`pop(a)`|pops `a` then pushes `a!` to stack|

#### Roman Numerals 
|Symbol|Name|Pops|Description|
|------|----|-----|-----------------------------------|
|`Mk`|1 Million|`push(a)`|pushes `1_000_000` to stack|
|`CMk`|900k|`push(a)`|pushes `900_000` to stack|
|`Dk`|500k|`push(a)`|pushes `500_000` to stack|
|`CDk`|400k|`push(a)`|pushes `400_000` to stack|
|`Ck`|100k|`push(a)`|pushes `100_000` to stack|
|`XCk`|90k|`push(a)`|pushes `90_000` to stack|
|`Lk`|50k|`push(a)`|pushes `50_000` to stack|
|`XLk`|40k|`push(a)`|pushes `40_000` to stack|
|`Xk`|10k|`push(a)`|pushes `10_000` to stack|
|`IXk`|9k|`push(a)`|pushes `9_000` to stack|
|`Vk`|5k|`push(a)`|pushes `5_000` to stack|
|`M`|1k|`push(a)`|pushes `1_000` to stack|
|`CM`|900|`push(a)`|pushes `900` to stack|
|`D`|500|`push(a)`|pushes `500` to stack|
|`CD`|400|`push(a)`|pushes `400` to stack|
|`XC`|90|`push(a)`|pushes `90` to stack|
|`L`|50|`push(a)`|pushes `50` to stack|
|`XL`|40|`push(a)`|pushes `40` to stack|
|`X`|10|`push(a)`|pushes `10` to stack|
|`IX`|9|`push(a)`|pushes `9` to stack|
|`V`|5|`push(a)`|pushes `5` to stack|
|`IV`|4|`push(a)`|pushes `4` to stack|
|`I`|1|`push(a)`|pushes `1` to stack|


### Planned

|Symbol|Name|Pops|Description|
|------|----|-----|-----------------------------------|
|`?`|if|`pop(a, b, c)`|pops `a` (expression) then `b` and/or `c` (execs based on `a`)|
|`\|`|else|`pop(a, b, c)`| pops `a` (expression) then `b` and/or `c` (execs based on `a`)|
|`@`|loop|`N/A`| loops until break (`;`))|
|`;`|break|`N/A`| breaks out of active loop)|
