前面提到过，如果想在声明式宏里精确处理参数个数之类的问题，并不容易做到。关于这件事的具体写法，可以参考 2.2.5 节中的示例。

### 2.2.4 DSL

声明式宏也非常适合用来构建 DSL。正如本章开头所说，DSL 的价值在于把开发者从领域专家那里获得的领域知识，直接封装进应用程序。这个思路和领域驱动设计中的“通用语言”很接近：当专家和开发者使用同一套词汇和概念交流时，沟通成本会更低，代码表达也会更准确。

DSL 的目标，是为某个特定领域提供一门专用语言，同时把那些无关的复杂性隐藏起来。这里的“复杂性”可能包括额外校验，也可能是一些实现层面的细枝末节。有人认为，DSL 甚至能让非程序员也看懂代码库。像 Cucumber 这样的测试框架（https://github.com/cucumber-rs/cucumber），背后的一个核心想法就是：用领域专家也能读懂的语言来写测试。进一步说，领域专家甚至可能亲自补充测试。在 Rust 生态里，这种 mini-DSL 非常常见，而宏正是构建它们的常用工具。标准库里的 `println!` 和 `format!` 就是两个很典型的例子，它们通过花括号语法来决定如何格式化并输出变量。

下面来看一个具体例子：我们实现一个用于处理账户转账的 DSL。第一步，先定义一个 `Account` 结构体，用来保存账户余额。它还提供了两个方法，分别用于增加和减少账户金额。这里我们只允许正数金额，因此使用 `u32`；至于账户变成负数的情况，暂时不在这个示例的讨论范围内。`Account` 同时派生了 `Debug`，这通常都是个不错的选择，而在这里，我们只是为了方便打印结果。

清单 2.12 展示了 `Account` 的定义：

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

#1 `Add` 和 `Sub` trait 让我们可以在下面的 `Account` 实现里，直接对 `u32` 调用 `add` 和 `sub` 方法。

接着看清单 2.13 中的 `exchange` 宏。这个宏向使用者暴露出一个小型 DSL。它最大的好处是，可以用外行人也比较容易理解的自然语言来描述操作；如果写出的命令不符合宏的规则，错误还会在编译期直接暴露出来。除此之外，当我们在两个账户之间转账时（第三条规则），交易内部的复杂处理也被隐藏在 DSL 背后了。

清单 2.13 这个宏为“转账”提供了一个 mini-DSL：

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

#1 这里必须在 `$giver.subtract($amount)` 后面补一个分号，因为这条展开结果里包含两条语句；如果不写，编译器会报错，提示这里缺少 `;`。

#2 这个调用形式使用了很接近自然语言的写法来表达交易。

#3 程序会打印出 `"Poor: Account { money: 50 }, rich: Account { money: 160 }"`。

当然，DSL 的能力完全不止于此。你可以继续往里扩展货币类型，并自动完成不同币种之间的换算；也可以为透支设计专门规则。不过与此同时，也必须认真测试，确保不同的宏分支不会互相“碰撞”。所谓碰撞，就是你原本以为输入会命中 matcher X，结果却提前落到了 matcher Y。

下面这个例子就很好地说明了这个问题。这个宏本意是“给穷人钱”，而且如果有人一分钱都不给，它还应该出言讽刺。但实际调用时，我们明明给的是零，却反而得到了夸奖。原因就在于第一条规则可以匹配任意字面量，而零当然也属于字面量；又因为宏会按声明顺序逐条匹配，所以更宽泛的第一条规则先命中了，后面更具体的第二条规则永远不会执行。

清单 2.14 展示了这个存在缺陷的财富转移宏：

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

#1 实际输出是 `"How generous"`，这显然不是我们想要的结果。

这个问题的解决办法非常直接：把两个分支的顺序调换一下即可。这里的底层原则是，宏规则或 matcher 应该按照“从最具体到最宽泛”的顺序来书写。这个原则在 `match` 模式匹配里也同样成立，只不过编译器在那种场景下通常只会给出警告，而不会强制阻止你这样写。（你可以试试把兜底分支 `_` 放到其他分支前面。）

更值得警惕的是，在宏这里，即便你违反了这个顺序规则，Rust 依然可能把它视为合法实现，而且连警告都不给你。因此，单靠静态检查并不足以发现这类问题。要想把这些 bug 真正揪出来，你必须把所有“分支”都测到。
