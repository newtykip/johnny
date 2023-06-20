import { Listener } from "@newtykins/botkit";
import { ActivityType } from "discord.js";

@Listener.Config({
	name: 'ready',
	event: Listener.Events.ClientReady,
	once: true
})
export default class Ready extends Listener<typeof Listener.Events.ClientReady> {
	async run() {
		this.logger.info(`Logged in as ${this.client.user?.tag}!`);

		this.client.user?.setPresence({
			activities: [
				{
					name: ':3',
					type: ActivityType.Streaming,
					url: 'https://www.twitch.tv/monstercat'
				}
			]
		})
	}
}
