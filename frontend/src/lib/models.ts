export interface FeOllamaModel {
	name: string;
	model: string;
	size: number;
	detailFormat: string;
	detailFamily: string;
	detailParameterSize: string;
	detailQuantizationLevel: string;
}

export interface FeDbOllamaModel {
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
	result: string;
	temperature: number;
	numCtx: number;
	seed: number;
	topK: number;
	topP: number;
	created: Date;
	prompt: string;
	durationMs: number;
}

export interface FeOllamaRunningModel {
	name: string;
	model: string;
	size: number;
	format: string;
	family: string;
	parameterSize: string;
	detailQuantizationLevel: string;
	detailParameterSize: string;
}

export type PropsChatPrompt = {
	chats: FeOllamaChat[];
	prompt: FeOllamaPrompt;
};

export type PropsAllPrompts = {
	prompts: FeOllamaPrompt[];
};

export type PropsFeDbModels = {
	models: FeDbOllamaModel[];
};

export type PropsFeLocalModels = {
	localModels: FeOllamaModel[];
};

export type PropsAllModels = {
	models: OllamaModel[];
};

export type PropsAllChats = {
	chats: FeOllamaChat[];
};

export type PropsFeAllRunningModels = {
	models: FeOllamaRunningModel[];
};

export interface FeOllamaChatQueue {
	id: number;
	modelId: number;
	promptId: number;
	state: string;
	temperature: number;
	seed: number;
	numCtx: number;
	topK: number;
	topP: number;
	created: Date;
	updated: Date;
}

export type PropsFeAllQueue = {
	queue: FeOllamaChatQueue[];
};

export interface FeUpdateOllamaChatResult {
	chatId: number;
	result: string;
}

// TODO any
export interface Message {
	role: string;
	content: string;
	images: Array<string> | undefined;
	tool_calls: any | undefined;
}

export interface ChatResponse {
	model: string;
	created_at: string;
	response: string;
	done: boolean;
	context: Array<number> | undefined;
	total_duration: number | undefined;
	load_duration: number | undefined;
	prompt_eval_count: number | undefined;
	prompt_eval_duration: number | undefined;
	eval_count: number | undefined;
	eval_duration: number | undefined;
	done_reason: string | undefined;
	message: Message | undefined;
}

export interface ModelDetails {
	format: string;
	family: string;
	families: Array<string> | undefined;
	parameter_size: string;
	quantization_level: string;
}

export interface FeOllamaInformation {
	modelfile: string;
	parameters: string | undefined;
	details: ModelDetails;
	modelInfo: Map<string, string | number | boolean | Array<string>>;
	license: string;
	template: string;
	modifiedAt: Date;
}

export type PropsModelInformation = {
	information: FeOllamaInformation;
};

export type CreateModelRequest = {
	model: string;
	modelfile: string;
	quantize: string | undefined;
};

export interface CreateModelResponse {
	status: string;
}
