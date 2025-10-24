use std::marker::PhantomData;

struct Counter {
    count: usize,
}

struct CounterIter {
    index: usize,
    to: usize,
}

// impl Counter {
//     fn iter(&self) -> CounterIter {
//         CounterIter {
//             index: 0,
//             to: self.count,
//         }
//     }
// }

impl IntoIterator for &Counter {
    type Item = usize;
    type IntoIter = CounterIter;

    fn into_iter(self) -> Self::IntoIter {
        CounterIter {
            index: 0,
            to: self.count,
        }
    }
}

impl IntoIterator for &mut Counter {
    type Item = usize;
    type IntoIter = CounterIter;

    fn into_iter(self) -> Self::IntoIter {
        CounterIter {
            index: 0,
            to: self.count,
        }
    }
}

impl IntoIterator for Counter {
    type Item = usize;
    type IntoIter = CounterIter;

    fn into_iter(self) -> Self::IntoIter {
        CounterIter {
            index: 0,
            to: self.count,
        }
    }
}

impl Iterator for CounterIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.to {
            let i = self.index;
            self.index += 1;
            Some(i)
        } else {
            None
        }
    }
}

// 原始结构体
#[derive(Debug)]
struct Rect {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
}

// ----------------------------------------------------
// 1. 自定义迭代器结构体
// ----------------------------------------------------

/// Rect 字段的可变引用迭代器。
/// 'a 是 Rect 结构体的生命周期。
pub struct RectMutIterator<'a> {
    // 指向 Rect 内部当前 f32 元素的原始可变指针。
    ptr: *mut f32,
    // 剩余的元素数量。
    len: usize,
    // 生命周期标记：将迭代器的生命周期 'a 绑定到 Rect 的 &mut 借用上。
    _marker: PhantomData<&'a mut f32>,
}

// ----------------------------------------------------
// 2. Rect 上的 `iter_mut` 方法
// ----------------------------------------------------

impl Rect {
    /// 创建一个返回 Rect 内部字段可变引用的迭代器。
    pub fn iter_mut(&mut self) -> RectMutIterator {
        // 关键的 unsafe 块：将 Rect 结构体视为一个 f32 数组。
        unsafe {
            // 将 &mut Rect 转换为 *mut f32 (指向 a 字段的指针)。
            // 这样做是安全的，因为所有字段都是 f32，且它们在内存中是连续的。
            let ptr_f32 = self as *mut Rect as *mut f32;

            RectMutIterator {
                ptr: ptr_f32,
                len: 4, // 字段总数
                _marker: PhantomData,
            }
        }
    }
}

impl<'a> IntoIterator for &'a mut Rect {
    type Item = &'a mut f32;
    type IntoIter = RectMutIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

// ----------------------------------------------------
// 3. 迭代器的实现
// ----------------------------------------------------

impl<'a> Iterator for RectMutIterator<'a> {
    type Item = &'a mut f32; // 返回 f32 的可变引用

