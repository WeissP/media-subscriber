<script>
    import Videopreview from "./Videopreview.svelte";
    export let pool
    import { onMount } from 'svelte';
    // let length = pool[0].videos_info.length
    // let publish =pool[0].videos_info.published
    // let title = pool[0].videos_info.title
    // let img = pool[0].videos_info.thumbnails[0].url
    $:onMount(e =>console.log("pool:" + pool));
</script>

<div class="videopool-container">
    {#if pool && pool.videos && Array.isArray(pool.videos)}
        {#each pool.videos as video,index}
        <div class="videopreview-container">
            <Videopreview length={video.video_info.length} publish={video.video_info.published} title={video.video_info.title} img={video.video_info.thumbnails[4].url}/>
        </div>	
        {/each}
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
</style>