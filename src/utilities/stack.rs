pub struct Stack<T> {
    structure: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            structure: Vec::new(),
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.structure.pop()
    }

    pub fn push(&mut self, element: T) {
        self.structure.push(element);
    }

    pub fn is_empty(&self) -> bool {
        self.structure.is_empty()
    }
}
