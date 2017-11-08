use get_config;
use serenity::model::Mentionable;
use serenity::model::RoleId;

command!(ban(_ctx, msg, args) {
    let config = get_config();

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

    let author = msg.author.id;

    let member = guild.member(id).unwrap();

    //Convert to serenity types.
    let admin_id = RoleId(config.admin_id);
    let mod_id = RoleId(config.mod_id);

    if !member.roles.contains(&admin_id) && !member.roles.contains(&mod_id) {
        //Member not authorized to ban people.
        let help_msg = format!("{}, you can't do that.", author.mention());
        msg.channel_id.say(&help_msg).unwrap();
    }

    //Delete messages in the past day. The BanOptions is a tuple implementation for (u8, str).
    if let Err(why) = guild.ban(id, options) {
        error!("Error banning user: {}", why);
    }
});

command!(kick(_ctx, msg, args) {
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
        msg.channel_id.say("You must provide at least one argument to this command for it to work.\nThe first argument must be a mention of the user you want to kick.");
        reason = String::new();
    }

    let mut log_msg: String;

    match guild.kick(id) {
        Ok(()) => {
            if reason.len() == 0 {
                log_msg = format!("User {} was kicked. No reason.", id.mention());
            } else {
                log_msg = format!("User {} was kicked for reason: {}", id.mention(), reason);
            }
        },
        Err(e) => {
            log_msg = format!("Error logging kick: {}", e);
        },
    }

    msg.channel_id.say(&log_msg).unwrap();
});
