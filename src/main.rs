struct Structure{
    v : Vec<i32>, 
    //test
}

impl Structure{
    fn new()->Structure{
        Structure{
            v : vec![],
        }
    }
    fn add (&mut self, number:i32){
       let mut pos=0;
       for element in &self.v{
           if *element<number{
               pos=pos+1;
           }
           else if *element==number{
               return;
           }
       }
       self.v.insert(pos,number);
    }
    fn remove(&mut self, number:i32){
        let mut index: usize=0;
        for element in &self.v{
            if *element < number{
                index=index+1;
            }
            else if *element==number{
                break;
            }
            else {
                return;
            }
        }
        if index<self.v.len() {
            self.v.remove(index);
        }        
    }
    fn display(&self){
        // for element in &self.v{
        //     println!("{}",element);
        // }
        println!("{:?}",self.v);
    }
}
#[cfg(test)]
mod tests;
fn main() {
    let mut vector=Structure::new();
    vector.add(1);
    vector.add(2);
    vector.add(3);
    vector.add(5);
    vector.display();
    vector.remove(3);
    vector.display();
}
