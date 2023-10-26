<script>
    import { Card, CardBody, CardTitle, Ribbon, El, Icon } from 'yesvelte'
    export let img,length
    let mark = 1
    const time = new Date(length*1000)
    
    function secondsToHms(d) {
        d = Number(d);
        var h = Math.floor(d / 3600);
        var m = Math.floor(d % 3600 / 60);
        var s = Math.floor(d % 3600 % 60);

        var hDisplay = h > 0 ? h + ":" : "";
        var mDisplay = m > 0 ? m + ":" : "";
        var sDisplay = s > 0 ? s  : "";
        return hDisplay + mDisplay + sDisplay; 
    }
</script>

<div class="video-preview-img-container">
        <img src={img} alt="img">
        {#if mark == 1}
            
        {/if}
        <div class="bottom-right-time-container" >{secondsToHms(length)}</div>
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
        border-radius: 3px;
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
