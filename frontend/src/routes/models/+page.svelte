<script lang="ts">
    import {models_import} from '$lib/apiService.ts';
    import LoadingSpinner from '$lib/components/LoadingSpinner.svelte';
    import type {PageData} from './$types';
    import type {FeOllamaModel, InsertModelsResponse, OllamaModel} from "$lib/models.ts";

    let {data}: { data: PageData } = $props();

    let models: FeOllamaModel[] = $state(data.models);

    console.log(`models from page.server.ts ${JSON.stringify(models)}`)

    let hasData = $derived(models != undefined && models != null);
    let running = $state(false);

    let importedModels: InsertModelsResponse[] = $state([]);

    const importModels = async (): Promise<void> => {
        running = true;
        importedModels = await models_import();
        running = false;
    };

    let hasDataImportedModels = $derived(importedModels !== undefined && importedModels.length > 0);

</script>

<LoadingSpinner show={running}/>

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

    {#if hasDataImportedModels}
        <div class="row">
            <div class="col-lg-12">
                <h1>imported Ollama models local</h1>

                <table class="table table-striped">
                    <thead>
                    <tr>
                        <th>
                            modelId
                        </th>
                        <th>
                            name
                        </th>
                        <th>
                            model
                        </th>
                        <th>
                            result
                        </th>
                    </tr>
                    </thead>
                    <tbody>
                    {#each importedModels as model}
                        <tr>
                            <td>
                                {model.modelId}
                            </td>
                            <td>
                                {model.name}
                            </td>
                            <td>
                                {model.model}
                            </td>
                            <td>
                                {model.result}
                            </td>
                        </tr>
                    {/each}
                    </tbody>
                </table>
            </div>
        </div>
    {/if}
</div>

