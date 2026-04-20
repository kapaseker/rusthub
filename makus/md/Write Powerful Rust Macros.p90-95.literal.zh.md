arguments 是不可能的。关于如何实现这一点，请参见 2.2.5 节中的示例。

### 2.2.4 DSL

声明式宏也很适合创建 DSL。正如引言中提到的，DSL 会把开发者从领域专家那里学到的关于该领域的知识封装进应用程序中。捕获领域知识这一想法，与通用语言的思想有关；后者来自领域驱动设计，它主张当专家和开发者使用相同的词汇与概念时，彼此沟通会更容易。这会带来更好的代码。

DSL 的目标，是创建一种适合该领域并隐藏无关复杂性的专用语言。这些复杂性可能包括诸如额外校验、处理某些细微差别之类的事情。有些人认为，DSL 可以让非程序员也能理解代码库；像 Cucumber 这样的测试框架（https://github.com/cucumber-rs/cucumber）背后的一个想法，就是用领域专家也能理解的语言来编写测试。另一个想法是，专家甚至可以借助这些工具自己添加测试！在 Rust 生态中，mini-DSL 随处可见，而它们通常是借助宏创建的。标准库中的两个简单例子是 `println!` 和 `format!`，它们提供了一种使用花括号来决定如何打印指定变量的特殊语法。

作为一个示例，我们将编写一个用于处理账户之间转账的 DSL。首先，我们创建一个 `Account` 结构体，用来保存账户中的金额。它还带有向账户中增加金额和从账户中移除金额的方法。我们只希望金额为正数（因此使用 `u32`），但处理账户变成负数的情况超出了这里的范围。`Account` 还派生了 `Debug`。总体来说这是个好主意，不过我们这里这样做的动机只是为了打印一些结果。

清单 2.12 `Account` 结构

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

#1 `Add` 和 `Sub` trait 使我们能够在下面的 `Account` 实现中，对 `u32` 使用 `add` 和 `sub` 方法。

现在来看清单 2.13 中的 `exchange` 宏，它向用户呈现了一个 mini-DSL。如你所见，这个宏允许我们使用外部人员也能理解的自然语言来描述动作。而且当这个宏无法理解某条命令时，它会在编译期对此提出报错。另外，当在两个账户之间转账时（第三对规则），我们把一次交易的复杂性隐藏在了 DSL 用户看不到的地方。

清单 2.13 呈现用于转账的 mini-DSL 的宏

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

#1 我们必须在 `$giver.subtract($amount)` 后面加上分号，因为这个转写器这里有两条语句而不是一条，而编译器会抱怨这里本应出现一个 `;`。

#2 使用自然语言来指定交易

#3 这会打印出 `"Poor: Account { money: 50 }, rich: Account { money: 160 }"`。

而且并没有必要止步于此。你可以加入货币类型并自动完成转换。或者你也可以为透支添加特殊规则。不过，为了确保你的宏场景彼此不会“碰撞”（也就是你以为会落到匹配器 X，结果却落到了 Y），良好的测试是必需的。举个简单的例子，下面这个宏本来是为穷人捐钱用的，并且会斥责那些什么都不给的人。可我们调用这个宏时，对“什么都不给”反而得到了赞美。那是因为第一条分支接受任意字面量，其中也包括零。并且由于宏会按顺序检查匹配器，而且总会先匹配到第一条更通用的分支，所以第二条分支永远到达不了。

清单 2.14 一个有缺陷的财富转移宏

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

#1 会打印 `"How generous"`！这并不是我们预期的结果。

在这个例子里，解决方案很简单。只要交换这些 case 的顺序即可！底层规则是这样的：按照从最具体到最不具体的顺序来编写宏规则/匹配器。这个规则同样适用于配合 `match` 的模式匹配，不过编译器并不会强制这个顺序，它只会给出一个警告。（试着把兜底分支 `_` 放到其他匹配之前。）更令人担忧的是，即使忽略这条规则，根据 Rust 的判断，这依然会形成一个有效的宏实现，甚至不会产生任何警告。所以，静态检查不足以根除这类 bug，你应该测试所有这些“分支”。