    fn next(&mut self) -> Option<Self::Item> {
        // 1. 检查是否迭代完毕
        if self.len == 0 {
            return None;
        }

        // 2. 减少剩余元素计数
        self.len -= 1;

        // 3. 核心 unsafe 逻辑
        unsafe {
            // 获取当前元素的指针
            let current_ptr = self.ptr;

            // 将内部指针移动到下一个元素
            // offset(1) 是一个 safe 抽象，但它内部执行的是 unsafe 指针算术
            // 它确保 `self.ptr` 现在指向下一个 f32 (即下一个字段)
            self.ptr = self.ptr.offset(1);

            // 4. 将当前的原始指针转换回安全的 &mut f32 引用
            // 这是安全的，因为我们通过 len 和 ptr 的管理，
            // 确保了当前元素是独占的，且生命周期被正确绑定。
            Some(&mut *current_ptr)
        }
    }
}

// ----------------------------------------------------
// 4. 使用示例
// ----------------------------------------------------

fn main() {
    let mut rect = Rect {
        a: 1.0,
        b: 2.0,
        c: 3.0,
        d: 4.0,
    };

    println!("Original Rect: a={}, b={}, c={}, d={}", rect.a, rect.b, rect.c, rect.d);

    // 迭代并修改值：完全像使用 Vec 或数组一样安全地使用
    for (i, val) in rect.iter_mut().enumerate() {
        // val 是 &mut f32
        *val += 10.0 * (i as f32);
    }

    // 验证修改
    println!("Modified Rect: a={}, b={}, c={}, d={}", rect.a, rect.b, rect.c, rect.d);
    // 输出: Modified Rect: a=1.0, b=12.0, c=23.0, d=34.0
}
//
// struct RectIterMut<'a> {
//     // 使用数组而不是 Vec，避免堆分配
//     fields: &'a mut Rect,
//     index: usize,
// }
//
// impl Rect {
//     fn iter_mut(&mut self) -> RectIterMut {
//         // 这里可以安全地同时借用不同的字段
//         RectIterMut {
//             fields: self,
//             index: 0,
//         }
//     }
// }
//
// impl<'a> Iterator for &'a mut Rect {
//     type Item = &'a mut f32;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         Some(&mut self.a)
//     }
// }
//
// impl<'a> Iterator for RectIterMut<'a> {
//     type Item = &'a mut f32;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         match self.index {
//             0 => {
//                 self.index += 1;
//                 Some(&mut self.fields.a)
//             }
//             1 => {
//                 self.index += 1;
//                 Some(&mut self.fields.b)
//             }
//             2 => {
//                 self.index += 1;
//                 Some(&mut self.fields.c)
//             }
//             3 => {
//                 self.index += 1;
//                 Some(&mut self.fields.d)
//             }
//             _ => None,
//         }
//     }
// }
//
// use std::iter::FusedIterator;
//
// struct Rect {
//     a: f32,
//     b: f32,
//     c: f32,
//     d: f32,
// }
//
// struct RectIterMut<'a> {
//     rect: &'a mut Rect,
//     next: usize,
// }
//
// impl<'a> Iterator for RectIterMut<'a> {
//     type Item = &'a mut f32;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         match self.next {
//             0 => {
//                 self.next = 1;
//                 Some(&mut self.rect.a)
//             }
//             1 => {
//                 self.next = 2;
//                 Some(&mut self.rect.b)
//             }
//             2 => {
//                 self.next = 3;
//                 Some(&mut self.rect.c)
//             }
//             3 => {
//                 self.next = 4;
//                 Some(&mut self.rect.d)
//             }
//             _ => None,
//         }
//     }
// }
//
// impl<'a> FusedIterator for RectIterMut<'a> {}
//
// impl Rect {
//     fn iter_mut(&mut self) -> RectIterMut {
//         RectIterMut {
//             rect: self,
//             next: 0,
//         }
//     }
// }

#[cfg(test)]
mod test {
    use crate::learn_iter::{Counter, Rect};

    #[test]
    fn test_rect() {
        let mut rec = Rect {
            a: 1.0,
            b: 2.0,
            c: 3.0,
            d: 4.0,
        };

        for i in &mut rec {
            *i += 10.0
        }

        println!("{:?}", rec);
    }

    #[test]
    fn test_count() {
        let mut count = Counter { count: 8 };

        for i in &count {
            println!("{}", i);
        }

        println!("————————————————————");

        for i in &count {
            println!("{}", i);
        }

        println!("————————————————————");

        (&count).into_iter().enumerate().for_each(|(i, v)| {
            println!("{}: {}", i, v);
        });

        println!("————————————————————");

        for i in &mut count {}

        println!("————————————————————");

        // for i in count {
        //     println!("{}", i);
        // }

        for i in &count {
            println!("{}", i);
        }

        // for i in count {
        //     println!("{}", i);
        // }

        // let mut rect = Rect {
        //     a: 1.0,
        //     b: 2.0,
        //     c: 3.0,
        //     d: 4.0,
        // };
        //
        // let r = &mut rect;
        // let mut a = &mut rect.a;
        // let mut b = &mut rect.b;
        //
        // *a = 200f32;
        // *b = 300f32;
        // r.d = 100f32;
    }

    #[test]
    fn test_vec_iter() {
        let mut a = vec![1, 2, 3];

        for i in &a {
            println!("{}", i);
        }

        a.iter_mut().for_each(|i| {
                *i = 100;
        });
    }
}
