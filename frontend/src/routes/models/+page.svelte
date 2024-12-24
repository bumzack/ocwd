<script lang="ts">

	import { type FeOllamaModel } from '$lib/models.ts';
	import { load_models, models_import } from '$lib/apiService.ts';
	import LoadingSpinner from '$lib/components/LoadingSpinner.svelte';

	let models = $state([]);
	let hasData = $derived(models != undefined && models != null);
	let running = $state(false);
	let responses = $state([]);


	const load_data = async (): Promise<FeOllamaModel[]> => {
		return await load_models();
	};

	load_data().then(d => models = d);

	const importModels = async (): Promise<void> => {
		running = true;
		const res = await models_import();
		responses = res;
		console.log(`res  ` + JSON.stringify(res, null, 4));

		running = false;
	};

</script>

<LoadingSpinner show={!hasData} />

<div class="container">
	<div class="row">
		<div class="col-lg-12">
			<h1>Import models</h1>

			<button class="btn btn-primary" onclick={importModels}>Import Models</button>
		</div>
	</div>
	<div class="row">
		<div class="col-lg-12">
			<h1>Ollama models local</h1>
			{#if hasData}
				<table class="table table-striped">
					<thead>
					<tr>
						<th>
							id
						</th>
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
							detail_format
						</th>
						<th> detail_family
						</th>
						<th>detail_parameter_size
						</th>
						<th>detail_quantization_level
						</th>
						<th>created
						</th>
					</tr>
					</thead>
					<tbody>
					{#each models as model}
						<tr>
							<td>
								{model.id}
							</td>
							<td>
								{model.name}
							</td>
							<td>
								{model.model}
							</td>
							<td>
								{model.size}
							</td>
							<td>
								{model.detail_format}
							</td>
							<td>
								{model.detail_family}
							</td>
							<td>
								{model.detail_parameter_size}
							</td>
							<td>
								{model.detail_quantization_level}
							</td>
							<td>
								{model.created.toLocaleString()}
							</td>
						</tr>
					{/each}
					</tbody>
				</table>
			{:else}
				<p>no data available</p>
			{/if}
		</div>
	</div>

</div>


<style>
    @import 'bootstrap/dist/css/bootstrap.min.css';

</style>