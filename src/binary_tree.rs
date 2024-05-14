pub trait Number:std::cmp::PartialOrd +std::marker::Copy{}
impl<T: Number> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node {
            value: value,
            left: None,
            right: None,
        }
    }

    pub fn add(&mut self, value: T) {
        // if value less than stored val
        //Go left. If leaf, add node

        //else go right

        if self.value > value {
            if self.left.is_none() {
                self.left = Some(Box::new(Node::new(value)));
            } else {
                self.left.as_mut().unwrap().add(value);
            }
        } else {
            if self.right.is_none() {
                self.right = Some(Box::new(Node::new(value)));
            } else {
                self.right.as_mut().unwrap().add(value);
            }
        }
    }

    pub fn delete(&mut self, del_val:T) ->Result<(),()> {
        if del_val < self.value{
            let res = self.left.as_mut().unwrap().delete(del_val);
            match res{
                Ok(())=>Ok({

                }),
                Err(())=>{
                    self.left= None;
                    return  Ok(())
                }
            }
        }
        else if del_val>self.value{
            let res = self.right.as_mut().unwrap().delete(del_val);
            match res{
                Ok(())=>Ok({

                }),
                Err(())=>{
                    self.right= None;
                    return  Ok(())
                }
            }
        }
        else{
            if self.left.is_none() && self.right.is_some(){
                self.value = self.right.as_ref().unwrap().value;
                self.right = None;
                return Ok(())
            }
            else if self.right.is_none() && self.left.is_some(){
                self.value = self.left.as_ref().unwrap().value;
                self.left = None;
                return Ok(());
            }
            else if self.right.is_none() && self.left.is_none(){
                return Err(())
            }

            self.value = self.right.as_ref().unwrap().min_value();
            self.right.as_mut().unwrap().delete(self.value);
            return Ok(())
        }
    }

    fn min_value(&self) -> T{
        let mut minV = self.value;
        if self.left.is_some(){
             minV = self.left.as_ref().unwrap().min_value();
        }
        return minV
    }

    pub fn search(&self, search_val: T) -> bool {
        if search_val == self.value {
            return true;
        }
        if search_val > self.value {
            if self.right.is_none() {
                return false;
            }
            return self.right.as_ref().unwrap().search(search_val);
        }
        if search_val < self.value {
            if self.left.is_none() {
                return false;
            }
            return self.left.as_ref().unwrap().search(search_val);
        }
        return false;
    }
}

impl Number for i32{}

#[derive(Debug)]
pub struct Node<T: Number> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}
