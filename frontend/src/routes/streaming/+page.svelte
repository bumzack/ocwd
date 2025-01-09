<script lang="ts">
	import type { ChatResponse } from '$lib/models.ts';
	import { marked } from 'marked';
	import { streaming_response } from '$lib/apiService.ts';

	let result: string = $state('');

	// close
	const start_magic = async (): Promise<void> => {
		streaming_response().then(response => {
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
				console.log(`new chunk is done ${JSON.stringify( chunk.value)}`);

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
</script>


<div class="container-fluid">
	<div class="row">
		<div class="col-lg-12">
			<h1>Import models</h1>
			<button class="btn btn-primary" onclick={start_magic}>Let the magic do its work!</button>

			<p>{@html marked(result, {
				breaks: true,
				sanitize: true,
				smartypants: true,
			}) }</p>
		</div>
	</div>
</div>
