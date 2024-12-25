import { error } from '@sveltejs/kit';
import { chats_load_all } from '$lib/apiService.ts';
import { type PropsAllChats } from '$lib/models.ts';
import type {PageServerLoad} from "./$types";

export const ssr = true;

export const load: PageServerLoad = async () => {
	const chats = await chats_load_all();
	if (chats) {
		const props: PropsAllChats = {
			chats: chats
		};
		return props;
	}

	error(404, 'Not found');
};
