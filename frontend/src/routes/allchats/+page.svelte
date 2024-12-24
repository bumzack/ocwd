<script lang="ts">

    import {type FeOllamaChat} from '$lib/models.ts';
    import {chats_load_all} from '$lib/apiService.ts';
    import LoadingSpinner from '$lib/components/LoadingSpinner.svelte';
    import {marked} from "marked";

    let chats = $state([]);
    let hasData = $derived(chats != undefined && chats != null);

    const load_data = async (): Promise<FeOllamaChat[]> => {
        return await chats_load_all();
    };

    load_data().then(d => {
        chats = d;
    });

</script>

<LoadingSpinner show={!hasData }/>

<div class="container">
    <div class="row">

        <div class="col-lg-12">
            <h1>Ollama Chats</h1>

            {#if hasData}
                <div class="container-fluid">
                    {#each chats as chat}

                        <div class="row">
                            <div class="col-12">
                                <div class="card">
                                    <div class="card-header">
                                        {chat.prompt}
                                    </div>
                                    <div class="card-header">
                                        {chat.modelName} / {chat.modelSize}
                                    </div>
                                    <div class="card-body">
                                        <!--								<h5 class="card-title">Special title treatment</h5>-->
                                        <p class="card-text">{@html marked(chat.response, {
                                            breaks: true,
                                            sanitize: true,
                                            smartypants: true,
                                        }) }
                                    </div>
                                    <div class="card-footer text-body-secondary">
                                        duration: {chat.durationMs}ms, numCtx: {chat.numCtx},
                                        seed: {chat.seed},
                                        temperature: {chat.temperature}
                                        , topK: {chat.topK}, topP: {chat.topP}, created: {chat.created}
                                    </div>
                                </div>
                                <hr/>
                            </div>
                        </div>
                    {/each}
                </div>
            {:else}
                <p>no data available</p>
            {/if}
        </div>
    </div>
</div>


<style>
    @import 'bootstrap/dist/css/bootstrap.min.css';
</style>