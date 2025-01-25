import { error } from '@sveltejs/kit';
import { queue_load } from '$lib/apiService.ts';
import { type PropsFeAllQueue } from '$lib/models.ts';
import type { PageServerLoad } from './$types';

export const ssr = true;

export const load: PageServerLoad = async () => {
	const queue = await queue_load();

	console.log(`queue ${JSON.stringify(queue, null, 4)}`);

	if (queue) {
		console.log(`queue  ${JSON.stringify(queue, null, 4)}`);
		const props: PropsFeAllQueue = {
			queue: queue
		};
		return props;
	}

	error(404, 'Not found');
};
