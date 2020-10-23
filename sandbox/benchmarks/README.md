# Ideas

## Type inference performance hit

What is the performance hit during build time due to type-inference?
- try writing a code with X thousands of lines like:
```
let x1: u8 = 1;
let x2: u32 = 2;
...
```
and compare build times to same code _without_ the types

```
let x1 = 1;
let x2 = 2;
...
```

Expectation is that the second one will have higher expected build time (even if only slightly).


