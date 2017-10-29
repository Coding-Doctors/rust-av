use serenity::cache::CACHE;

command!(ban(_ctx, msg, args) {
    let guild = message.guild();

    if let None = guild {
        info!("No guild found for this message. Commands are not allowed in private message/groups.");
    }

    let guild = guild.unwrap().read().unwrap();
    
    let id = msg.user.id;
    
    //Delete messages in the past day.
    if let Err(why) = guild.ban(id, 1) {
        error!("Error banning user: {}", why);
    }
})
