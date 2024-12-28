<script lang="ts">
    import {type PageData} from "./$types";
    import LoadingSpinner from "$lib/components/LoadingSpinner.svelte";
    import {marked} from "marked";

    let {data}: { data: PageData } = $props();
    let chats = $state(data.chats);

    let hasData = $derived(chats != undefined && chats != null);
</script>

<LoadingSpinner show={!hasData }/>

<div class="container-fluid">
    <div class="row">
        <div class="col-lg-12">
            <h1>Ollama Chats</h1>
            {#if hasData}
                <div class="container-fluid">
                    <p>{chats.length} chats</p>
                    {#each chats as chat}
                        <div class="row">
                            <div class="col-12">
                                <div class="card">
                                    <div class="card-header">
                                        {chat.prompt}

                                        <p> duration: {chat.durationMs}ms, numCtx: {chat.numCtx},
                                            seed: {chat.seed},
                                            temperature: {chat.temperature}
                                            , topK: {chat.topK}, topP: {chat.topP}, created: {chat.created}
                                        </p>
                                    </div>
                                    <div class="card-header">
                                        {chat.modelName} / {chat.modelSize}
                                    </div>
                                    <div class="card-body">
                                        <p class="card-text">{@html marked(chat.response, {
                                            breaks: true,
                                            sanitize: true,
                                            smartypants: true,
                                        }) }
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
