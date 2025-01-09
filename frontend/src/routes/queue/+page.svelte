<script lang="ts">
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();
	let queues = $state(data.queue);
	let hasData = $derived(queues != undefined && queues != null);

</script>

<div class="container-fluid">
	<div class="row">
		<div class="col-lg-12">
			<h1>Ollama chat queue</h1>
			{#if hasData}
				<p>{queues.length} entries in queue</p>
				<table class="table table-striped">
					<thead>
					<tr>
						<td>
							id
						</td>
						<td>
							modelId
						</td>
						<td>
							promptId
						</td>
						<td>
							state
						</td>
						<td>
							temperature
						</td>
						<td>
							seed
						</td>
						<td>
							numCtx
						</td>
						<td>
							topK
						</td>
						<td>
							topP
						</td>
						<td>
							created
						</td>
						<td>
							updated
						</td>
					</tr>
					</thead>
					<tbody>
					{#each queues as entry}
						<tr>
							<td>
								{entry.id}
							</td>
							<td>
								{entry.modelId}
							</td>
							<td>
								{entry.promptId}
							</td>
							<td>
								{entry.state}
							</td>
							<td>
								{entry.temperature}
							</td>
							<td>
								{entry.seed}
							</td>
							<td>
								{entry.numCtx}
							</td>
							<td>
								{entry.topK}
							</td>
							<td>
								{entry.topP}
							</td>
							<td>
								{entry.created.toLocaleString()}
							</td>
							<td>
								{entry.updated.toLocaleString()}
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

