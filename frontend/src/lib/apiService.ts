import type {
	CreateModelRequest,
	FeDbOllamaModel,
	FeOllamaChat,
	FeOllamaChatQueue,
	FeOllamaChatQueueResponse,
	FeOllamaInformation,
	FeOllamaModel,
	FeOllamaPrompt,
	FeOllamaRunningModel,
	FeRunModelRequest,
	FeStreamingRequest,
	FeUpdateOllamaChatResult,
	InsertModelsResponse
} from './models';

import { env } from '$env/dynamic/public';
import type { FeLiveChatRequest, FeLiveChatResponse } from '$lib/livemodels.ts';

export const load_local_models = async (): Promise<FeOllamaModel[]> => {
	const server = env.PUBLIC_BACKEND_URL;
	try {
		const url = `${server}/api/model`;
		console.log(`url ${url}`);

		const response = await fetch(url, {
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json, text/plain, */*'
			}
		});

		if (response.ok) {
			return await response.json();
		} else {
			const error = new Error('error loading models');
			return Promise.reject(error);
		}
	} catch (e) {
		console.info(`error getting model data ${e}`);
	}
	return [];
};

export const load_db_models = async (): Promise<FeDbOllamaModel[]> => {
	const server = env.PUBLIC_BACKEND_URL;
	try {
		const url = `${server}/api/dbmodel`;
		console.log(`url ${url}`);

		const response = await fetch(url, {
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json, text/plain, */*'
			}
		});

		if (response.ok) {
			return await response.json();
		} else {
			const error = new Error('error loading models');
			return Promise.reject(error);
		}
	} catch (e) {
		console.info(`error getting model data ${e}`);
	}
	return [];
};

export const load_running_models = async (): Promise<FeOllamaRunningModel[]> => {
	const server = env.PUBLIC_BACKEND_URL;
	try {
		const url = `${server}/api/model/loaded`;
		console.log(`url ${url}`);

		const response = await fetch(url, {
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json, text/plain, */*'
			}
		});

		if (response.ok) {
			return await response.json();
		} else {
			const error = new Error('error loading loaded models');
			return Promise.reject(error);
		}
	} catch (e) {
		console.info(`error getting running model data ${e}`);
	}
	return [];
};

export const enqueue_models = async (
	request: FeRunModelRequest
): Promise<FeOllamaChatQueueResponse[]> => {
	const server = env.PUBLIC_BACKEND_URL;
	try {
		const url = `${server}/api/model/enqueue`;
		console.log(`url ${url}`);

		const response = await fetch(url, {
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json, text/plain, */*'
			},
			method: 'POST',
			body: JSON.stringify(request)
		});

		if (response.ok) {
			return await response.json();
		} else {
			const error = new Error('error loading models');
			return Promise.reject(error);
		}
	} catch (e) {
		console.info(`error getting enqueing the new prompts ${e}`);
	}
	return Promise.reject(new Error(`No model response received."`));
};

export const models_import = async (): Promise<InsertModelsResponse[]> => {
	const server = env.PUBLIC_BACKEND_URL;
	try {
		const url = `${server}/api/model/import`;
		console.log(`url ${url}`);

		const response = await fetch(url, {
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json, text/plain, */*'
			},
			method: 'POST'
		});

		if (response.ok) {
			return await response.json();
		} else {
			const error = new Error('error loading models');
			return Promise.reject(error);
		}
	} catch (e) {
		console.info(`error getting the company data ${e}`);
	}
	return Promise.reject(new Error(`No model response received."`));
};

export const prompts_load = async (): Promise<FeOllamaPrompt[]> => {
	const server = env.PUBLIC_BACKEND_URL;
	try {
		const url = `${server}/api/prompt`;
		console.log(`url ${url}`);

		const response = await fetch(url, {
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json, text/plain, */*'
			},
			method: 'GET'
		});

		if (response.ok) {
			return await response.json();
		} else {
			const error = new Error('error loading models');
			return Promise.reject(error);
		}
	} catch (e) {
		console.info(`error getting the company data ${e}`);
	}
	return Promise.reject(new Error(`No model response received."`));
};

export const chats_load_by_prompt_id = async (promptId: number): Promise<FeOllamaChat[]> => {
	const server = env.PUBLIC_BACKEND_URL;
	try {
		const url = `${server}/api/chat/byid/${promptId}`;
		console.log(`url ${url}`);

		const response = await fetch(url, {
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json, text/plain, */*'
			},
			method: 'GET'
		});

		console.log(`resp ${JSON.stringify(response, null, 4)}`);
		if (response.ok) {
			return await response.json();
		} else {
			const error = new Error('error loading prompt by id');
			return Promise.reject(error);
		}
	} catch (e) {
		console.info(`error getting the company data ${e}`);
	}
	return Promise.reject(new Error(`No model response received."`));
};

export const prompt_by_id = async (promptId: number): Promise<FeOllamaPrompt> => {
	const server = env.PUBLIC_BACKEND_URL;
	try {
		const url = `${server}/api/prompt/${promptId}`;
		console.log(`url ${url}`);

		const response = await fetch(url, {
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json, text/plain, */*'
			},
			method: 'GET'
		});

		if (response.ok) {
			return await response.json();
		} else {
			const error = new Error('error loading models');
			return Promise.reject(error);
		}
	} catch (e) {
		console.info(`error getting the company data ${e}`);
	}
	return Promise.reject(new Error(`No model response received."`));
};

