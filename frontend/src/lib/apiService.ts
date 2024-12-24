import type {
	FeOllamaChat,
	FeOllamaChatQueueResponse,
	FeOllamaModel,
	FeOllamaPrompt,
	FeRunModelRequest,
	InsertModelsResponse
} from './models';

const server = 'http://10.0.0.48:3023';


export const load_models = async (): Promise<FeOllamaModel[]> => {
	try {
		const response = await fetch(server + '/api/model', {
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json, text/plain, */*'
			}
		});

		if (response.ok) {
			console.log(`response from Backend is ok!`);
		} else {
			console.log(`error: response from Backend is not ok!`);
		}

		const j: FeOllamaModel[] = await response.json();

		if (response.ok) {
			const models = j.map(m => {
				const model: FeOllamaModel = {
					created: m.created,
					detailFormat: m.detailFamily,
					detailFamily: m.detailFormat,
					detailParameterSize: m.detailParameterSize,
					detailQuantizationLevel: m.detailQuantizationLevel,
					id: m.id,
					model: m.model,
					name: m.name,
					size: m.size
				};
				return model;
			});
			if (models) {
				return models as FeOllamaModel[];
			} else {
				return Promise.reject(new Error(`No companies found "`));
			}
		} else {
			const error = new Error('error loading models');
			return Promise.reject(error);
		}
	} catch (e) {
		console.info(`error getting the company data ${e}`);
	}
	return [];
};


export const enqueue_models = async (request: FeRunModelRequest): Promise<FeOllamaChatQueueResponse[]> => {
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
			console.log(`response from Backend is ok!`);
		} else {
			console.log(`error: response from Backend is not ok!`);
		}

		const j: FeOllamaChatQueueResponse[] = await response.json();

		if (response.ok) {
			if (response) {
				// add fetchedAt helper (used in the UI to help differentiate requests)
				// 	return Object.assign(companies, { fetchedAt: formatDate(new Date()) });
				return j as FeOllamaChatQueueResponse[];
			} else {
				return Promise.reject(new Error(`No response found "`));
			}
		} else {
			const error = new Error('error loading models');
			return Promise.reject(error);
		}
	} catch (e) {
		console.info(`error getting the company data ${e}`);
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
			console.log(`response from Backend is ok!`);
		} else {
			console.log(`error: response from Backend is not ok!`);
		}

		if (response.ok) {
			const j: InsertModelsResponse[] = await response.json();

			if (response) {
				console.log(`response from Backend is ok! response ${JSON.stringify(response, null, 4)}`);

				// add fetchedAt helper (used in the UI to help differentiate requests)
				// 	return Object.assign(companies, { fetchedAt: formatDate(new Date()) });
				return j as InsertModelsResponse[];

			} else {
				return Promise.reject(new Error(`No response found "`));
			}
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
			console.log(`response from Backend is ok!`);
		} else {
			console.log(`error: response from Backend is not ok!`);
		}

		if (response.ok) {
			const j: FeOllamaPrompt[] = await response.json();

			if (response) {
				console.log(`response from Backend is ok! response ${JSON.stringify(response, null, 4)}`);

				// add fetchedAt helper (used in the UI to help differentiate requests)
				// 	return Object.assign(companies, { fetchedAt: formatDate(new Date()) });
				return j as FeOllamaPrompt[];

			} else {
				return Promise.reject(new Error(`No response found "`));
			}
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
			console.log(`response from Backend is ok!`);
		} else {
			console.log(`error: response from Backend is not ok!`);
		}

		if (response.ok) {
			const j: FeOllamaChat[] = await response.json();

			if (response) {
				console.log(`response from Backend is ok! response ${JSON.stringify(response, null, 4)}`);

				// add fetchedAt helper (used in the UI to help differentiate requests)
				// 	return Object.assign(companies, { fetchedAt: formatDate(new Date()) });
				return j as FeOllamaChat[];

			} else {
				return Promise.reject(new Error(`No response found "`));
			}
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
			console.log(`response from Backend is ok!`);
		} else {
			console.log(`error: response from Backend is not ok!`);
		}

		if (response.ok) {
			const j: FeOllamaPrompt = await response.json();

			if (response) {
				console.log(`response from Backend is ok! response ${JSON.stringify(response, null, 4)}`);

				// add fetchedAt helper (used in the UI to help differentiate requests)
				// 	return Object.assign(companies, { fetchedAt: formatDate(new Date()) });
				return j as FeOllamaPrompt;

			} else {
				return Promise.reject(new Error(`No response found "`));
			}
		} else {
			const error = new Error('error loading models');
			return Promise.reject(error);
		}
	} catch (e) {
		console.info(`error getting the company data ${e}`);
	}
	return Promise.reject(new Error(`No model response received."`));
};

