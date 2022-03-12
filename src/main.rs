struct Structure{
    v : Vec<i32>,
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
                index+=1;
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
    fn prime_numbers(&self)-> Vec<i32>{
        let mut v_prime=self.v.clone();
        let mut index=0;
        let mut first=1;
        while  index<v_prime.len() {
            if v_prime[index]%2==0&&v_prime[index]!=2{
                v_prime.remove(index);
                if index==0{
                    first=0;
                }
                else{
                    index-=1;
                }
            }
            else if v_prime[index]==1{
                v_prime.remove(index);
                if index==0{
                    first=0;
                }
                else{
                    index-=1;
                }
            }
            else if v_prime[index]<=0{
                v_prime.remove(index);
                if index==0{
                    first=0;
                }
                else{
                    index-=1;
                }
            }
            else{
                let root=((v_prime[index] as f64).sqrt()+1.0) as i32;
                for d in (3..root).step_by(2){
                    if v_prime[index]%d==0&&v_prime[index]!=d{
                        v_prime.remove(index);
                        if index==0{
                            first=0;
                        }
                        else{
                            index-=1;
                        }
                    }
                }
            }
            if first==1{
                index+=1;
            }
            first=1;
        }
        return v_prime;
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
    vector.add(4);
    vector.add(21);
    vector.add(9);
    vector.add(11);
    vector.display();
    vector.display();
    println!("{}",-2%2);
    println!("{:?}",vector.prime_numbers());
}
