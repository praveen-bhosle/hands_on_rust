use std::{collections::btree_map::Keys, io} ; 
use bracket_lib::prelude::* ; 

const FRAME_TIME:f32  = 60.0 ; 

#[derive(PartialEq)]
enum GameMode  { 
    Menu , 
    Playing , 
    Dead 
}

struct Player { 
   x : i8 , 
   y : i8 , 
   velocity_vertical :  i8 
} 

impl Player { 
    fn new() -> Self { 
        Self { 
            x : 10 , 
            y : 10 ,
            velocity_vertical : 0 
        }
    }
}

struct Obstacle { 
   x: i8 , 
   y: i8 , 
   velocity_horizontal : i8 
}

struct State {  
    mode : GameMode , 
    frame_time: f32 , 
    player : Player , 
    //obstacle : Obstacle 
} 

impl State { 
    fn new()  -> Self { 
        Self { 
            mode : GameMode::Menu ,  
            frame_time : 0.0  , 
            player : Player::new()  
        }
    }

    fn render_player(&self , ctx : &mut BTerm) {    
        ctx.print(self.player.x, self.player.y , "Bird"); 
    }
   
    fn make_bird_fall(&mut self) {  
        if self.player.y == 50 { 
            self.mode = GameMode::Dead ;  
            self.reset_bird_position(); 
        }
        self.player.velocity_vertical += 1 ; 
        self.player.y += self.player.velocity_vertical ;     
    } 

    fn flap(&mut self) { 
        if self.player.y >= 15 { 
         self.player.y -= 1 ;  
        }   
    }

    fn  reset_bird_position( &mut self ) {  
        self.player.y = 10 ; 
    }

}

impl GameState for State { 
    fn tick(&mut self, ctx: &mut BTerm) { 
        ctx.cls() ; 
        ctx.print(1, 1, "hi praveen"); 
        ctx.print(1, 3, ctx.frame_time_ms); 
        if self.mode == GameMode::Playing { 
        self.frame_time += ctx.frame_time_ms ;  } 
        if self.frame_time > FRAME_TIME {  
            self.make_bird_fall(); 
            self.frame_time = 0.0 ; 
        }
        match self.mode {  
            GameMode::Menu    =>     ctx.print(1,2, "this is main menu.") , 
            GameMode::Playing =>    { ctx.print(1,2,"this is playing.") ;  self.render_player(ctx);  } , 
            GameMode::Dead    =>     ctx.print(1,2,"This is dead ")
        }
        let key_pressed = ctx.key  ; 
        match key_pressed { 
            Some(key) =>    { 
                match key  {  
                    VirtualKeyCode::P => { ctx.print_centered(10, "P was pressed") ; 
                     self.mode = GameMode::Playing ; 
                    },
                    VirtualKeyCode::M => { 
                        ctx.print_centered(10, "M was pressed") ; 
                        self.mode = GameMode::Menu ; 
                    } , 
                    VirtualKeyCode::D => { 
                        ctx.print_centered(10, "D was pressed") ; 
                        self.mode = GameMode::Dead ; 
                    } ,  
                    VirtualKeyCode::Space => { 
                        self.flap(); 
                    }
                    _ => {} 
                }
            }  ,
            None  => {}  
        }
    }
}


fn  main() -> BError {   
      
    let context = BTermBuilder::simple80x50()
                                .with_title("flappy")
                                .build()? ; 
    main_loop(context , State::new())   
}