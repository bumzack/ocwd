import { error } from '@sveltejs/kit';
import { load_models } from '$lib/apiService.ts';
import  { type PropsFeAllModels } from '$lib/models.ts';
import type { PageServerLoad } from './$types';

export const ssr = true;

export const load: PageServerLoad = async () => {
	const models = await load_models();
	if (models) {
		const props: PropsFeAllModels = {
			models: models
		};
		return props;
	}

	error(404, 'Not found');
};
