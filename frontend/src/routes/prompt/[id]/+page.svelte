<script lang="ts">
    import {marked} from "marked";
    import type {PageData} from './$types';

    let {data}: { data: PageData } = $props();
    let chats = $state(data.chats);
    let prompt = $state(data.prompt);
    let hasData = $derived(chats != undefined && chats.length > 0);
</script>

<div class="container-fluid">
    <div class="row">
        <div class="col-12">
            {#if prompt !== undefined}
                <div class="card">
                    <div class="card-header">
                        Prompt
                    </div>
                    <div class="card-body">
                        <p>{prompt.prompt}</p>
                    </div>
                    <div class="card-footer text-body-secondary">
                        id: {prompt.id} / {prompt.created}
                    </div>
                </div>
            {:else}
                <p>no prompt found</p>
            {/if}
        </div>
    </div>
    <br/>
    <div class="row">
        <div class="col-lg-3">
            <p>{chats.length} chats</p>

            {#if hasData}
                <ul>
                    {#each chats as chat}
                        <li>{chat.modelName} / {chat.modelSize} </li>
                    {/each}
                </ul>
            {:else}
                <p>no data available</p>
            {/if}
        </div>

        <div class="col-lg-9">
            {#if prompt !== undefined}
                {#if hasData}
                    <div class="container-fluid">
                        {#each chats as chat}
                            <div class="row">
                                <div class="col-12">
                                    <div class="card">
                                        <div class="card-header">
                                            <h4>{chat.modelName} / {chat.modelSize}</h4>
                                            <br/>
                                            <p>
                                                duration: {chat.durationMs}ms, numCtx: {chat.numCtx}, seed: {chat.seed},
                                                temperature: {chat.temperature}, topK: {chat.topK}, topP: {chat.topP},
                                                created: {chat.created}
                                            </p>
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
            {:else}
                <p>no prompt found</p>
            {/if}
        </div>
    </div>
</div>
