import { error } from '@sveltejs/kit';
import { load_db_models } from '$lib/apiService.ts';
import { type PropsFeDbModels } from '$lib/models.ts';
import type { PageServerLoad } from './$types';

export const ssr = true;

export const load: PageServerLoad = async () => {
	const models = await load_db_models();

	console.log(`models ${JSON.stringify(models, null, 4)}`);

	if (models) {
		const props: PropsFeDbModels = {
			models: models
		};
		return props;
	}

	error(404, 'Not found');
};
