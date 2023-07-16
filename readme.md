<div align="center">
<img src="johnny.png" height="250">
<h3>johnny</h3>
the least bloated all-in-one bot on discord
</div>

## compilation

### a little bit of pretext

for those who do not know much about programming, johnny is written in a compiled programming language called [rust](https://www.rust-lang.org/) which has a nifty feature called [conditional compilation](https://en.wikipedia.org/wiki/Conditional_compilation) which is used in johnny to let you select only the features YOU want for your discord bot providing a customisable experience and cutting out all of the useless features.

### how do i actually compile it?

if you do not have it already, you are going to need the Rust

#### feature List
the following features are enabled by default, you can disable these during compilation by using the `--no-default-features` flag (not recommended unless you know what you're doing!)

- **tui** - *TODO: document*

the rest of these features are not enabled by default, and you will have to enable them by appending them to your `--features` flag value.

<hr>

you must pick one of the following database drivers to use any underlined features

- **postgres** - *TODO: document*
- **mysql** - *TODO: document*
- **sqlite** - *TODO: document*

<hr>

- **<u>moderation</u>** - enables all of the below
	- **<u>autorole</u>** - *TODO: document*

- **image** - enables all of the below
	- **pride** - *TODO: document*

- **verbose** - *TODO: document*


the following flags are not recommended for most people who use this bot, but are documented for transparency.

- **johnny** - for use in [newt's server (:](https://discord.gg/ywra9UeJGh) - you most likely do not want this, but you can if you'd like i suppose!
- **db** - contains common dependencies for individual database drivers, you do not need to enable this yourself!
- **development** - please only enable this if you are in a development environment! it will only bring you potential pain if not


## development

### testing all feature combinations

in order to ensure that all feature combinations compile as expected, you can make use of [cargo-all-features](https://github.com/frewsxcv/cargo-all-features). to install this, you can run `cargo install cargo-all-features`. you can then do the test using `cargo build-all-features`. this package is installed globally, so you can use this across all of your projects - and I highly recommend that you do if they have a lot of configuration and moving parts like this one!

note: **sqlite** is the only enabled database driver across all of the build matrix - this is to prevent forced compile errors from triggering and cancelling the rest of the build matrix. this should not cause any missed errors as all of these drivers just end up outsourcing work to [sea-orm](https://github.com/SeaQL/sea-orm/), however it may be something to keep in mind.

## in the near future...

tere are some neat things we'd like to have done within the near future.

- create support hub for instances of the bot
	- add extra debugging info in the logs to help volunteers aid with this information
	- write up a guide for volunteers to properly help other individuals
- automatically compile popular feature combinations so that people do not have to deal with compilation on their own - should also cut down the amount of support tickets
- move this thing to Trello or something, goddamn!
- full-featured moderation system
- minecraft-related commands
	- scan a server given its IP and get data about it.
		- let server owners set an IP associated with their own server.
			- maybe we could have voice channels providing live-ish stats about them? this might be better done using a Minecraft plugin... potential side project? i've always wanted to get into minecraft plugin development!
	- hypixel statistics.
		- skyblock too?
	- is there a way to integrate with [DiscordSRV](https://github.com/DiscordSRV/DiscordSRV) to provide a frontend for it through this bot? discuss with the maintainer (or reverse engineer, or make my own plugin to provide this functionality!)
- remove the need for .env files - all of this configuration should be done using the TUI!
	- `--no-default-features` should not be promoted, allow [TOML](https://toml.io/en/) configuration in the same directory as the executable for people running without a TUI!
- generate a list of dependencies in this readme file with thank yous to the developers! Do the same for contributors! We love open-source!
- web dashboard (I dread this, if someone would like to help give me a shout :])
- come up for a new name for this thing or rename the **johnny** flag to avoid potential confusion

and much much more (:

<sub>licensed with the <a href="license.md">opinionated queer license v1.1</a> - tl;dr see <a href="https://oql.avris.it/">here</a> :]</sub>
