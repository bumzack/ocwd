<script lang="ts">
	import type { ModelChatParams, OllamaModel } from '$lib/models.ts';
	import { chat } from '$lib/apiService.ts';
	import type { PageData } from './$types';
	import { marked } from 'marked';
	import type { ChatRequest, FeLiveChatRequest, FeLiveChatResponse, MessagePingPong } from '$lib/livemodels.ts';

	let { data }: { data: PageData } = $props();
	let models: OllamaModel[] = $state(data.models);

	console.log(`models ${JSON.stringify(models, null, 4)}`);

	// let result: string = $state('');
	let hasData: boolean = $derived(models != undefined && models != null);
	let prompt: string = $state('');
	let selectedModelId: string = $state('');
	let showInsights: boolean = $state(false);

	let r: ChatRequest = {
		format: '', messages: undefined, model: '', options: undefined, prompt: '', stream: false, tools: undefined
	};
	let first: MessagePingPong = {
		request_message: r,
		response_messages: [],
		markdown: undefined
	};

	const init: FeLiveChatResponse = { req_resp: [first] };
	let dialogue: FeLiveChatResponse = $state(init);

	const promptEmpty = () => {
		return prompt !== null && prompt !== undefined && prompt.trim().length > 0;
	};

	let modelParam: ModelChatParams = $state({
		temperature: 0.1,
		seed: 23,
		numCtx: 4096,
		topK: 5,
		topP: 23
	});

	// close
	const send_request = async (): Promise<void> => {
		let req: FeLiveChatRequest = {
			modelId: +selectedModelId,
			numCtx: modelParam.numCtx,
			prompt: prompt,
			seed: modelParam.seed,
			temperature: modelParam.temperature,
			topK: modelParam.topK,
			topP: modelParam.topP,
			enableTools: false,
			responses: undefined,
			messages: []
		};

		console.log(`sending live request: ${JSON.stringify(req)}`);

		chat(req).then(response => {
			console.log(`response ${JSON.stringify(response, null, 4)}`);
		});
	};

	let disabled = $derived(!promptEmpty() || (selectedModelId == undefined || selectedModelId.length > 0));

	$effect(() => {
		console.log(`disabled ${disabled}`);
		console.log(`modelParam ${JSON.stringify(modelParam, null, 4)}`);
		console.log(`selectedModel ${selectedModelId}`);
	});
</script>

<div class="container-fluid">

	{#each dialogue.req_resp as req_res }
		<div class="row">
			<div class="col-lg-6">
				<h1>Have a Live-Chat</h1>
				<div class="row">
					<div class="col-lg-12">
						<div class="card">
							<h5 class="card-header">You</h5>
							<div class="card-body">
							<textarea bind:value={prompt} class="form-control" id="exampleFormControlTextarea1" rows="3"
												placeholder="Do your worst!"></textarea>
							</div>
							<div class="col-md-12">
								<div class="form-check">
									<button class="btn btn-primary" onclick={send_request}>Chat!!</button>
								</div>
							</div>
							<div class="card-footer">
								<div class="row">
									<div class="col-md-8">
										{#if hasData}
											<select
												bind:value={selectedModelId}
											>
												{#each models as model}
													<option value={model.id}>
														{model.model}
													</option>
												{/each}
											</select>
											<br />
										{:else}
											<p>no models available</p>
										{/if}
									</div>
									<div class="col-md-4">
										<div class="form-check">
											<input class="form-check-input" type="checkbox" bind:checked={showInsights} value=""
														 id="flexCheckChecked">
											<label class="form-check-label" for="flexCheckChecked">
												Show insights
											</label>
										</div>
									</div>
								</div>
							</div>
						</div>
					</div>
				</div>
			</div>
			{#if showInsights}
				<div class="col-lg-6">
					<h1>Insights </h1>
					<div class="row">
						<div class="col-lg-6">
						</div>
					</div>
				</div>
			{/if}
		</div>
		<br />
		{#if req_res.markdown !== undefined}
			<div class="row">
				<div class="col-lg-6">
					<div class="row">
						<div class="col-lg-12">
							<div class="card">
								<h5 class="card-header text-end">Assistant</h5>
								<div class="card-body">
									<p class="card-text">
										{@html marked(req_res.markdown, {
											breaks: true,
											sanitize: true,
											smartypants: true,
										}) }
								</div>
								<div class="card-footer">
									<div class="row">
										<div class="col-md-4">
											tokens: TODO
										</div>
										<div class="col-md-4">
											duration: TODO
										</div>
									</div>
								</div>
							</div>
						</div>
					</div>
				</div>
				{#if showInsights}
					<div class="col-lg-6">
						<h1>Insights </h1>
						<div class="row">
							<div class="col-lg-6">
							</div>
						</div>
					</div>
				{/if}
			</div>
		{/if}
	{/each}

</div>

<br />

