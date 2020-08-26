pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
        }
    }

    pub fn from(items: Vec<T>) -> Self {
        Self {
            items,
        }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push() {
        let mut st = Stack::new();
        st.push(21);
        assert_eq!(&21, st.peek().unwrap());
        assert_eq!(false, st.empty());
    }

    #[test]
    fn pop() {
        let mut st = Stack::new();
        st.push("pop test".to_string());
        assert_eq!("pop test".to_string(), st.pop().unwrap());
    }

    #[test]
    fn peek() {
        let mut st = Stack::new();
        st.push(6.3);
        assert_eq!(&6.3, st.peek().unwrap());
    }

    #[test]
    fn len() {
        let st: Stack<i32> = Stack::new();
        assert_eq!(0, st.len());
    }

    #[test]
    fn empty() {
        let st: Stack<i32> = Stack::new();
        assert_eq!(true, st.empty());
    }

    #[test]
    fn from() {
        let vector = vec![1, 2, 3, 4, 5];
        let st = Stack::from(vector);
        assert_eq!(vec![1, 2, 3, 4, 5], st.items);
    }
}

