<script>
    import Imgcontainer from "./Imgcontainer.svelte";
    import { createEventDispatcher } from 'svelte';
    import { Accordions, Accordion, AccordionBody } from 'yesvelte'
    import { Button, Icon } from 'yesvelte'

    export let videoInfo

    const date = new Date(videoInfo.published * 1000); // Convert seconds to milliseconds
    let publish_ = date.toLocaleDateString();

    let dispatch = createEventDispatcher();
</script>

<div class = "videodetail-container" >
    <Imgcontainer img={videoInfo.thumbnails[4].url} length={videoInfo.length}/>
    <h1>{videoInfo.title}</h1>
    <p>{publish_}</p>
    <div class = "videodetail-download-container">
        <Button><Icon name="download"/>Video</Button>
        <Button><Icon name="download" />Audio</Button>
    </div>
    <div class ="close-button-container">
        <Button on:click={_ =>{dispatch("closeEvent",true)}}><Icon name="x"/></Button>
    </div>

</div>


<style>
    .videodetail-container{
        position: relative;
        margin:10px;
        overflow:hidden        
    }
    .videodetail-download-container{
        height: 50px;
        display: flex;
        flex-direction: row-reverse;
    }
    .close-button-container{
        position: absolute;
        right: 0;
        top: 0;
    }
</style>