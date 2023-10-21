<script>
	import tabler from 'yesvelte/css/tabler.min.css?url'
	import { Accordions, Accordion, AccordionBody } from 'yesvelte'
	import { onMount } from 'svelte';
	import { Button, Icon } from 'yesvelte'
	import Videopreview from './pages/Videopreview.svelte';
    import Videospool from './pages/Videospool.svelte';
	import { videosByTag } from './js/fetch';

	let data =[];
</script>
<svelte:head>
	<link rel='stylesheet' href={tabler}/>
</svelte:head>


<Button color="light" on:click={e => {videosByTag("test", res => data =res)}}> 
	<Icon name="tags" size="2x"></Icon>
</Button>
<Videospool pool={data}/>


<style>
	
</style>