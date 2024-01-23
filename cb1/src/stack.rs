use std::mem::take;
use crate::Stack;

// TODO Complete implementation
impl Stack for Vec<i32> {
    fn init() -> Self {
        let stack = Vec::new();
        return stack;
    }

    fn push_val(&mut self, i: i32) {
        self.push(i);
    }

    fn top_val(&self) -> Option<&i32> {
        return self.last();
    }

    fn pop_val(&mut self) -> Option<i32> {
        return self.pop();
    }

    fn is_empty(&self) -> bool {
        return self.is_empty();
    }
}

#[derive(Debug)]
pub enum ListStack {
    Val(i32, Option<Box<ListStack>>),
    Nil,
}

use ListStack::Nil;
use ListStack::Val;

// Complete implementation of Stack for ListStack
impl Stack for ListStack {
    fn init() -> Self {
        Nil
    }

    fn push_val(&mut self, i: i32) {
        match self {
            Val(value, other) => *self = Val(i, Some(Box::new(Val(*value, take(other))))),
            Nil => *self = Val(i, None),
        };
    }

    fn top_val(&self) -> Option<&i32> {
        match self {
            Val(value, _) => Some(value),
            Nil => None
        }
    }

    fn pop_val(&mut self) -> Option<i32> {
        match self {
            Val(value, other) => {
                let popped_value = *value;
                match other.take() {
                    None => *self = Nil,
                    Some(other) => *self = *other,
                };
                Some(popped_value)
            }
            Nil => None,
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            Nil => true,
            _ => false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::stack::ListStack;
    use crate::Stack;
    use std::fmt::Debug;

    #[test]
    fn vec_fill_and_clear() {
        println! {"Testing Vec<T>"}
        fill_and_clear_impl(Vec::init());
    }

    #[test]
    fn linked_fill_and_clear() {
        println! {"Testing ListStack"}
        fill_and_clear_impl(ListStack::init());
    }

    fn fill_and_clear_impl<T: Stack + Debug>(mut stack: T) {
        stack.push_val(1);
        assert_eq!(stack.top_val(), Some(&1));

        stack.push_val(2);
        assert_eq!(stack.top_val(), Some(&2));

        stack.push_val(-3);
        assert_eq!(stack.top_val(), Some(&-3));

        println!("{:?}", stack);

        let mut comparison = vec![1, 2, -3];
        while let Some(val) = stack.pop_val() {
            assert_eq!(comparison.pop().unwrap(), val);
        }

        assert!(stack.is_empty())
    }
}
