<script>
    import Imgcontainer from "./Imgcontainer.svelte";
    import { infosByTag } from '/home/jzhang/media-subscriber/frontend/src/js/fetch.js';
    import { createEventDispatcher } from 'svelte';
    // let img ="https://inv.bp.projectsegfau.lt/vi/4XDCnhDPTFo/maxres.jpg"
    export let videoInfo
    // title ="《陌生人—柴静对话圣战分子》最后一集 《以人之名》"

    const date = new Date(videoInfo.published * 1000); // Convert seconds to milliseconds
    let publish_ = date.toLocaleDateString();

    let dispatch = createEventDispatcher();
</script>

<div class = "video-preview-container" on:click={_ => {dispatch("videoEvent",videoInfo)}}>
    <!-- on:click={e=>{showdatail=true}} -->
    <Imgcontainer img={videoInfo.thumbnails[4].url} length={videoInfo.length}/>
    <div class="video-title-container">{videoInfo.title}</div>
    <p>{publish_}</p>
</div>

<style>
    .video-preview-container{
        /* position: relative; */
        overflow:hidden        
    }
    .video-title-container{
        height: 45px;
        overflow:hidden
    }    
</style>