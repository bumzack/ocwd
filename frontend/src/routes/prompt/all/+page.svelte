<script lang="ts">

	import type { PageData } from './$types';
	import LoadingSpinner from '$lib/components/LoadingSpinner.svelte';

	let { data }: { data: PageData } = $props();
	let prompts = $state(data.prompts);
	let hasData = $derived(prompts != undefined && prompts != null && prompts.length > 0);

</script>

<!--<LoadingSpinner show={!hasData} />-->

<div class="container-fluid">
	<div class="row">
		<div class="col-lg-12">
			<h1>Prompts</h1>
			{#if hasData}
				<p>{prompts.length} prompts</p>
				<table class="table table-striped table-bordered table-sm">
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
								<a href="/prompt/{prompt.id}">{prompt.id} / results</a>
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