export const chats_load_all = async (): Promise<FeOllamaChat[]> => {
	const server = env.PUBLIC_BACKEND_URL;
	try {
		const url = `${server}/api/chat/all`;
		console.log(`url ${url}`);

		const response = await fetch(url, {
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json, text/plain, */*'
			},
			method: 'GET'
		});

		if (response.ok) {
			return await response.json();
		} else {
			const error = new Error('error loading models');
			return Promise.reject(error);
		}
	} catch (e) {
		console.info(`error getting the company data ${e}`);
	}
	return Promise.reject(new Error(`No model response received."`));
};

export const queue_load = async (): Promise<FeOllamaChatQueue[]> => {
	const server = env.PUBLIC_BACKEND_URL;
	try {
		const url = `${server}/api/queue`;
		console.log(`url ${url}`);

		const response = await fetch(url, {
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json, text/plain, */*'
			},
			method: 'GET'
		});

		if (response.ok) {
			return await response.json();
		} else {
			const error = new Error('error loading models');
			return Promise.reject(error);
		}
	} catch (e) {
		console.info(`error getting queue entries ${e}`);
	}
	return Promise.reject(new Error(`No model response received."`));
};

export const ollama_chat_update_result = async (
	request: FeUpdateOllamaChatResult
): Promise<FeOllamaChat> => {
	const server = env.PUBLIC_BACKEND_URL;
	try {
		const url = `${server}/api/chat/result`;
		console.log(`url ${url}`);

		const response = await fetch(url, {
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json, text/plain, */*'
			},
			body: JSON.stringify(request),
			method: 'PUT'
		});

		if (response.ok) {
			return await response.json();
		} else {
			const error = new Error('error loading models');
			return Promise.reject(error);
		}
	} catch (e) {
		console.info(`error getting the company data ${e}`);
	}
	return Promise.reject(new Error(`No model response received."`));
};

// https://stackoverflow.com/questions/74330190/how-to-respond-with-a-stream-in-a-sveltekit-server-load-function
// this is probably not the way
export const streaming_response = async (req: FeStreamingRequest): Promise<Response> => {
	const server = env.PUBLIC_BACKEND_URL;
	try {
		const url = `${server}/ollama/api/stream`;
		console.log(`url ${url}`);

		return await fetch(url, {
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json, text/plain, */*'
			},
			body: JSON.stringify(req),
			method: 'POST'
		});
	} catch (e) {
		console.info(`error getting the streaming chat data. err: ${e}`);
	}
	return Promise.reject(new Error(`No chat response received."`));
};

export const create_model = async (request: CreateModelRequest): Promise<Response> => {
	const server = env.PUBLIC_BACKEND_URL;
	try {
		const url = `${server}/api/model/create`;
		console.log(`url ${url}`);

		return await fetch(url, {
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json, text/plain, */*'
			},
			body: JSON.stringify(request),
			method: 'POST'
		});
	} catch (e) {
		console.info(`error getting the streaming chat data. err: ${e}`);
	}

	return Promise.reject(new Error(`No chat response received."`));
};

export const ollama_model_information = async (model: string): Promise<FeOllamaInformation> => {
	const server = env.PUBLIC_BACKEND_URL;
	try {
		const url = `${server}/api/model/details/${model}`;
		console.log(`url  ${url}`);

		const response = await fetch(url, {
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json, text/plain, */*'
			},
			method: 'GET'
		});

		if (response.ok) {
			return await response.json();
		} else {
			const error = new Error('error loading model information');
			console.log(`error ${error}`);
			return Promise.reject(error);
		}
	} catch (e) {
		console.info(`error getting the model information ${e}`);
	}
	return Promise.reject(new Error(`No model information received."`));
};

export const chat = async (req: FeLiveChatRequest): Promise<FeLiveChatResponse> => {
	const server = env.PUBLIC_BACKEND_URL;
	try {
		const url = `${server}/api/chat`;
		console.log(`url ${url}`);

		const response = await fetch(url, {
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json, text/plain, */*'
			},
			body: JSON.stringify(req),
			method: 'POST'
		});
		if (response.ok) {
			return await response.json();
		} else {
			const error = new Error('error getting chat response');
			return Promise.reject(error);
		}
	} catch (e) {
		console.info(`error getting the chat data. err: ${e}`);
	}
	return Promise.reject(new Error(`No chat response received."`));
};
