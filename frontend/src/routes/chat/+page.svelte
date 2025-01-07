<script lang="ts">

	import type { PageData } from './$types';
	import LoadingSpinner from '$lib/components/LoadingSpinner.svelte';
	import { enqueue_models, ollama_chat_update_result } from '$lib/apiService.ts';
	import type { FeOllamaChatQueueResponse, FeUpdateOllamaChatResult, OllamaModel } from '$lib/models.ts';

	let { data }: { data: PageData } = $props();
	let models: OllamaModel[] = $state(data.models);

	console.log(`models ${JSON.stringify(models)}`);

	let hasData: boolean = $derived(models != undefined && models != null);
	// let prompt: string = $state('');
	//
 	let running: boolean = $state(false);
	// let allSelected: boolean = $state(false);
	//
	// let btnText: string = $derived(allSelected ? 'unselect all' : 'select all');
	// let responses: Array<FeOllamaChatQueueResponse> = $state([]);
	//
	// const promptEmpty = () => {
	// 	return prompt !== null && prompt !== undefined && prompt.trim().length > 0;
	// };
	//
	// const modelSelected = () => {
	// 	const sel_models = models.filter(m => m.checked == true).length;
	// 	return models !== undefined && sel_models > 0;
	// };
	//
	// // TODO: this is not working, probably derived is not the way to go
	// let disabled = $derived(!promptEmpty() && !modelSelected());
	//
	// const start_magic = async (): Promise<void> => {
	// 	responses = [];
	// 	const selected_models = models.filter(m => m.checked);
	// 	const modelReq = selected_models.map(model => {
	// 		return {
	// 			modelId: model.id,
	// 			temperature: model.temperature,
	// 			numCtx: model.numCtx,
	// 			seed: model.seed,
	// 			topK: model.topK,
	// 			topP: model.topP
	// 		};
	// 	});
	//
	// 	const req = {
	// 		prompt: prompt,
	// 		models: modelReq
	// 	};
	//
	// 	running = true;
	// 	const res = await enqueue_models(req);
	// 	responses = res;
	//
	// 	running = false;
	// };

	// const select_all_models = (): void => {
	// 	if (allSelected) {
	// 		models.forEach(m => m.checked = false);
	// 	} else {
	// 		models.forEach(m => m.checked = true);
	// 	}
	// 	allSelected = !allSelected;
	// };
	//
	// const update_result = async (): Promise<void> => {
	// 	const req: FeUpdateOllamaChatResult = {
	// 		chatId: -1,
	// 		result: 'new result'
	// 	};
	// 	running = true;
	// 	const res = await ollama_chat_update_result(req);
	// 	console.log(`res update result ${JSON.stringify(res, null, 4)}`);
	// 	running = false;
	// };

</script>

 <LoadingSpinner show={running} />

{models}
<div class="container-fluid">
	<div class="row">
		<div class="col-lg-6">
			<h1>Ollama Chat</h1>
			<form class="row g-3">
<!--				<div class="col-md-12">-->
<!-- 						<label class="form-label" for="exampleFormControlTextarea1">Prompt:</label>-->
<!--						<textarea bind:value={prompt} class="form-control" id="exampleFormControlTextarea1"-->
<!--											rows="3"></textarea>-->
<!--				</div>-->
<!--				<div class="col-md-12">-->
<!--					<label class="form-label" for="exampleFormControlTextarea1">Prompt:</label>-->
<!--					<textarea bind:value={prompt} class="form-control" id="exampleFormControlTextarea1"-->
<!--										rows="3"></textarea>-->

<!--				</div>-->
<!--				<div class="col">-->
<!--					<button class="btn btn-primary" {disabled} onclick={start_magic}>Chat!</button>-->
<!--				</div>-->
<!--				<div class="col">-->
<!--					<button class="btn btn-primary" {disabled} onclick={start_magic}>Add to enqueue!</button>-->
<!--				</div>-->
			</form>

			<div class="row">
				<div class="col-12">
					<h2>enqueued chats</h2>
					<!--{#each responses as aresponse}-->
					<!--	<p>model id: {aresponse.modelId}, prompt id: {aresponse.promptId}, state {aresponse.state},-->
					<!--		created {aresponse.created}</p>-->
					<!--{/each}-->
				</div>
			</div>
		</div>

		<div class="col-lg-6">
			<h4>Available Models</h4>
			{#if hasData}
<!--				<form>-->
<!--					<button onclick={select_all_models} class="btn btn-primary">{btnText}</button>-->
<!--				</form>-->
				<p>{models.length} models</p>

				{#each models as model}
					<div class="card">
						<div class="card-header">
							<label class=" d-flex gap-2">
								<input class="form-check-input flex-shrink-0" type="checkbox"
											 bind:checked={model.checked}>
								<span>
								<small class="d-block text-body-secondary">{model.name}
									/ {(model.size / 1024 / 1024 / 1024).toLocaleString('de-DE', { minimumFractionDigits: 0 })}</small>
							</span>
							</label>
						</div>
						<div class="card-body">
							<form class="row g-3">
								<div class="col-md-3">
									<label for="inputNumCtx" class="form-label">NumCtx</label>
									<input bind:value={model.numCtx} type="number" class="form-control"
												 id="inputNumCtx">
								</div>
								<div class="col-md-2">
									<label for="inputSeed" class="form-label">Seed</label>
									<input bind:value={model.seed} type="number" class="form-control" id="inputSeed">
								</div>
								<div class="col-md-3">
									<label for="inputTemperature" class="form-label">Temperature</label>
									<input bind:value={model.temperature} type="number" class="form-control"
												 id="inputTemperature">
								</div>
								<div class="col-md-2">
									<label for="inputTopK" class="form-label">top-k</label>
									<input bind:value={model.topK} type="number" class="form-control" id="inputTopK">
								</div>
								<div class="col-md-2">
									<label for="inputTopP" class="form-label">top-k</label>
									<input bind:value={model.topP} type="number" class="form-control" id="inputTopP">
								</div>
							</form>
						</div>
					</div>
					<br />
				{/each}
			{:else}
				<p>no data available</p>
			{/if}
		</div>
	</div>
</div>
