use bracket_lib::prelude::*;
enum GameMode {
    Menu,
    HighScore,
    Playing,
    End
}
const PLAYER_INIT_X : i32 = 5;
const PLAYER_INIT_Y : i32 = 25;
const SCREEN_WIDTH : i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;
const MIN_OBSTACLE_OFFSET : i32 =20;
#[derive(Debug)]
struct Player{
    x: i32,
    y: i32,
    velocity: f32,
}
#[derive(Debug)]
struct Obstacle{
    x: i32,
    gap_y: i32,
    size : i32,
}

struct State{
    mode: GameMode,
    player_name: String,
    frame_time: f32,
    player: Player,
    obstacles: Vec<Obstacle>,
    last_obstacle_x : i32,
    score : i32,
}

impl Obstacle {
    fn new(x:i32, score: i32) -> Self {
        let mut random=RandomNumberGenerator::new();
        println!("New Obstacle with {}",x);
        Self {
            x: x,
            gap_y: random.range(10,40),
            size: i32::max(5, 20-score)
        }
    }
    
    fn check_and_reset(&mut self,x:i32, score: i32, last_x: i32) -> bool {
        let mut reset = false;
        if self.x <= x-PLAYER_INIT_X {
            println!("Check {} {} {}",self.x,x,PLAYER_INIT_X);
            let mut random=RandomNumberGenerator::new();
            self.x = last_x+2*MIN_OBSTACLE_OFFSET;//+random.range(0,20);
            self.size = 2*i32::max(3,10-score);
            self.gap_y = random.range(10,40);
            reset = true;
        }
        reset
    }
    fn render(&mut self, ctx: &mut BTerm, player_x : i32) {
        if self.x - player_x < SCREEN_WIDTH - PLAYER_INIT_X {
            let screen_x = self.x - player_x;
            let half_size = self.size/2;

            for y in 0..self.gap_y-half_size {
                ctx.set(screen_x,y, RED, BLACK,to_cp437('/'));
            }
            for y in self.gap_y+half_size..SCREEN_HEIGHT {
                ctx.set(screen_x,y, RED, BLACK,to_cp437('/'));
            }

        }
    }

    fn hit_player(&mut self, player: &Player) -> bool {
        let x_match = self.x-5 == player.x;
        let below_gap = player.y > self.gap_y+self.size/2;
        let above_gap = player.y < self.gap_y-self.size/2;
        x_match && (below_gap || above_gap)
    }
}
impl Player{
    fn new(x: i32, y: i32) ->Self {
        Player{
            x,
            y,
            velocity: 0.0,
        }
    }

    fn reset(&mut self, x: i32,y: i32){
        self.x = x;
        self.y = y;
        self.velocity = 0.0;
    }

    fn render(&mut self, ctx:&mut BTerm) {
        ctx.set(5,self.y,YELLOW,BLACK,to_cp437('@'));
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < 5.0 {
            self.velocity+=1.0;
        }
        self.y += self.velocity as i32;
        self.x += 1;
        if self.y < 0{
              self.y = 0;
              self.velocity = 0.0;
        }
    }
    
    fn flap(&mut self){
        self.velocity = -5.0;
    }
}
impl State{
    fn new() ->Self {
        State {
            mode :  GameMode::Menu,
            player_name : String::from("Player1"),
            frame_time: 0.0,
            player : Player::new(PLAYER_INIT_X, PLAYER_INIT_Y),
            obstacles: vec![
                Obstacle::new(80,0),
                Obstacle::new(160,0),
                Obstacle::new(240,0),
            ],
            last_obstacle_x : SCREEN_WIDTH*3,
            score : 0,
        }
    }
    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.print_centered(5, "Welcome to Yet Another Flappy Bird!");
        ctx.print_centered(6, "Press to (S)tart");
        ctx.print_centered(7, "Press to view (H)igh scores");
        ctx.print_centered(8, "Press to (Q)uit");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::S => self.mode = GameMode::Playing,
                VirtualKeyCode::H => self.mode = GameMode::HighScore,
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.print_centered(5, "Game Over");
        ctx.print_centered(6,"Player Name:");
        ctx.print_centered(7,&format!("{}",self.player_name));
        ctx.print_centered(8,&format!("Score: {}",self.score));
        ctx.print_centered(9,"Press  to (E)dit and Enter to confirm");
        ctx.print_centered(10,"Press to (M)ain Menu");
        ctx.print_centered(11,"Press to (R)estart");
        ctx.print_centered(12,"Press to (Q)uit");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Return => self.save(ctx),
                VirtualKeyCode::M => self.mode = GameMode::Menu,
                VirtualKeyCode::R => self.start(ctx),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn score(&mut self, ctx: &mut BTerm) {
        //todo read file and display top 5 high scores
        ctx.print_centered(5, "High Scores:");
        ctx.print_centered(6,"1.");
        ctx.print_centered(7,"2.");
        ctx.print_centered(8,"3.");
        ctx.print_centered(9,"4.");
        ctx.print_centered(10,"5.");
        ctx.print_centered(11,"Press to (M)ain Menu");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::S => self.start(ctx),
                VirtualKeyCode::M => self.mode = GameMode::Menu,
                _ => {}
            }
        }
    }


    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time +=ctx.frame_time_ms;
        if self.frame_time >= FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
            for ob in &mut self.obstacles {
                if ob.hit_player(&self.player) {
                    println!("Hit detected! {:?} {:?}",ob,self.player);
                    self.mode = GameMode::End;
                    return
                }
                let obstacle_reset = ob.check_and_reset(self.player.x,self.score,self.last_obstacle_x);
                if obstacle_reset {
                    self.score +=1;
                    self.last_obstacle_x = ob.x;
                }
            }

        }

        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        self.player.render(ctx);
        for ob in &mut self.obstacles {
            ob.render(ctx,self.player.x);
        }
        if self.player.y >SCREEN_HEIGHT {
            self.mode = GameMode::End;
        }
        ctx.print(0,0,format!("p.x {}",self.player.x));
        ctx.print(10,0,format!("o1.x {}",self.obstacles[0].x));
        ctx.print(20,0,format!("o2.x {}",self.obstacles[1].x));
        ctx.print(30,0,format!("o3.x {}",self.obstacles[2].x));
 
        ctx.print(70,0,format!("Score {}",self.score));
    } 

    fn start(&mut self, ctx: &mut BTerm) {
        self.player.reset(PLAYER_INIT_X,PLAYER_INIT_Y);
        self.score = 0;
        for i in 0..3 {
            self.obstacles[i] = Obstacle::new(SCREEN_WIDTH*(i as i32 + 1),0);
        }
        self.mode=GameMode::Playing;
        self.last_obstacle_x = 240;
    }

    fn save(&mut self, ctx: &mut BTerm) {
    }
}
impl GameState for State{
    fn tick(&mut self,ctx: &mut BTerm){
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::HighScore => self.score(ctx),
            GameMode::Playing => self.play(ctx)
        }
    }
}
fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Yet Another Flappy Bird")
        .build()?;
    let state = State::new();
    println!("Init done!");
    main_loop(context,state)
}
