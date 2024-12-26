<script lang="ts">
    import type {PageData} from './$types';
    import type {FeOllamaRunningModel} from "$lib/models.ts";

    let {data}: { data: PageData } = $props();
    let models: FeOllamaRunningModel[] = $state(data.models);
    let hasData = $derived(models != undefined && models != null);
</script>

<div class="container">
    <div class="row">
        <div class="col-lg-12">
            <h1>Running Ollama models</h1>
            {#if hasData}
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
                            detail_parameter_size
                        </th>
                        <th>
                            quantizationLevel
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
                <p>no data available</p>
            {/if}
        </div>
    </div>
</div>

