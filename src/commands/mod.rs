command!(ban(_ctx, msg, args) {
    let guild = msg.guild();

    if let None = guild {
        info!("No guild found for this message. Commands are not allowed in private message/groups.");
    }

    let guild = guild.unwrap();
    let guild = guild.read().unwrap();
    
    //First mention in the message.
    let id = msg.mentions[0].id;
    
    let mut reason: String;
    
    //If there were arguments provided and we can split them up.
    if let Some((_, remainder)) = args.split_first() {
        let joined = remainder.join(" ");
        reason = joined;
    } else {
        msg.channel_id.say("You must provide at least one argument to this command for it to work.\nThe first argument must be a mention of the user you want to ban");
        reason = String::new();
    }

    let options: (u8, &str) = (1, &reason);

    //Delete messages in the past day. The BanOptions is a tuple implementation for (u8, str).
    if let Err(why) = guild.ban(id, options) {
        error!("Error banning user: {}", why);
    }
});
