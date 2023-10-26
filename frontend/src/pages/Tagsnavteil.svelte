<script>
    import { Button, Icon } from 'yesvelte'
    import { infosByTag } from '/home/jzhang/media-subscriber/frontend/src/js/fetch.js';
    import { createEventDispatcher } from 'svelte';
    let dispatch = createEventDispatcher();
</script>
<div class = "searchbar-container">
    <input type="text" placeholder="Search.." name="search">
</div>
<div class="tags-container">
    <div class="tag-container">
        <Button on:click={e => {infosByTag("test", res => {dispatch("tagEvent",res)})}}>Test</Button>
        <Button>Sport</Button>
        <Button>Music</Button>
        <Button>News</Button>
        <Button>Moive</Button>
    </div>
</div>

<style>
    .searchbar-container{
        margin:10px;
        height: 30px;
    }
    .tags-container{
        margin:5px;
        display: flex;
    }
    .tag-container{
        margin:2px;
        min-width:200px;
        overflow:hidden
    }
</style>