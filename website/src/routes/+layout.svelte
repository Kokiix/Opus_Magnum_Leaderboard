<script>
	import favicon from "$lib/assets/favicon.svg";

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
	<title>Opus Magnumoki Leaderboard</title>
</svelte:head>

{@render children()}
