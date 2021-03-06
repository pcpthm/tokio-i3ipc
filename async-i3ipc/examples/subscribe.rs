use async_std::stream::Stream;
use std::io;

use async_i3ipc::{
    event::{Event, Subscribe},
    I3,
};

// This will spawn the default scheduler which spawns 1 thread per core
// You can use #[tokio::main(basic_scheduler)] to use a single thread (you don't
// need anything more)
#[async_std::main]
async fn main() -> io::Result<()> {
    let mut i3 = I3::connect().await?;
    let resp = i3.subscribe([Subscribe::Window]).await?;

    println!("{:#?}", resp);
    let mut listener = i3.listen();
    while let Some(event) = listener.next().await {
        match event? {
            Event::Workspace(ev) => println!("workspace change event {:?}", ev),
            Event::Window(ev) => println!("window event {:?}", ev),
            Event::Output(ev) => println!("output event {:?}", ev),
            Event::Mode(ev) => println!("mode event {:?}", ev),
            Event::BarConfig(ev) => println!("bar config update {:?}", ev),
            Event::Binding(ev) => println!("binding event {:?}", ev),
            Event::Shutdown(ev) => println!("shutdown event {:?}", ev),
            Event::Tick(ev) => println!("tick event {:?}", ev),
        }
    }
    Ok(())
}
