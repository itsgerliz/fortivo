# fortivo
## About
If you ever thought:\
"Damn I have a lot of passwords for multiple different accounts, I really don't know how to keep this organized"\
Then, Fortivo might be for you, it lets you store your secrets (they don't have to be passwords) in files with a specific custom format

## Terminology
A Fortivo's file where you save your secrets its called ``Arca`` while the secrets inside it are called ``Arcanum(s)``

## Design principles
- Fortivo does NOT assume anything, i.e., when creating a new Arca, if you dont explicitly say when asked that you want it to be encrypted Fortivo will NOT encrypt it
- Fortivo does NOT depend on any third party<sup>[[1]](#considerations)</sup> , i.e., Fortivo will NOT require things like network access and is completely isolated 

## Licensing
This software is licensed under the BSD-3-Clause license
- Read it locally in file ``LICENSE``
- Or online [here](https://github.com/itsgerliz/fortivo/blob/master/LICENSE)

## Considerations
- [1]: Does not include software libraries needed to build or run Fortivo itself

---

© Copyright 2025 Germán R. V. ([@itsgerliz](https://github.com/itsgerliz))\
Released under the BSD-3-Clause License