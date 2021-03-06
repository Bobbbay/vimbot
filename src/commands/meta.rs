use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use rand::Rng;

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.say(&ctx.http, "Pong!");

    Ok(())
}

// really just an example of how shit works
#[command]
fn help(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "available commands: iwantemacs, iwantneovim, iwantvi, idontwantemacs, idontwantvi, idontwantneovim, jDots, gDots, kDots, zDots, ynix, pins, neovitality, yflakes, insultCindy");
    Ok(())
}

#[command]
fn q(ctx: &mut Context, msg: &Message) -> CommandResult {
    let mut member = ctx
        .http
        .get_member(648963701734506497, *msg.author.id.as_u64())
        .unwrap();
    let _ = member.add_role(&ctx, RoleId(648972141169213440));
    msg.channel_id.say(&ctx.http, "I GNU it--you're verified.");
    Ok(())
}

// really just an example of how shit works
#[command]
fn iwantemacs(ctx: &mut Context, msg: &Message) -> CommandResult {
    let mut member = ctx
        .http
        .get_member(648963701734506497, *msg.author.id.as_u64())
        .unwrap();
    let _ = member.add_role(&ctx, RoleId(648997712351592459));
    msg.channel_id.say(&ctx.http, "added Emacs role");
    Ok(())
}

#[command]
fn iwantneovim(ctx: &mut Context, msg: &Message) -> CommandResult {
    let mut member = ctx
        .http
        .get_member(648963701734506497, *msg.author.id.as_u64())
        .unwrap();
    let _ = member.add_role(&ctx, RoleId(648997739119771689));
    msg.channel_id.say(&ctx.http, "added NeoVim role");
    Ok(())
}

#[command]
fn iwantvi(ctx: &mut Context, msg: &Message) -> CommandResult {
    let mut member = ctx
        .http
        .get_member(648963701734506497, *msg.author.id.as_u64())
        .unwrap();
    let _ = member.add_role(&ctx, RoleId(649275285253914654));
    msg.channel_id.say(&ctx.http, "added vi role");
    Ok(())
}

#[command]
fn idontwantemacs(ctx: &mut Context, msg: &Message) -> CommandResult {
    let mut member = ctx
        .http
        .get_member(648963701734506497, *msg.author.id.as_u64())
        .unwrap();
    let _ = member.remove_role(&ctx, RoleId(648997712351592459));
    msg.channel_id.say(&ctx.http, "removed Emacs role");
    Ok(())
}

#[command]
fn idontwantneovim(ctx: &mut Context, msg: &Message) -> CommandResult {
    let mut member = ctx
        .http
        .get_member(648963701734506497, *msg.author.id.as_u64())
        .unwrap();
    let _ = member.remove_role(&ctx, RoleId(648997739119771689));
    msg.channel_id.say(&ctx.http, "removed NeoVim role");
    Ok(())
}

#[command]
fn idontwantvi(ctx: &mut Context, msg: &Message) -> CommandResult {
    let mut member = ctx
        .http
        .get_member(648963701734506497, *msg.author.id.as_u64())
        .unwrap();
    let _ = member.remove_role(&ctx.http, RoleId(649275285253914654));
    msg.channel_id.say(&ctx.http, "removed vi role");
    Ok(())
}

#[command]
fn yflakes(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "
    Flakes are sheer awesomesauce.
      1) Pin builds to commits allows for effortless extreme mixing and matching between stable and unstable and anywhere in between. Use what you know is stable, and pin the bleeding edge down to stable commits.
      2) Multiple outputs allow a monolithic repo to manage all your configurations in one place.
      3) Custom sources! Declaratively manage everything with a `flake.nix` in it and nix will know exactly how to build or override a package.
      4) Can apply arbitrary patches to package builds before building. Not sure why you would want this, but it's possible :flushed:
    ");
    Ok(())
}

