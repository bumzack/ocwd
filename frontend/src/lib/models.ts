export interface FeOllamaModel {
	id: number,
	name: string,
	model: string,
	size: number,
	detailFormat: string,
	detailFamily: string,
	detailParameterSize: string,
	detailQuantizationLevel: string,
	created: Date,
}

export interface FeRunModel {
	modelId: number,
	temperature: number | undefined,
	numCtx: number | undefined,
	seed: number | undefined,
	topK: number | undefined,
	topP: number | undefined,
}

export interface FeRunModelRequest {
	prompt: string,
	models: FeRunModel[],
}

export interface FeOllamaChatQueueResponse {
	modelId: number,
	promptId: number,
	state: string,
	created: Date,
}

export interface InsertModelsResponse {
	model: string,
	name: string,
	model_id: number | undefined,
	result: string,
}

export interface FeOllamaPrompt {
	id: number,
	prompt: string,
	created: Date,
}

export interface FeOllamaChat {
	id: number,
	modelName: string,
	modelSize: string,
	response: string,
	temperature: number | undefined,
	numCtx: number | undefined,
	seed: number | undefined,
	topK: number | undefined,
	topP: number | undefined,
	created: Date,
	durationMs: number,
}

export type Props = {
	chats: FeOllamaChat[] | undefined;
	prompt: FeOllamaPrompt | undefined;
};
