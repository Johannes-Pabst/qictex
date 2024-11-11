pub trait StringTools{
    fn remove_while<T>(&mut self, condition:T) ->String where T:FnMut(char,usize)->bool;
}
impl StringTools for String {
    fn remove_while<T>(&mut self, mut condition:T) ->String where T:FnMut(char,usize)->bool {
        let mut chars=self.chars();
        let mut i=0;
        let mut b=true;
        while let Some(c) = chars.next() {
            if !condition(c, i) {
                b=false;
                break;
            }
            i+=1;
        }
        if b{
            let temp=self.clone();
            *self=String::new();
            return temp;
        }
        let temp=self.split_off(self.char_indices().nth(i).unwrap().0);
        let temp2=self.clone();
        *self=temp;
        temp2.to_string()
    }
}