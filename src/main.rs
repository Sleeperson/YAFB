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

struct Player{
    x: i32,
    y: i32,
    velocity: f32,
}

struct State{
    mode: GameMode,
    player_name: String,
    frame_time: f32,
    player: Player,
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
            player : Player::new(PLAYER_INIT_X, PLAYER_INIT_Y),
            frame_time: 0.0,
        }
    }
    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.print_centered(5, "Welcome to Yet Another Flappy Bird!");
        ctx.print_centered(6, "Press to (S)tart");
        ctx.print_centered(7, "Press to view (H)igh scores");
        ctx.print_centered(8, "Press to (Q)uit");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::S => self.start(ctx),
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
        ctx.print_centered(8,&format!("Score: {}",self.player.x-PLAYER_INIT_X));
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
        }

        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        self.player.render(ctx);
        if self.player.y >SCREEN_HEIGHT {
            self.mode = GameMode::End;
        }
    } 

    fn start(&mut self, ctx: &mut BTerm) {
        self.player.reset(PLAYER_INIT_X,PLAYER_INIT_Y);
        self.mode=GameMode::Playing;
    }

    fn save(&mut self, ctx: &mut BTerm) {
    }
}
impl GameState for State{
    fn tick(&mut self,ctx: &mut BTerm){
        ctx.cls();
        ctx.print(0,0,"Hello, Bracket Terminal");
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
    main_loop(context,State::new())
}
