import { error } from '@sveltejs/kit';
import { prompts_load } from '$lib/apiService.ts';
import { type PropsAllPrompts } from '$lib/models.ts';
import type { PageServerLoad } from './$types';

export const ssr = true;

export const load: PageServerLoad = async () => {
	const prompts = await prompts_load();
	console.log(`prompts ${JSON.stringify(prompts, null, 4)}`);

	if (prompts) {
		const props: PropsAllPrompts = {
			prompts: prompts
		};
		return props;
	}

	error(404, 'Not found');
};
