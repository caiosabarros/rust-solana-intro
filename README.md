# Rust Owned Calculator Program

# Overview

This is an introductory program in Rust for those aiming to add to or move from the Solidity language
and also build smart contracts in the Solana blockchain.

This is an owned calculator. It performs the basic operations,
- add 
- sub
- div
- mul
- multiple_add

However, only a specific signer may call the function mul. This is intentional because we want to demonstrate how to apply the famous Solidity `onlyOwner` modifier in Solana programs.

The operations of add and sub are checked for overflow, which means they will panic if they get an malicious input.

The other operations are used without the security checks. Contributions through PRs are welcome to make this available.

## Motivation

The intention is to demonstrate how to apply the famous Solidity `onlyOwner` modifier in Solana programs, where only the owner of the program may call the mul function. The owner is usually set to be the creator of the program, but it may as well be set to correspond to any address in the Solana blockchain.

Along the way I learned a lot about how Rust syntax can be "translated" into Solidity and vice-versa.

## WalkThrough

[Solana Owned Calculator Demo Video](https://www.youtube.com/watch?v=5_0WghdrOyk)

## Development Environment

It is assumed the user has the following tools installed
- Solana CLI
- Anchor
- Chai
- Typescript
- Yarn
- Rust

All of these can be downloaded following the instructions at [RareSkills Hello World Solana](https://www.rareskills.io/post/hello-world-solana)

## Rust

Rust uses lots of concepts, but it does not have a concept of a `contract` or class-like type similar to Solidity or JS because it is not an OOP programming language. It works using modules, functions, structs and it is statically typed.

## Useful Websites

- [Rust Docs](https://doc.rust-lang.org/reference/introduction.html)
- [RareSkills Solana Course](https://www.rareskills.io/solana-tutorial#solana-course)

## Future Work

- Test good scenarios for all the functions
- Test bad scenarios for all the functions
- Add security checks according to tests
