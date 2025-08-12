use bracket_lib::prelude::* ; 

const FRAME_TIME:f32  = 300.0 ; 

#[derive(PartialEq)]
enum GameMode  { 
    Menu , 
    Playing , 
    Dead 
}

struct Player { 
   x : i8 , 
   y : i8 
} 

impl Player { 
    fn new() -> Self { 
        Self { 
            x : 10 , 
            y : 10   
        }
    }
}

struct Obstacle { 
   x: i8 , 
   y: i8 , 
}

impl Obstacle {    
    fn new() -> Self  {
        let y = fastrand::i8(10..49) ; 
        Self { x: 79  , y } 
    } 
}


struct State {  
    mode : GameMode , 
    frame_time: f32 , 
    player : Player , 
    obstacle : Obstacle , 
    frame_time2 : f32 
} 

impl State { 
    fn new()  -> Self { 
        Self { 
            mode : GameMode::Menu ,  
            frame_time : 0.0  , 
            frame_time2 : 0.0  , 
            player : Player::new()  , 
            obstacle : Obstacle::new()  
        }
    }

    fn render_player(&self , ctx : &mut BTerm) {    
        ctx.print(self.player.x, self.player.y , "Bird"); 
    }
   
    fn make_bird_fall(&mut self) {  
        if self.player.y >= 50  { 
            self.mode = GameMode::Dead ;  
            self.reset_bird_position(); 
            return ; 
        }

        self.player.y += 2 ;     
    } 

    fn flap(&mut self) { 
        if self.player.y >= 10 { 
         self.player.y -= 1 ;  
        }   
    }

    fn  reset_bird_position( &mut self ) {  
        self.player.y = 10 ; 
    }

    fn  render_obstacle(&self , ctx : &mut BTerm) {  
        ctx.print(self.obstacle.x, self.obstacle.y, "Obstacle"); 
    } 
    
    fn move_obstacle(&mut self ) { 
        if self.obstacle.x == 0  {  self.reset_obstacle();  return ;   }
        self.obstacle.x -= 1  ; 
    }

    fn collission(&mut self) -> bool  {  
         if self.obstacle.y == self.player.y  { 
            let diff = self.obstacle.x-self.player.x  ; 
            println!("diff is {}" , diff ) ; 
            if diff<4  && diff > -8   { 
                return true ; 
            }
         }
         return  false ; 
    }

    fn reset_obstacle(&mut self) { 
        let y  = fastrand::i8(10..49) ; 
        self.obstacle.x = 79 ; 
        self.obstacle.y = y ; 
    }

}

impl GameState for State { 
    fn tick(&mut self, ctx: &mut BTerm) { 
        println!("{}",self.player.y) ; 
        ctx.cls() ; 
        ctx.print(1, 1, "hi praveen"); 
        ctx.print(1, 3, ctx.frame_time_ms); 
        if self.mode == GameMode::Playing { 
        self.frame_time += ctx.frame_time_ms ;
        self.frame_time2 += ctx.frame_time_ms ;  } 
        if self.frame_time2 > FRAME_TIME / 2.0  {  
            self.move_obstacle(); 
            self.frame_time2 = 0.0 ; 
        }
        if self.frame_time > FRAME_TIME {  
            self.make_bird_fall();  
            self.frame_time = 0.0 ; 
        }
        match self.mode {  
            GameMode::Menu    =>     ctx.print(1,2, "this is main menu.") , 
            GameMode::Playing =>    { 
                ctx.print(1,2,"this is playing.") ;  
                self.render_player(ctx);   
                self.render_obstacle(ctx);  
                let x  = self.collission() ; 
                println!("collided {}" , x ) ;  
                if self.collission() { self.mode = GameMode::Dead ;  }                 
            } , 
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