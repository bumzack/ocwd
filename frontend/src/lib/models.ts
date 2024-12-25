export interface FeOllamaModel {
	id: number;
	name: string;
	model: string;
	size: number;
	detailFormat: string;
	detailFamily: string;
	detailParameterSize: string;
	detailQuantizationLevel: string;
	created: Date;
}

export interface OllamaModel {
	id: number;
	name: string;
	model: string;
	size: number;
	created: Date;
	temperature: number;
	numCtx: number;
	seed: number;
	topK: number;
	topP: number;
	checked: boolean;
}

export interface FeRunModel {
	modelId: number;
	temperature: number;
	numCtx: number;
	seed: number;
	topK: number;
	topP: number;
}

export interface FeRunModelRequest {
	prompt: string;
	models: FeRunModel[];
}

export interface FeOllamaChatQueueResponse {
	modelId: number;
	promptId: number;
	state: string;
	created: Date;
}

export interface InsertModelsResponse {
	model: string;
	name: string;
	modelId: number | undefined;
	result: string;
}

export interface FeOllamaPrompt {
	id: number;
	prompt: string;
	created: Date;
}

export interface FeOllamaChat {
	id: number;
	modelName: string;
	modelSize: string;
	response: string;
	temperature: number | undefined;
	numCtx: number | undefined;
	seed: number | undefined;
	topK: number | undefined;
	topP: number | undefined;
	created: Date;
	prompt: string;
	durationMs: number;
}

export type PropsChatPrompt = {
	chats: FeOllamaChat[];
	prompt: FeOllamaPrompt;
};

export type PropsAllPrompts = {
	prompts: FeOllamaPrompt[];
};

export type PropsAllModels = {
	models: OllamaModel[];
};

export type PropsAllChats = {
	chats: FeOllamaChat[];
};
