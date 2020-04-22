#![allow(dead_code)]
mod app;
mod board;
mod geometry;
mod logic;
mod pieces;


fn main() {
    let (mut ctx, mut event_loop) = ggez::ContextBuilder::new("my_game", "Cool Game Author")
    .build()
    .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = app::App::new(&mut ctx);
    
    // Run!
    if let Err(e) = ggez::event::run(&mut ctx, &mut event_loop, &mut my_game) {
        println!("Error occured: {}", e);
    }
    
}
