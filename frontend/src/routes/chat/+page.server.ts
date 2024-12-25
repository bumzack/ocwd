import { error } from '@sveltejs/kit';
import { load_models } from '$lib/apiService.ts';
import { type PropsAllModels } from '$lib/models.ts';

export const ssr = true;

export const load: PageServerLoad = async () => {
	const models = await load_models();
	if (models) {
		const props: PropsAllModels = {
			models: models
		};
		return props;
	}

	error(404, 'Not found');
};
