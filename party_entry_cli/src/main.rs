use std::io::stdin ; 


#[derive(Debug)]
enum VisitorAction { 
  Accept , 
  AcceptWithNote{ note : String } , 
  Refuse , 
  Probation 
} 

#[derive(Debug)]
struct Visitor {  
    name : String , 
    action : VisitorAction , 
    age : u8 
} 

impl Visitor { 
  fn new( name :String , action : VisitorAction , age : u8  ) -> Self  { 
      Visitor { 
        name , 
        action ,
        age 
      }
  } 

  fn greet(&self) { 
     match &self.action { 
        VisitorAction::Accept => println!("Hi {} ,welocome to the party."  , self.name ) ,
        VisitorAction::AcceptWithNote { note  } => { 
             println!("Hi {} , welcome to the party " , self.name)   ; 
             println!("{}",note) ;  
             if self.age < 18 { 
                println!("Dont give drinks."); 
             }
            } , 
        VisitorAction::Probation => println!("Hey u r a probationary member") , 
        VisitorAction::Refuse => println!("U r not allowed")
    }   
  } 
} 

fn  read_string(error_str : &str ) -> String { 
    let mut  strr = String::from("") ; 
    stdin()
           .read_line(&mut strr)
           .expect(&error_str) ; 
    strr
        .trim()
        .to_string()
}

fn prompt_new_person(name:String) -> Visitor  { 
    let mut age = 0  ; 
    loop { 
        println!("Enter ur age: ") ;  
        let age_str  =  read_string("error reading the age.") ; 
        match age_str.parse::<u8>() { 
            Ok(curr_age) => { age = curr_age ;   break; } ,
            _ =>  println!("Enter a valid age range 1-255")
        }
        if age != 0 {  
            break ; 
        }
    } 
    Visitor::new(name, VisitorAction::Accept, age) 
}

fn print_vec(list: &Vec<Visitor> )  {
    for v in list.iter() { 
        println!("{:?}"  , v) ;  
    }
}

fn  check_name(list : &mut Vec<Visitor> , name : String  )  { 
    println!("{}" , name) ;   
    let visitor_option  = list.iter().find( |visitor|  visitor.name == name   )  ; 
    match visitor_option  { 
        Some(visitor) => Visitor::greet(visitor) ,  
        None  =>  { println!("You are not on the list.");   
                    add_visitor(list, prompt_new_person(name)); 
                  }
    }
}

fn  add_visitor(list: &mut Vec<Visitor>, visitor : Visitor ) { 
    list.push(visitor); 
} 

fn  main() {    
    let mut  visitor_list  = vec![ Visitor::new(String::from("Praveen"), VisitorAction::Accept , 18) , Visitor::new( String::from("Praveen2"), VisitorAction::AcceptWithNote { note: String::from("Dont give beer.")  } , 44 )] ; 
    loop {
        let  name = read_string("error reading name.") ;  
        if name.is_empty() { break ; }
        check_name(&mut visitor_list, name);
     } 
} 