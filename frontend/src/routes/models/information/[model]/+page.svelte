<script lang="ts">
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();
	let information = $state(data.information);

</script>

<div class="container-fluid">
	<div class="row">
		<div class="col-lg-12">
			<h1>Model Information</h1>
			<table class="table table-striped">
				<tbody>
				<tr>
					<td>modelfile</td>
					<td>{@html information.modelfile.replaceAll("\n", "<br>")}</td>
				</tr>
				<tr>
					<td>details</td>
					<td>
						<pre>{JSON.stringify(information.details, null, 4)}</pre>
					</td>
				</tr>
				<tr>
					<td>parameters</td>
					{#if information.parameters !== undefined && information.parameters !== null}
						<td>{@html information.parameters.replaceAll("\n", "<br>")}</td>
					{/if}
				</tr>
				<tr>
					<td>template</td>
					<td>{@html information.template.replaceAll("\n", "<br>")}</td>
				</tr>
				<tr>
					<td>model_info</td>
					<td>
						<pre>{JSON.stringify(information.modelInfo, null, 4)}</pre>
					</td>
				</tr>
				<tr>
					<td>license</td>
					<td>{@html information.license.replaceAll("\n", "<br>")}</td>
				</tr>
				</tbody>
			</table>
		</div>
	</div>
</div>
