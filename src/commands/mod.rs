command!(ban(_ctx, msg, args) {
    let guild = msg.guild();

    if let None = guild {
        info!("No guild found for this message. Commands are not allowed in private message/groups.");
    }

    let guild = guild.unwrap();
    let guild = guild.read().unwrap();

    let id = msg.mentions[0].id;

    //Delete messages in the past day.
    if let Err(why) = guild.ban(id, 1) {
        error!("Error banning user: {}", why);
    }
});
