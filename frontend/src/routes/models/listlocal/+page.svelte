<script lang="ts">
	import type { FeOllamaModel } from '$lib/models.ts';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();
	let models: FeOllamaModel[] = $state(data.localModels);
	let hasData = $derived(models != undefined && models != null);
</script>

<div class="container-fluid">
	<div class="row">
		<div class="col-lg-12">
			<h1>Local Ollama Models</h1>
			{#if hasData}
				<p>{models.length} models</p>
				<table class="table table-striped">
					<thead>
					<tr>
						<th>
							name
						</th>
						<th>
							model
						</th>
						<th>
							size
						</th>
						<th>
							format
						</th>
						<th>
							family
						</th>
						<th>
							parameter size
						</th>
						<th>
							quantization level
						</th>

					</tr>
					</thead>
					<tbody>
					{#each models as model}
						<tr>
							<td>
								{model.name}
							</td>
							<td>
								<a href="/models/information/{model.model}">{model.model}</a>
							</td>
							<td>
								{(model.size / 1024 / 1024 / 1024).toLocaleString('de-DE', { minimumFractionDigits: 0 })}
							</td>
							<td>
								{model.detailFormat}
							</td>
							<td>
								{model.detailFamily}
							</td>
							<td>
								{model.detailParameterSize}
							</td>
							<td>
								{model.detailQuantizationLevel}
							</td>
						</tr>
					{/each}
					</tbody>
				</table>
			{:else}
				<p>no local models available</p>
			{/if}
		</div>
	</div>
</div>

