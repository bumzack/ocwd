import { error } from '@sveltejs/kit';
import { ollama_model_information } from '$lib/apiService.ts';
import { type PropsModelInformation } from '$lib/models.ts';
import type { PageServerLoad } from './$types';

export const ssr = true;

export const load: PageServerLoad = async ({ params }) => {
	const model = params.model;
	const information = await ollama_model_information(model);

	console.log(`information ${JSON.stringify(information, null, 4)}`);

	if (information) {
		const props: PropsModelInformation = {
			information: information
		};
		return props;
	}

	error(404, 'Not found');
};
