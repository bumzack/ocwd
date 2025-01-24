import type { Message } from '$lib/models.ts';

export interface Property {
	type: string;
	description: string;
	enums: Array<string> | undefined;
}

export interface Parameter {
	type: string;
	properties: Map<string, Property>;
	required: Array<string> | undefined;
}

export interface Function {
	name: string;
	description: string;
	parameters: Parameter;
}

export interface Format {
	type: string;
	function: Map<string, string>;
}

export interface Tool {
	type: string;
	function: Function;
}

export interface FeLiveRequest {
	modelId: number;
	prompt: string;
	seed: number;
	numCtx: number;
	temperature: number;
	topK: number;
	topP: number;
}

export interface FeLiveResponse {
	modelId: number;
	prompt: string;
	seed: number;
	numCtx: number;
	temperature: number;
	topK: number;
	topP: number;
}

export interface FeLiveChatRequest {
	modelId: number;
	prompt: string;
	seed: number;
	numCtx: number;
	temperature: number;
	topK: number;
	topP: number;
	enableTools: boolean;
	responses: FeLiveChatResponse | undefined;
	messages: Array<Message> | undefined;
}

export interface MessagePingPong {
	request_message: ChatRequest;
	response_messages: Array<ChatResponse>;
	markdown: string | undefined;
}

export interface FeLiveChatResponse {
	req_resp: Array<MessagePingPong>;
}

export interface ChatRequestOptions {
	num_keep: number | undefined;
	seed: number | undefined;
	num_predict: number | undefined;
	top_k: number | undefined;
	top_p: number | undefined;
	min_p: number | undefined;
	typical_p: number | undefined;
	repeat_last_n: number | undefined;
	temperature: number | undefined;
	repeat_penalty: number | undefined;
	presence_penalty: number | undefined;
	frequency_penalty: number | undefined;
	mirostat: number | undefined;
	mirostat_tau: number | undefined;
	mirostat_eta: number | undefined;
	penalize_newline: boolean | undefined;
	stop: Array<string> | undefined;
	numa: boolean | undefined;
	num_ctx: number | undefined;
	num_batch: number | undefined;
	num_gpu: number | undefined;
	main_gpu: number | undefined;
	low_vram: boolean | undefined;
	vocab_only: boolean | undefined;
	use_mmap: boolean | undefined;
	use_mlock: boolean | undefined;
	num_thread: number | undefined;
}

export interface ChatRequest {
	model: string;
	prompt: string | undefined;
	stream: boolean;
	options: ChatRequestOptions | undefined;
	messages: Array<Message> | undefined;
	format: string | undefined;
	tools: Array<Tool> | undefined;
}

export interface ChatResponse {
	model: string;
	created_at: string;
	response: string | undefined;
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
