import { error } from '@sveltejs/kit';
import { chats_load_by_prompt_id, prompt_by_id } from '$lib/apiService.ts';
import { type Props } from '$lib/models.ts';

export const ssr = false;

export const load: PageLoad = async ({ params }) => {
	console.log(`loading chats for promptId ${params.id}`);
	const id = Number(params.id);
	const chats = await chats_load_by_prompt_id(id);
	const prompt = await prompt_by_id(id);

	console.log(`page.ts   chats ${JSON.stringify(chats)}`);
	console.log(`page.ts   prompt ${JSON.stringify(prompt)}`);


	if (chats) {
		let props: Props = {
			chats: chats,
			prompt: prompt
		};
		return props;
	}

	error(404, 'Not found');
};