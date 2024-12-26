import { error } from '@sveltejs/kit';
import { load_running_models } from '$lib/apiService.ts';
import { type PropsFeAllRunningModels } from '$lib/models.ts';
import type { PageServerLoad } from './$types';

export const ssr = true;

export const load: PageServerLoad = async () => {
	const models = await load_running_models();
	if (models) {
		const props: PropsFeAllRunningModels = {
			models: models
		};
		return props;
	}

	error(404, 'Not found');
};
