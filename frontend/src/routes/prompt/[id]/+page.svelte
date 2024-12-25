<script lang="ts">
    import {marked} from "marked";
    import type {PageData} from './$types';

    // let {chats, prompt} = $props();
    let {data}: { data: PageData } = $props();
    let chats = $state(data.chats);
    let prompt = $state(data.prompt);

    //	let  {chats, prompt}: { data: PageData } = $props();

    console.log(`chats ${JSON.stringify(chats, null, 4)}`);
    console.log(`prompt ${JSON.stringify(prompt, null, 4)}`);

    let hasData = $derived(chats != undefined && chats.length > 0);
</script>

<div class="container">
    <div class="row">
        <div class="col-12">
            {#if prompt !== undefined}
                <p>{prompt.prompt}</p>
                <p>id: {prompt.id} / {prompt.created}</p>
            {:else}
                <p>no prompt found</p>
            {/if}
        </div>
    </div>
    <div class="row">
        <div class="col-lg-2">
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

        <div class="col-lg-10">
            {#if prompt !== undefined}
                {#if hasData}
                    <div class="container-fluid">
                        {#each chats as chat}

                            <div class="row">
                                <div class="col-12">
                                    <div class="card">
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
                                            duration: {chat.durationMs}ms, numCtx: {chat.numCtx}, seed: {chat.seed},
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
            {:else}
                <p>no prompt found</p>
            {/if}
        </div>
    </div>
</div>

