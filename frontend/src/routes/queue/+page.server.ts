import { error } from '@sveltejs/kit';
import { queue_load } from '$lib/apiService.ts';
import { type PropsFeAllQueue } from '$lib/models.ts';
import type { PageServerLoad } from './$types';

export const ssr = true;

export const load: PageServerLoad = async () => {
	const queue = await queue_load();
	if (queue) {
		const props: PropsFeAllQueue = {
			queue: queue
		};
		return props;
	}

	error(404, 'Not found');
};
