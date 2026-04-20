arguments is impossible. See section 2.2.5 for an example of how you can make this work.

### 2.2.4 DSLs

Declarative macros are also good for creating DSLs. As mentioned in the introduction, DSLs encapsulate knowledge about the domain, that the developers have learned from domain experts, into the application. This idea of capturing domain knowledge is related to that of a ubiquitous language, which comes from Domain-Driven Design and argues that communication between experts and developers is easier when they use the same words and concepts. This leads to better code.

The goal of a DSL is to create a specialized language that is suitable to the domain and hides irrelevant complexities. Among these complexities could be things like additional validation and taking care of certain subtleties. Some think that a DSL could make a codebase understandable to nonprogrammers, and one idea behind testing frameworks like Cucumber (https://github.com/cucumber-rs/cucumber) is to write tests in a language that the domain experts could understand. Another idea is that experts would even be able to add their own tests using these tools! Within the Rust ecosystem, mini-DSLs abound, and they are often created by using macros. Two simple examples from the standard library are `println!` and `format!`, which offer a special syntax using curly braces to determine how to print specified variables.

As an example, we will write a DSL for handling transfers between accounts. First, we create an `Account` struct that contains the amount of money in our account. It also has methods for adding and removing money from the account. We only want positive amounts (hence the use of `u32`), but handling accounts going into the negative is beyond scope. `Account` also derives `Debug`. This is a good idea in general, though our motivation here is simply to print some outcomes.

Listing 2.12 The `Account` structure

```rust
use std::ops::{Add, Sub};

#[derive(Debug)]
struct Account {
    money: u32,
}

impl Account {
    fn add(&mut self, money: u32) {
        self.money = self.money.add(money)
    }

    fn subtract(&mut self, money: u32) {
        self.money = self.money.sub(money)
    }
}
```

#1 The Add and Sub traits allow us to use the add and sub methods on u32 inside the Account implementation below.

Now check out the `exchange` macro in listing 2.13, which presents users with a mini-DSL. As you can see, this macro allows us to use natural language, understandable to outsiders, to describe actions. And when the macro does not understand a command, it will complain about this at compile time. Also, when transferring money between two accounts (third pair), we are hiding the complexity of a transaction from the DSL user.

Listing 2.13 Macro that presents a mini-DSL for exchanging money

```rust
macro_rules! exchange {
    (Give $amount:literal to $name:ident) => {
        $name.add($amount)
    };
    (Take $amount:literal from $name:ident) => {
        $name.subtract($amount)
    };
    (Give $amount:literal from $giver:ident to $receiver:ident) => {
        $giver.subtract($amount);
        $receiver.add($amount)
    };
}

fn main() {
    let mut the_poor = Account {
        money: 0,
    };
    let mut the_rich = Account {
        money: 200,
    };

    exchange!(Give 20 to the_poor);
    exchange!(Take 10 from the_rich);
    exchange!(Give 30 from the_rich to the_poor);
    println!("Poor: {the_poor:?}, rich: {the_rich:?}")
}
```

#1 We have to end $giver.subtract($amount) with a semicolon because the transcriber has two statements instead of one, and the compiler would complain about expecting a ;.

#2 Uses natural language to specify transactions

#3 This prints "Poor: Account { money: 50 }, rich: Account { money: 160 }".

And there is no need to stop here. You could add currency types and automatically convert them. Or you could have special rules for overdraft. Good testing is needed to make sure that your macro scenarios do not “collide” (which is when you think you will land in matcher X but you end up in Y) with each other, though. As a simple example, the following macro is meant for giving money to the poor and berates people who don’t give anything. We call our macro and are complimented for giving nothing. That’s because the first clause accepts any literal, which includes zero. And since the macro checks the matcher in order and always matches the first more general clause, the second one is never reached.

Listing 2.14 A faulty wealth-transfer macro

```rust
macro_rules! give_money_to_the_poor {
    (Give $example:literal) => {
        println!("How generous");
    };
    (Give 0) => {
        println!("Cheapskate");
    };
}

fn main() {
    give_money_to_the_poor!(Give 0);
}
```

#1 Prints "How generous"! That is not what we expected.

The solution, in this case, is simple. Just switch the order of the cases! The underlying rule is this: write macro rules/matchers from most-specific to least-specific. This rule also applies to pattern matching with `match`, though the order is not enforced by the compiler, which only produces a warning. (Try putting the catchall `_` before other matches.) More worrisome is ignoring this rule still results in a valid macro implementation according to Rust, which won’t even produce a warning. So static checks are insufficient to root out these bugs, and you should test all your “branches.”
