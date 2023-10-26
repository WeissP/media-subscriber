<script>
    import Videopreview from "./Videopreview.svelte";
    import Videodetail from "./Videodetail.svelte";
    export let pool
    let videodetail
    let showdetail=false
    import { onMount } from 'svelte';
    // let length = pool[0].videos_info.length
    // let publish =pool[0].videos_info.published
    // let title = pool[0].videos_info.title
    // let img = pool[0].videos_info.thumbnails[0].url
    // $:onMount(e =>console.log("pool:" + pool));
</script>

<div class="videopool-container" >
    {#if pool.videos && pool.videos && Array.isArray(pool.videos)}
        {#each pool.videos as video,index}
        <div class="videopreview-container">
            <Videopreview videoInfo={video.video_info} on:videoEvent={e => {videodetail = e.detail;showdetail=true}}/>
            <!-- length={video.video_info.length} publish={video.video_info.published} title={video.video_info.title} img={video.video_info.thumbnails[4].url -->
        </div>	
        {/each}
    {/if}
    {#if showdetail==true}
        <div class = "videodetail-container">
            <Videodetail videoInfo ={videodetail} on:closeEvent={e => showdetail=!e.detail}/> 
        </div>    
    {/if}
</div>
    
<style>
    .videopreview-container{
        width:250px;
        height:200px;
        margin:10px;

    }
    .videopool-container{
        display: flex;
        flex-wrap: wrap;
    }
    .videodetail-container{
        position: fixed;
        top:100px;
        left:300px;
        background-color: white;
    }
    
</style>