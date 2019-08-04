1.2:
- using `chrono` crate for date validation - it is now not possible to create PESEL for non-existent day, such as: 29 Feb not in a leap year, 31st of April or 31st of June etc.
- improved Error handling:
    - Errors are now easier to use, as they contain not only message, but also Enum describing the type of error
- out of range check has been fixed
- improved documentation (comments, docstrings and README)

1.1:
- documentation improved (docs & readme)
1.0 - initial implementation