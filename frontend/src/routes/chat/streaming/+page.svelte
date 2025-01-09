<script lang="ts">
	import type { ChatResponse, FeStreamingRequest, ModelChatParams, OllamaModel } from '$lib/models.ts';
	import { marked } from 'marked';
	import { streaming_response } from '$lib/apiService.ts';
	import type { PageData } from '../../../../.svelte-kit/types/src/routes/prompt/add/$types';

	let { data }: { data: PageData } = $props();
	let models: OllamaModel[] = $state(data.models);

	console.log(`models ${JSON.stringify(models, null, 4)}`);

	let result: string = $state('');
	let hasData: boolean = $derived(models != undefined && models != null);
	let prompt: string = $state('');
	let selectedModelId: string = $state('');

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
	const start_magic = async (): Promise<void> => {
		let req: FeStreamingRequest = {
			modelId: +selectedModelId,
			numCtx: modelParam.numCtx,
			prompt: prompt,
			seed: modelParam.seed,
			temperature: modelParam.temperature,
			topK: modelParam.topK,
			topP: modelParam.topP
		};

		console.log(`sending streaming request: ${JSON.stringify(req)}`);

		streaming_response(req).then(response => {
			result = '';
			// `response` is a stream!
			const reader = response.body.pipeThrough(new TextDecoderStream()).getReader();

			// Find out how big the response is.
			const length = response.headers.get('Content-Length');
			console.log('length  ' + length);
			// Initialize how much we've received. Nothing so far.
			let received = 0;

			// What happens when the stream delivers a chunk?
			const onReadChunk = (chunk: ReadableStreamReadResult<string>) => {
				// Each chunk has a `done` property. If it's done,
				if (chunk.done) {
					// Update the UI so people know it's done.
					console.log(`new chunk is done ${chunk}`);
					return;
				}

				// If it's not done, increment the received variable, and the bar's fill.
				// received += chunk.;
				console.log(`received ${received}`);
				console.log(`new chunk is done ${JSON.stringify(chunk.value)}`);

				const values = chunk.value.trim().split('}\n');

				// I tried this npm package "@streamparser/json-whatwg": "^0.0.21",
				// but it is "done" after the first complete JSON. so it does not work as I would expect
				// don't know why, but yeah - here we are.
				// const parser = new JSONParser({ stringBufferSize: undefined, paths: ['$.*'], keepStack: false });
				// 		const reader = response.body.pipeThrough(parser).getReader();


				// oh boy ph boy: if something is an ugly piece of code, then is it!!!
				// but: it works, at least for my use cases
				let responses = values.map(v => {
					let js = v;
					if (!v.endsWith('}')) {
						js += '}';
					}
					let chatResponse: ChatResponse = JSON.parse(js);
					return chatResponse;
				});
				responses.forEach(r => {
					if (!r.done) {
						console.log(`adding response:  ||${JSON.stringify(r)}||`);
						result += r.response;
					}
				});


				// Keep reading, and keep doing this AS LONG AS IT'S NOT DONE.
				reader.read().then(onReadChunk);
			};

			// Do the first read().
			reader.read().then(onReadChunk);
		});
	};

	let disabled = $derived(!promptEmpty() || (selectedModelId == undefined || selectedModelId.length > 0));


	$effect(() => {
		console.log(`disabled ${disabled}`);
		console.log(`modelParam ${modelParam}`);
		console.log(`selectedModel ${selectedModelId}`);
	});
</script>

<div class="container-fluid">
	<div class="row">
		<div class="col-lg-12">
			<h1>Have a Live-Chat</h1>
			<div class="row">
				<div class="col-lg-8">
					<form class="row g-3">
						<div class="col-md-12">
							<label class="form-label" for="exampleFormControlTextarea1">Prompt:</label>
							<textarea bind:value={prompt} class="form-control" id="exampleFormControlTextarea1" rows="3"></textarea>
						</div>
						<div class="col">
							<button class="btn btn-primary" {disabled} onclick={start_magic}>Chat!!</button>
						</div>
					</form>
					<br>
					<p>{@html marked(result, {
						breaks: true,
						sanitize: true,
						smartypants: true,
					}) }</p>
				</div>

				<div class="col-lg-4">
					<h4>Available Models</h4>
					{#if hasData}
						<p>{models.length} models</p>

						<div class="card-body">
							<form class="row g-3">
								<div class="col-md-3">
									<label for="inputNumCtx" class="form-label form-control-sm">NumCtx</label>
									<input bind:value={modelParam.numCtx} type="number" class="form-control form-control-sm"
												 id="inputNumCtx">
								</div>
								<div class="col-md-2">
									<label for="inputSeed" class="form-label form-control-sm">Seed</label>
									<input bind:value={modelParam.seed} type="number" class="form-control form-control-sm" id="inputSeed">
								</div>
								<div class="col-md-3">
									<label for="inputTemperature" class="form-label form-control-sm">Temperature</label>
									<input bind:value={modelParam.temperature} type="number" class="form-control form-control-sm"
												 id="inputTemperature">
								</div>
								<div class="col-md-2">
									<label for="inputTopK" class="form-label form-control-sm">top-k</label>
									<input bind:value={modelParam.topK} type="number" class="form-control form-control-sm" id="inputTopK">
								</div>
								<div class="col-md-2">
									<label for="inputTopP" class="form-label form-control-sm">top-k</label>
									<input bind:value={modelParam.topP} type="number" class="form-control form-control-sm" id="inputTopP">
								</div>
							</form>
						</div>
						<br />

						{#each models as model}
							<div class="card">
								<div class="card-header">
									<label class=" d-flex gap-2">
										<input class="form-check-input flex-shrink-0" type="radio" name="modelSelect"
													 value={model.id} bind:group={selectedModelId}>
										<span>
											<small class="d-block text-body-secondary">{model.name}
												/ {(model.size / 1024 / 1024 / 1024).toLocaleString('de-DE', { minimumFractionDigits: 0 })}</small>
										</span>
									</label>
								</div>
								<br />
							</div>
						{/each}
					{:else}
						<p>no data available</p>
					{/if}
				</div>
			</div>
		</div>
	</div>
</div>
