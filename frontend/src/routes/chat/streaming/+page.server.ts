import { error } from '@sveltejs/kit';
import { load_db_models } from '$lib/apiService.ts';
import { type OllamaModel, type PropsAllModels } from '$lib/models.ts';
import type { PageServerLoad } from './$types';

export const ssr = true;

export const load: PageServerLoad = async () => {
	const models = await load_db_models();
	// console.log(`models ${JSON.stringify(models)}`);
	if (models) {
		let mms = models.map((mod) => {
			const om: OllamaModel = {
				checked: false,
				created: mod.created,
				id: mod.id,
				model: mod.model,
				name: mod.name,
				numCtx: 4096,
				seed: 23,
				size: mod.size,
				temperature: 0.1,
				topK: 20.0,
				topP: 0.5
			};
			return om;
		});

		const props: PropsAllModels = {
			models: mms
		};
		return props;
	}

	error(404, 'Not found');
};
