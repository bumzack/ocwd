import { error } from '@sveltejs/kit';
import { chats_load_by_prompt_id, prompt_by_id } from '$lib/apiService.ts';
import { type PropsChatPrompt } from '$lib/models.ts';
import type { PageServerLoad } from './$types';

export const ssr = true;

export const load: PageServerLoad = async ({ params }) => {
	const id = Number(params.id);
	const chats = await chats_load_by_prompt_id(id);
	const prompt = await prompt_by_id(id);

	console.log(`prompt ${JSON.stringify(prompt, null, 4)}`);
	console.log(`chats ${JSON.stringify(chats, null, 4)}`);

	if (chats) {
		const props: PropsChatPrompt = {
			chats: chats,
			prompt: prompt
		};
		return props;
	}

	error(404, 'Not found');
};
