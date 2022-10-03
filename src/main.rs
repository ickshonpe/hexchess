#![allow(dead_code)]
mod app;
mod board;
mod geometry;
mod logic;
mod pieces;


fn main() {
    let (mut ctx, event_loop) = ggez::ContextBuilder::new("my_game", "Cool Game Author")
    .build()
    .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = app::App::new(&mut ctx);
    
    // Run!
    ggez::event::run(ctx, event_loop, my_game);
    
}
