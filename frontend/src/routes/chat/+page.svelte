<script lang="ts">

    import type {PageData} from "./$types";

    let {data}: { data: PageData } = $props();
    let models = $state(data.models);

    console.log(`all mnodels ${JSON.stringify(models, null, 4)}`)
    // let hasData = $derived(models != undefined && models != null);
    // let prompt = $state('');
    // let numCtx = $state(4096);
    // let seed = $state(23);
    // let temperature = $state(0.1);
    // let topK = $state(20.0);
    // let topP = $state(0.5);
    //
    // let running = $state(false);
    // let allSelected = $state(false);
    //
    // let btnText = $derived(allSelected ? 'unselect all' : 'select all');
    // let responses = $state([]);
    //
    // const promptEmpty = () => {
    //     return prompt !== null && prompt !== undefined && prompt.length > 0;
    // };
    //
    // const modelSelected = () => {
    //     return models !== undefined && models.size > 0;
    // };

   //  let disabled = $derived(!promptEmpty() && !modelSelected());

    // const start_magic = async (): Promise<void> => {
    //     responses = [];
    //     // console.log(`all mnodels ${JSON.stringify(models, null, 4)}`)
    //     const selected_models = models.filter(m => m.checked);
    //     // console.log(`selected_models ${JSON.stringify(selected_models, null, 4)}`)
    //
    //     const modelReq = selected_models.map(model => {
    //         return {
    //             modelId: model.id,
    //             temperature: temperature,
    //             numCtx: numCtx,
    //             seed: seed,
    //             topK: topK,
    //             topP: topP
    //         };
    //     });
    //
    //     const req = {
    //         prompt: prompt,
    //         models: modelReq
    //     };
    //
    //     running = true;
    //
    //     const res = await enqueue_models(req);
    //     responses = res;
    //     console.log(`res  ` + JSON.stringify(res, null, 4));
    //
    //     running = false;
    // };
    //
    // const select_all_models = (): void => {
    //     if (allSelected) {
    //         models.forEach(m => m.checked = false);
    //     } else {
    //         models.forEach(m => m.checked = true);
    //     }
    //     allSelected = !allSelected;
    // };

</script>

<h2>bla</h2>

<p>{running}</p>
<p>{responses}</p>

<p>{disabled}</p>
<p>{btnText}</p>

<p>{allSelected}</p>
<p>{hasData}</p>

<p>{hasData}</p>
<p>{hasData}</p>
<p>{hasData}</p>


<!--<LoadingSpinner show={!hasData || running}/>-->

<!--<div class="container">-->
<!--    <div class="row">-->
<!--        <div class="col-lg-6">-->
<!--            <h1>Ollama Chat</h1>-->

<!--            <form class="row g-3">-->
<!--                <div class="col-md-12">-->
<!--                    <div class="form-check">-->
<!--                        <label class="form-label" for="exampleFormControlTextarea1">Prompt:</label>-->
<!--                        <textarea bind:value={prompt} class="form-control" id="exampleFormControlTextarea1"-->
<!--                                  rows="3"></textarea>-->
<!--                    </div>-->
<!--                </div>-->

<!--                <div class="col-12">-->
<!--                    <button class="btn btn-primary" {disabled} onclick={start_magic}>Let the magic do its work!</button>-->
<!--                </div>-->
<!--            </form>-->

<!--            <div class="row">-->
<!--                <div class="col-12">-->
<!--                    <h2>enqueued chats</h2>-->
<!--                    {#each responses as aresponse}-->
<!--                        <p>model id: {aresponse.modelId}, prompt id: {aresponse.promptId}, state {aresponse.state},-->
<!--                            created {aresponse.created}</p>-->
<!--                    {/each}-->
<!--                </div>-->
<!--            </div>-->
<!--        </div>-->

<!--        <div class="col-lg-6">-->
<!--            <h4>Available Models</h4>-->
<!--            {#if hasData}-->
<!--                <form>-->
<!--                    <button onclick={select_all_models} class="btn btn-primary">{btnText}</button>-->
<!--                </form>-->

<!--                {#each models as model}-->
<!--                    <div class="card">-->
<!--                        <div class="card-header">-->
<!--                            <label class=" d-flex gap-2">-->
<!--                                <input class="form-check-input flex-shrink-0" type="checkbox"-->
<!--                                       bind:checked={model.checked}>-->
<!--                                <span>-->
<!--								<small class="d-block text-body-secondary">{model.name}-->
<!--                                    / {model.detailParameterSize}</small>-->
<!--							</span>-->
<!--                            </label>-->
<!--                        </div>-->
<!--                        <div class="card-body">-->
<!--                            <form class="row g-3">-->
<!--                                <div class="col-md-3">-->
<!--                                    <label for="inputNumCtx" class="form-label">NumCtx</label>-->
<!--                                    <input bind:value={numCtx} type="number" class="form-control" id="inputNumCtx">-->
<!--                                </div>-->
<!--                                <div class="col-md-2">-->
<!--                                    <label for="inputSeed" class="form-label">Seed</label>-->
<!--                                    <input bind:value={seed} type="number" class="form-control" id="inputSeed">-->
<!--                                </div>-->
<!--                                <div class="col-md-3">-->
<!--                                    <label for="inputTemperature" class="form-label">Temperature</label>-->
<!--                                    <input bind:value={temperature} type="number" class="form-control"-->
<!--                                           id="inputTemperature">-->
<!--                                </div>-->
<!--                                <div class="col-md-2">-->
<!--                                    <label for="inputTopK" class="form-label">top-k</label>-->
<!--                                    <input bind:value={topK} type="number" class="form-control" id="inputTopK">-->
<!--                                </div>-->
<!--                                <div class="col-md-2">-->
<!--                                    <label for="inputTopP" class="form-label">top-k</label>-->
<!--                                    <input bind:value={topP} type="number" class="form-control" id="inputTopP">-->
<!--                                </div>-->
<!--                            </form>-->
<!--                        </div>-->
<!--                    </div>-->
<!--                    <br/>-->

<!--                {/each}-->

<!--            {:else}-->
<!--                <p>no data available</p>-->
<!--            {/if}-->
<!--        </div>-->


<!--    </div>-->
<!--</div>-->
