# KiwiBinaryC - CLI Base Converter

A Simple Binary CLI converter

## Flags

1. -b(--base) sets the base for conversion
2. -n(--number) sets the number for conversion
3. -c(--code) sets the already converted code, combined with base to output base 10 number (mutally exclusive to --number)
4. -t(--target) sets the target base to conversion, only used in alongside -c and -b to convert another supported base to the target base;

<br>

## Supported Bases

- Hexadecimal(hex)
- Octal(oct)
- Binary(bin)

<br>

## Notes

Remeber to use lowercase while entering the arguments for the CLI!
It currently doesn't have noramlization for inputs

---

## Errors

Raises ConverterError with a colorful Error message when hit,

<br>

## Examples

```bash
- --base hex --code BEEF -> 48879 #(it's in base 10)
- --base octal --code 44323 -> 18643 #(base 10 again)
- --base bin --number 120 -> 1111000 #(it's in binary)
- --base hex --code BEEFED01 --target octal --> 27673766401 #(in octal)
- -b hex -c 9323DEF -> 1543287599 #(since we give it -c, which is code, it output a base 10)
```
