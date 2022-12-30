pub struct Node<T> {
    pub value: T,
    pub children: Vec<Node<T>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Node<T> {
        return Node{
            value,
            children:vec![],
        }
    }

    pub fn get_value(&self) -> &T {
        return &self.value;
    }

    //pub fn get_last_child(&self) -> &mut Node<T> {
    //    let len = self.children.len();
    //    return &mut self.children[len-1];
    //}

    //pub fn new_child(&mut self,data: T) {
    //    self.children.push(Rc::new(RefCell::new(Self::new(data))));
    //}

    pub fn add_child(&mut self,value: T) {
        self.children.push(Self::new(value));
    }

}