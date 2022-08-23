// tokio::spawn(async move {
//     let elements = {
//         let data_read = ctx_clone.data.read().await;
//         data_read.get::<RillRateComponents>().unwrap().clone()
//     };

//     loop {
//         // Get the REST GET latency by counting how long it takes to do a GET request.
//         let get_latency = {
//             let now = Instant::now();
//             // `let _` to suppress any errors. If they are a timeout, that will  be
//             // reflected in the plotted graph.
//             let _ = reqwest::get("https://discordapp.com/api/v6/gateway").await;
//             now.elapsed().as_millis() as f64
//         };

//         // POST Request is feature gated because discord doesn't like bots doing repeated
//         // tasks in short time periods, as they are considered API abuse; this is specially
//         // true on bigger bots. If you still wanna see this function though, compile the
//         // code adding `--features post-ping` to the command.
//         //
//         // Get the REST POST latency by posting a message to #testing.
//         //
//         // If you don't want to spam, use the DM channel of some random bot, or use some
//         // other kind of POST request such as reacting to a message, or creating an invite.
//         // Be aware that if the http request fails, the latency returned may be incorrect.
//         #[cfg(feature = "post-ping")]
//         let post_latency = {
//             let now = Instant::now();
//             let _ = ChannelId(381926291785383946).say(&ctx_clone, "Latency Test").await;
//             now.elapsed().as_millis() as f64
//         };

//         // Get the Gateway Heartbeat latency.
//         // See example 5 for more information about the ShardManager latency.
//         let ws_latency = {
//             let data_read = ctx.data.read().await;
//             let shard_manager = data_read.get::<ShardManagerContainer>().unwrap();

//             let manager = shard_manager.lock().await;
//             let runners = manager.runners.lock().await;

//             let runner = runners.get(&ShardId(ctx.shard_id)).unwrap();

//             if let Some(duration) = runner.latency {
//                 duration.as_millis() as f64
//             } else {
//                 f64::NAN // effectively 0.0ms, it won't display on the graph.
//             }
//         };