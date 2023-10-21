<script>
    export let img,length
</script>

<div class="video-preview-img-container">
    <img src={img} alt="img">
    <div class="bottom-right-time-container" >{length}</div>
</div>

<style>
    .video-preview-img-container{
        /*without position relation, time-container will drop to next father container with position relative. 
        May because of relative set the range of container, let son container know where is the boards*/
        position: relative;
        aspect-ratio: 16 / 9;
        background-color: rgb(0, 0, 0);
        overflow:hidden;
        display: flex;
        justify-content: center;
    }
    .bottom-right-time-container{
        position: absolute;
        background-color: rgb(0, 0, 0);
        color: aliceblue;
        font-size: 20px;
        font-weight:bold;
        opacity: 0.8;
        bottom: 5px;
        right: 6px;
    }
</style>
