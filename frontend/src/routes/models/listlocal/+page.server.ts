import { error } from '@sveltejs/kit';
import { load_db_models, load_local_models } from '$lib/apiService.ts';
import { type PropsFeLocalModels } from '$lib/models.ts';
import type { PageServerLoad } from './$types';

export const ssr = true;

export const load: PageServerLoad = async () => {
	const models = await load_db_models();
	const localModels = await load_local_models();

	if (models) {
		const props: PropsFeLocalModels = {
			localModels: localModels
		};
		return props;
	}

	error(404, 'Not found');
};
