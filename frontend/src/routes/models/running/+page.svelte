<script lang="ts">
    import type {PageData} from './$types';
    import type {FeOllamaRunningModel} from "$lib/models.ts";

    let {data}: { data: PageData } = $props();
    let models: FeOllamaRunningModel[] = $state(data.models);
    let hasData = $derived(models != undefined && models != null);
</script>

<div class="container-fluid">
    <div class="row">
        <div class="col-lg-12">
            <h1>Locally Running Ollama Models</h1>
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
                                {model.model}
                            </td>
                            <td>
                                {model.size.toLocaleString('de-DE', {minimumFractionDigits: 0})}
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
                <p>no locally running models found</p>
            {/if}
        </div>
    </div>
</div>

