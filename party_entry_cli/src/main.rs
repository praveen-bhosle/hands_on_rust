use std::io::stdin ;  

#[derive(Debug)] 
enum VisitorAction { 
    Accept,
    AcceptWithNote { note :String }, 
    Refuse, 
    Probation 
}

#[derive(Debug)]
struct Visitor { 
    name: String , 
    action : VisitorAction , 
    age : i8 
}

impl Visitor { 
    fn new(name: &str , action :VisitorAction , age : i8) -> Self { 
        Self { 
            name: name.to_lowercase() , 
            age , 
            action
         }
    }
    fn greet_visitor(&self) { 
        match &self.action {  
            VisitorAction::Accept => println!("Welcome to the party , {} " , self.name) , 
            VisitorAction::AcceptWithNote { note  } =>  { 
                println!("Welcome to the party, {} ", self.name )   ; 
                println!("{}" ,note ) ; 
                if self.age < 21 { 
                    println!("Dont serve alcohol to {}" , self.name ) ; 
                }
             } ,
            VisitorAction::Probation =>  println!("{} is now a probationary member" , self.name ) ,  
            VisitorAction::Refuse => println!("Do not allow {} in!" , self.name ) 
        } 
    }
} 



fn ur_name() -> String  {  
    let mut name = String::new() ; 
    stdin()
          .read_line(&mut name  )
          .expect("failed to read line.") ; 
    name
        .trim()
        .to_lowercase() 
}

fn  check_visitor(visitor_list: &mut Vec<Visitor> , name:&String )   { 
    let known_visitor =  visitor_list
        .iter()
        .find(|visitor| &visitor.name == name)  ; 
    
    match known_visitor { 
        Some(visitor) => visitor.greet_visitor(), 
        None => { add_vistor(  visitor_list, name); println!("You are not on the visitor list.") } 
    }     
}

fn add_vistor(visitor_list: &mut  Vec<Visitor> , name:&String) {  
    visitor_list.push(Visitor::new( name, VisitorAction::Refuse , 24 ))  ; 
}  


fn main() { 
    let mut  visitor_list = vec![ 
        Visitor::new("praveen",    VisitorAction::Accept , 19  ) , 
        Visitor::new("praveen2", VisitorAction::AcceptWithNote { note: String::from("I dont like beer")  }  ,  3 )   ] ; 
    loop { 
    let  name = ur_name() ;   
    if  name.is_empty()  { break ; } 
    check_visitor(&mut visitor_list, &name);    
    }
}