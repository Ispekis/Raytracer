use tetra::graphics::{self, Color, Texture};
// use tetra::math::num_traits::Saturating;
use tetra::{Context, ContextBuilder, State, window};
use tetra::math::Vec2;
#[derive(Copy, Clone)]
pub struct GameState;

pub struct render {
    pub game_state:GameState,
    pub ctx:Context,
    // pub pixel: graphics::Texture,
}

impl render {
    // pub fn new(&self, name:&str, width:i32, height:i32) -> tetra::Result  {
    //     let builder = new_ctx("Raytracer", width as i32, height as i32);
    //     let game_state = GameState::new();
    //     let ctx = self.create_context(builder, game_state);
    //     let mut color= vec![0; 32*32*4];
    //     color[0] = 255;
    //     color[1] = 255;
    //     color[2] = 255;
    //     color[3] = 0;
    //     let bytes = [255,0,0,255];
    //     // let pixel = Texture::from_rgba(&mut ctx, 1, 1, color);

    //     // render {game_state, ctx}
    // }
    pub fn create_context(&self, builder:ContextBuilder, game_state:GameState) -> tetra::Result {
        builder.build()?.run(|_| Ok(game_state))
    }
    pub fn update_screen(&self, x:u32, y:u32) {

        // let ctx1:Context = self.ctx;
        // self.pixel.draw(&mut self.ctx, Vec2::new(16.0,16.0));
    }

}

impl GameState {
    pub fn new() -> GameState {
        GameState
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::BLACK);
        // graphics::DrawParams::new();
        Ok(())
    }
}

pub fn new_ctx(name:&str, height:i32, width:i32) -> tetra::ContextBuilder {
    ContextBuilder::new(name, height, width)
}