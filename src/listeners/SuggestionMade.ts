import { Listener } from "@newtykins/botkit";
import type { ThreadChannel } from "discord.js";
import { emojis } from "../constants";

@Listener.Config({
	name: 'SuggestionMade',
	event: Listener.Events.ThreadCreate
})
export default class SuggestionMade extends Listener<typeof Listener.Events.ThreadCreate> {
	async run(thread: ThreadChannel) {
		// ensure that the thread is made in the suggestions channel
		if (thread.parentId === '1120764782014890032') {
			// add upvote and downvote reactions to the post
			let post = await thread.fetchStarterMessage();
			await post?.react(emojis.upvote);
			await post?.react(emojis.downvote);
		}
	}
}
