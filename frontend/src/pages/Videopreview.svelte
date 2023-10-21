<script>
    import Imgcontainer from "./Imgcontainer.svelte";
    // let img ="https://inv.bp.projectsegfau.lt/vi/4XDCnhDPTFo/maxres.jpg"
    export let length,publish,title,img
    // title ="《陌生人—柴静对话圣战分子》最后一集 《以人之名》"

    const date = new Date(publish * 1000); // Convert seconds to milliseconds
    let publish_ = date.toLocaleDateString();
</script>

<div class = "video-preview-container" >
    <Imgcontainer img={img} length={length}/>
    <div class="video-title-container">{title}</div>
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