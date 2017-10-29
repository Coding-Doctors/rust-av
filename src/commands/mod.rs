command!(ban(_ctx, msg, args) {
    let guild = msg.guild();

    if let None = guild {
        info!("No guild found for this message. Commands are not allowed in private message/groups.");
    }

    let guild = guild.unwrap();
    let guild = guild.read().unwrap();

    let id = msg.mentions[0].id;

    //Reason must come directly after the user ban mention (2nd arg).
    let reason = args.get(1).unwrap();

    let ban_options: (u8, &str) = (1, &reason);

    //Delete messages in the past day. The BanOptions is a tuple implementation for (u8, str).
    if let Err(why) = guild.ban(id, ban_options) {
        error!("Error banning user: {}", why);
    }
});
