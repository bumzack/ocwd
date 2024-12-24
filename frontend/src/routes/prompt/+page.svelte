<script lang="ts">

	import { type FeOllamaPrompt } from '$lib/models.ts';
	import { prompts_load } from '$lib/apiService.ts';
	import LoadingSpinner from '$lib/components/LoadingSpinner.svelte';

	let prompts = $state([]);
	let hasData = $derived(prompts != undefined && prompts != null);

	const load_data = async (): Promise<FeOllamaPrompt[]> => {
		return await prompts_load();
	};

	load_data().then(d => prompts = d);

</script>

<LoadingSpinner show={!hasData} />

<div class="container">

	<div class="row">
		<div class="col-lg-12">
			<h1>Prompts</h1>
			{#if hasData}
				<table class="table table-striped">
					<thead>
					<tr>
						<th width="120px">
							id
						</th>
						<th>
							prompt
						</th>
						<th>
							created
						</th>
					</tr>
					</thead>
					<tbody>
					{#each prompts as prompt}
						<tr>
							<td>
								{prompt.id} /<a href="/prompt/{prompt.id}">Chat Results</a>

							</td>
							<td>
								{prompt.prompt}
							</td>
							<td>
								{prompt.created}
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