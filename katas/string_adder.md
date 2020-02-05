# String Adder Kata

## Background

You are working on a project that needs to target a strange new system. Unfortunately the new system's ALU only works on 5-bit registers, or in other words, signed nibbles. This means that any atomic additions or subtractions in your code can only accept operands in the range of ±F (±15 in decimal). This rather limits your program's ability to work with any numbers greater than 15 or less than -15 and as such it is decided that numbers will be stored as strings.

## Part 1

Your task is to create a function that will accept two string arguments, `a` and `b`, which hold decimal representations of integers, add the two together using only additions with operands of magnitude < 16, and return the decimal representation of the result in a string. Essentially you are unable to just convert the arguments to integers, add them, convert them back to a string and return them (except in cases where both `a` and `b` are with the range ±15). For now you are only required to produce a function that accepts positive integer string and returns a positive integer strings.

### Acceptance Criteria

1. The function `string_add` should accept two strings `a` and `b` and return a string.
2. Given two positive integers `string_add` should return a decimal string representation of the result of adding the two integers. Example cases:
   1. Given `"1"` and `"1"` return `"2"`
   2. Given `"123"` and `"456"` return `"579"`
   3. Given `"67587"` and `"78122"` return `"145709"`

## Part 2

You successfully implement the new `string_add` function and the rest of your team begins using it. Sadly the software is now prone to crashes whenever `string_add` is called with arguments that cause undefined behaviour. You are now tasked with updating your function to handle these cases and return a `Result` which can either be `Ok(String)` or `Err(StringAddError)`. Your function signature should then look something like this:

```rust
fn string_add(a: &str, b: &str) -> Result<String, StringAddError>;
```

Proper handling of errors should be include:

1. Given empty strings for either or both arguments should return an `EmptyString` error.
2. Given negative values for either or both arguments should return a `NegativeArgument` error.
3. Given non-decimal strings (i.e. a string which contains any char other than 0-9) should return a `NotADecimal` error.

## Part 3

Your error handling is well-received and the rest of your team are happy... until they realise they need to _subtract_ numbers from each other too. You are now tasked with updating your function to accept negative decimal strings as arguments. You are also given some example cases to get you started:

- Given `"5"` and `"-3"` return `"2"`
- Given `"20"` and `"-30"` return `"-10"`
- Given `"-25"` and `"25"` return `"0"`

## Part 4

Now that your team is able to subtract numbers the team's velocity goes through the roof and the board of investors are impressed. Everyone is happy. That is, except for Mark. Mark is working on a part of the code that deals with hexadecimal values very frequently and he's getting tired of the tedium of converting the hex values to dec, running the add function and then converting the result back to hex. He's asked you to allow the function to accept hex string arguments as well as decimal.

You must now update your function to accept strings that start with `"0x"`, and if they do, apply the addition as usual but this time return the result in hexadecimal form. So if for example the first argument is hexadecimal but the second is decimal, you should still return the result as hexadecimal, and likewise if the first argument is decimal but the second is hexadecimal. The only case in which you will continue to return decimal values is when both `a` and `b` are in decimal form.