#[command]
fn ynix(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "
    Why NixOS?

    1. Reproducible - produces exactly the same build every time sans compiler nondeterminism.
    2. Unless early boot is broken, boots to a consistent state.
    3. Can easily rollback to the previous system configuration state.
    4. Declarative config FOR EVERYTHING!
    5. Integration with a whole bunch of packages. Your beloved GUI rices included.
    6. Cloud integration. Brilliant Docker images, also magic like NixOps
    7. Portability - Nix runs on Linux and macOS, and takes 5 minutes to install (clone config and you're done).
    8. Free of side effects - Actually uninstalls packages and its dependencies
    9. Bleeding and stable - Can run multiple versions of the package without conflicts
    10. Implicit containerization - Lorri and direnv make switching between project-local tooling easy.
    ");
    msg.channel_id.say(&ctx.http, "
            11. Virtual Machines - VFIO is EASY to set up and perform declaratively (much more so than arch).
    12. Kernel hacking - Kernel flags and patches are easy to add to config.
    13. Use flags + source-based - each package has overrides to allow (or disallow) features to be built. Makes for a lean mean machine. Binary caching for speed.
    14. Flakes are sheer awesomesauce.
      1) Pin builds to commits allows for effortless extreme mixing and matching between stable and unstable and anywhere in between. Use what you know is stable, and pin the bleeding edge down to stable commits.
      2) Multiple outputs allow a monolithic repo to manage all your configurations in one place.
      3) Custom sources! Declaratively manage everything with a `flake.nix` in it and nix will know exactly how to build or override a package.
      4) Can apply arbitrary patches to package builds before building. Not sure why you would want this, but it's possible :flushed:

    TL;DR: as bleeding edge as Arch, as stable as Ubuntu, as flexible as Gentoo. How can you say no? <https://nixos.org/#asciinema-demo-cover>

    Want to get started? <https://nixcloud.io/tour/?id=1> and <https://scrive.github.io/nix-workshop/> are good introductions to the language.
        ");
    Ok(())
}

#[command] // TODO: Fill this content in
fn ygentoo(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "
    **Why Gentoo?**

    **1. Customizability**
    Every Gentoo installation has many, many, many steps. This makes people skeptical - but I counter that they are simply afraid to try it out. In truth, Gentoo's installation takes a bit because you go from scratch. Gentoo has actually been recommended to people as an alternative to LFS systems. Hence, it's easy to see why Gentoo is so customizable. 
    **2. No install is the same**
    This was told to me by NeddySeagoon, one of the Gentoo Forums top administrators. Not only is no single install the same, but each install is unique. This means that you're not just a common user. You're the master of your computer. This overlaps much with point #1. 
    **3. Knowledge**
    One cannot imagine the amount of knowledge you gain from installing Gentoo. Believe me. You can go from a bare, naked system, to the sexiest thing there is. You gain knowledge on a lot of Linux's quirks, ups and downs. Gentoo manually teaches you every bit there is to know about Linux systems. Which leads us to our next point...
    **4. The Handbook is amazing**
    Stop. You can't resist. The Gentoo handbook is freaking amazing. Like, seriously. The handbook goes over MULTIPLE architectures, MULTIPLE install types, and MULTIPLE variations of your files.
    **5. The community is one of the best**
    This struck me when I started using Gentoo. I expected a bunch of Elitists like Arch users. But what I received was welcoming arms. When I first installed Gentoo, I had some problems - all which were solved in a snap in the Gentoo Forums. These are the crazy active, worldwide, forums. And let's not talk about the IRC chats for Gentoo, Gentoo Ebuild Developers, and so on and so on and so on. Also, here's a tunnel to the Gentoo Discord, if you're interested: <https://discord.gg/gentoo>.

    **6. Portage is amazing*
    Portage is probably the most versatile Package Manager I've seen. You get a make.conf, can pass env variables, have amazing support for dependency management, and can understand how it works in a snap. Who wouldn't want that?
    **7. Support for your favorite compiler. **
    Yeah, that's right. You can tell Gentoo to use ccache distcc gcc, and it will do exactly that. Oh, and did I mention that's from your make.conf I mentioned earlier? Anyways...
    **8. Simplicity**
Here's a controversial opinion - Gentoo is simple. It's just simple. How do I install a package? `emerge dev-lang/rust`. That's it. That's all. Uninstall it? emerge --unmerge dev-lang/rust. Check it's dependencies before uninstalling? (or check if anything depends on it) emerge -pv --depclean dev-lang/rust. Update the world set? emerge --ask @world. I could go on, and on, and on...

    There's a ton more that can be found and written down. Feel free to submit a PR with your reasons of using Gentoo!

    Want to get started? Check out the wiki, join the community, or find your way around. Gentoo is the best experience you'll ever have.

    With love,

    Gentoo representative(s) in Vi-tality.

    ");
    Ok(())
}

