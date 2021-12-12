# str-validator

A library of string validators and sanitizers.

This project tries to migrate the [validator.js](https://github.com/validatorjs/validator.js) library written in javascript to the rust programming language.

The main goal is to help me with the rust language. Anyone who wishes to participate in any way is welcome :), either helping with the migration or providing comments, suggestions, corrections, etc.

## Validators

Here is a list of the validators currently available.

Validator                               | Description
--------------------------------------- | --------------------------------------
**contains(str, seed [, options ])**    | check if the string contains the seed.<br/><br/>`options` is an object that defaults to `{ ignoreCase: false, minOccurrences: 1 }`.<br />Options: <br/> `ignoreCase`: Ignore case when doing comparison, default false<br/>`minOccurences`: Minimum number of occurrences for the seed in the string. Defaults to 1.