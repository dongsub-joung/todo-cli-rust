use std::io::{self, stdin};

fn inputing() ->String{
    let mut buff= String::new();
    stdin().read_line(&mut buff).unwrap();
    let result= buff.trim_end().to_string();

    return result;
}

fn main() {
    loop{
        // io
        let memo= inputing();
        
        
    }
}

struct Memos{
    id: usize,
    memo: String,
}

impl Memos {
    pub fn set_new_memo(_id: usize, _memo: String){
       // set in db the values
        return;
    }

    pub fn get_memo()-> (usize, String){
        // get from db
        return (0,String::from(""));
    }
}
