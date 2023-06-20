import { Command } from '@newtykins/botkit';
import { catAsciiArt, kaomoji } from '../constants';

@Command.Config({
	name: 'meow',
	description: 'Meow!',
})
export default class Meow extends Command {
	registerApplicationCommands(registry: Command.Registry) {
		registry.registerChatInputCommand(builder => builder.setName(this.name).setDescription(this.description));
	}

	async chatInputRun(interaction: Command.Chat.Interaction) {
		await interaction.deferReply({ ephemeral: true });

		// choose either a kaomoji or ascii art
		const cat = Math.random() < 0.5 ? kaomoji[Math.floor(Math.random() * kaomoji.length)] : catAsciiArt[Math.floor(Math.random() * catAsciiArt.length)];

		// reply with the cat
		const reply = await interaction.editReply(`meow!\n${cat}`);

		// work out ping
		const ping = reply.createdTimestamp - interaction.createdTimestamp;

		// edit the reply to include the ping
		await interaction.editReply(`meow! (${ping}ms)\n${cat}`);
	}
}
