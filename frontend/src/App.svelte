<script>
	import tabler from 'yesvelte/css/tabler.min.css?url'
	import { Accordions, Accordion, AccordionBody } from 'yesvelte'
	import { onMount } from 'svelte';
	import { Button, Icon } from 'yesvelte'
	import Videopreview from './pages/Videopreview.svelte';
    import Videospool from './pages/Videospool.svelte';
	import { infosByTag } from './js/fetch';
	import { Badge } from "yesvelte";
	import Avatarlist from './pages/Avatarlist.svelte';
	import Tagsnavteil from './pages/Tagsnavteil.svelte';

	let data =[];
</script>
<svelte:head>
	<link rel='stylesheet' href={tabler}/>
</svelte:head>

<div class="left-side-container">
	<div class ="header-container"><h1>Media-subscrbiber</h1></div>
	<Tagsnavteil on:tagEvent={e => data = e.detail}/>
</div>
<div class="right-side-container">
	<Avatarlist data={data}/>
	<Videospool pool={data}/>
</div>

<!-- <Button color="light" on:click={e => {infosByTag("test", res => data =res)}}> 
	<Icon name="tags" size="2x"></Icon>
</Button> -->
<!-- <Avatarlist data ={data}/> -->


<style>
	.left-side-container{
		position:absolute;
    	/* z-index:-1;  may used for background setting*/
		display: flex;
		flex-direction:  column ;
		height:100%;
		width: 250px;
		border-right-style:solid;
		border-right-width: 6px;
		border-image: linear-gradient(to right, #b9b9b9, #ffffff) 1 100%;
	}
	.header-container{
		margin:5px
	}
	.right-side-container{
		position:absolute;
		top:0;
		left:253px;
		right:0;
		bottom:0;
		display: flex;
		flex-direction:  column ;
	}
</style>