#[command]
fn jDots(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http,
        "justins doots kinda suck ngl: <https://github.com/DieracDelta/flakes>. Oh yeah let's plug his lame website too: <justin.restivo.me>");
    Ok(())
}

#[command]
fn zDots(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Zacks dotfiles? awww yee darwin flakes sexy af <https://github.com/zachcoyle/nix-config>");
    Ok(())
}

#[command]
fn bDots(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http,
        "Why does everyone put stuff in here? Well, I guess I'll introduce my dots. I use Gentoo - you'll find my make.conf in there. I use neovim most of the time - my configs are there too. You'll also find `dwm`, `alacritty`, etc. Anyways - they're all yours! <https://github.com/bobbbay/dotfiles>.");
    Ok(())
}

#[command]
fn neovitality(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "we got our own vim nix distro now: <https://github.com/vi-tality/neovitality>");
    Ok(())
}

#[command]
fn insultCindy(ctx: &mut Context, msg: &Message) -> CommandResult {
    let random_value : u8 = (rand::thread_rng()).gen();
    match random_value % 8 {
      0 => msg.channel_id.say(&ctx.http, "haha"),
      1 => msg.channel_id.say(&ctx.http, "haha"),
      2 => msg.channel_id.say(&ctx.http, "haha"),
      3 => msg.channel_id.say(&ctx.http, "haha"),
      4 => msg.channel_id.say(&ctx.http, "haha"),
      5 => msg.channel_id.say(&ctx.http, "haha"),
      6 => msg.channel_id.say(&ctx.http, "haha"),
      _ => msg.channel_id.say(&ctx.http, "haha")
    };
    Ok(())
}

#[command]
fn gDots(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Gytis doots? Here ya go ya bastards: <https://github.com/gytis-ivaskevicius/nixfiles>");
    Ok(())
}

#[command]
fn kDots(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Arks doots. He got some smexy haskell: <https://github.com/pnotequalnp/nixos>");
    Ok(())
}

#[command]
fn pins(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "
    Pins aggregation:


    Doots:
    All the flakes: <https://github.com/bqv/nixrc/blob/master/flake.nix>
    stdenv, wine: <https://github.com/Acizza/nixos-config/blob/desktop/overlays/overlay.nix>
    not sure why this is pinned: <https://github.com/badly-drawn-wizards/dotfiles>
    not sure why this is pinned: <https://github.com/bbigras/nix-config>

    Utils:
    <https://github.com/wucke13/nix-autobahn>

    Funky projects:
    <https://github.com/svanderburg/nijs>
    <https://github.com/t184256/nix-on-droid>
    <https://www.haskell.org/ghcup/> ( I have no idea what it is)
    <https://github.com/digitallyinduced/ihp>
    <https://github.com/neovim/neovim/blob/master/contrib/flake.nix>
    <https://nixos.wiki/wiki/NixOS_on_ARM>

    Interesting articles:
    <https://nixos.mayflower.consulting/blog/2020/06/17/windows-vm-performance/>
    <https://christine.website/blog/nixos-encrypted-secrets-2021-01-20>

    Status of XYZ:
    Firefox UI update: <https://wiki.mozilla.org/Firefox/Proton>
    *BSD support: <https://github.com/NixOS/nix/issues/3280>
    Pipewire: <https://github.com/NixOS/nixpkgs/issues/102547>
    Pentesting: <https://github.com/NixOS/nixpkgs/issues/81418>
    Java: <https://github.com/NixOS/nixpkgs/issues/106716>
        ");
    Ok(())
}
