<script lang="ts">
	import type { CreateModelRequest, CreateModelResponse } from '$lib/models.ts';
	import { create_model } from '$lib/apiService.ts';

	let result: string = $state('');
	let modelName: string = $state('');
	let modelFile: string = $state('');
	let quantize: string = $state('');

	// close
	const send_create_model = async (): Promise<void> => {
		let q = quantize.trim();
		let req: CreateModelRequest = {
			model: modelName,
			modelfile: modelFile,
			quantize: q
		};

		create_model(req).then(response => {
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
					let chatResponse: CreateModelResponse = JSON.parse(js);
					return chatResponse;
				});
				responses.forEach(r => {
					console.log(`adding response:  ||${JSON.stringify(r)}||`);
					result += r.status + '<br/>';

				});

				// Keep reading, and keep doing this AS LONG AS IT'S NOT DONE.
				reader.read().then(onReadChunk);
			};

			// Do the first read().
			reader.read().then(onReadChunk);
		});
	};

	let showResult: boolean = $derived(result !== undefined && result.trim().length > 0);
</script>


<div class="container-fluid">
	<div class="row">
		<div class="col-lg-12">
			<h1>Create model</h1>
			<form class="row g-3">
				<div class="col-md-3">
					<label for="modelName" class="form-label">Model name</label>
					<input bind:value={modelName} type="text" class="form-control" id="modelName">
				</div>
				<div class="col-md-12">
					<label class="form-label" for="exampleFormControlTextarea1">Model file:</label>
					<textarea bind:value={modelFile} class="form-control" id="exampleFormControlTextarea1" rows="3"></textarea>
				</div>
				<div class="col-md-2">
					<label for="inputTopK" class="form-label">quantize</label>
					<input bind:value={quantize} type="text" class="form-control" id="inputTopK">
				</div>
				<button class="btn btn-primary" onclick={send_create_model}>Create Model</button>

			</form>
		</div>
	</div>

	{#if showResult}
		<div class="row">
			<div class="col-lg-12">
				<h1>Response</h1>
				<p>{@html result}</p>
			</div>
		</div>
	{/if}
</div>
