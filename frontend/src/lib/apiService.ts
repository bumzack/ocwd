import type {
	CreateModelRequest,
	FeDbOllamaModel,
	FeOllamaChat,
	FeOllamaChatQueue,
	FeOllamaChatQueueResponse,
	FeOllamaModel,
	FeOllamaPrompt,
	FeOllamaRunningModel,
	FeRunModelRequest,
	FeUpdateOllamaChatResult,
	InsertModelsResponse,
	OllamaInformation
} from './models';

//const server = 'http://10.0.0.48:3023';
const server = 'http://127.0.0.1:3023';

export const load_local_models = async (): Promise<FeOllamaModel[]> => {
	try {
		const response = await fetch(server + '/api/model', {
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
	try {
		const response = await fetch(server + '/api/dbmodel', {
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
	try {
		const response = await fetch(server + '/api/model/loaded', {
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
	try {
		const response = await fetch(server + '/api/model/enqueue', {
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
	try {
		const response = await fetch(server + '/api/model/import', {
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
	try {
		const response = await fetch(server + '/api/prompt', {
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
	try {
		const url = `${server}/api/chat/${promptId}`;

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

export const prompt_by_id = async (promptId: number): Promise<FeOllamaPrompt> => {
	try {
		const url = `${server}/api/prompt/${promptId}`;

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
	try {
		const url = `${server}/api/chat`;

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
	try {
		const response = await fetch(server + '/api/queue', {
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
	try {
		const url = `${server}/api/chat/result`;

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
export const streaming_response = async (): Promise<Response> => {
	try {
		const url = `${server}/ollama/api/stream`;
		return await fetch(url);
	} catch (e) {
		console.info(`error getting the streaming chat data. err: ${e}`);
	}

	return Promise.reject(new Error(`No chat response received."`));
};

export const create_model = async (request: CreateModelRequest): Promise<Response> => {
	try {
		const url = `${server}/api/model/create`;
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

export const ollama_model_information = async (model: string): Promise<OllamaInformation> => {
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
