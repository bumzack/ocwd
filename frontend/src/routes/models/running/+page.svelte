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
                            detail_format
                        </th>
                        <th>
                            detail_family
                        </th>
                        <th>
                            detail_parameter_size
                        </th>
                        <th>
                            detail_quantization_level
                        </th>
                        <th>
                            detail_quantization_level
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
                                {model.size}
                            </td>
                            <td>
                                {model.format}
                            </td>
                            <td>
                                {model.family}
                            </td>
                            <td>
                                {model.parameter_size}
                            </td>
                            <td>
                                {model.quantization_level}
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

