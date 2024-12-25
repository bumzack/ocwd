<script lang="ts">

    import type {PageData} from "./$types";

    let {data}: { data: PageData } = $props();
    let prompts = $state(data.prompts);
    let hasData = $derived(prompts != undefined && prompts != null);

</script>

<!--<LoadingSpinner show={!hasData}/>-->

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
