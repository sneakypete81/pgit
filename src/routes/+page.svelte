<script lang="ts">
	import Locations from '$lib/components/locations.svelte';
	import * as Resizable from '$lib/components/ui/resizable';

	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';

	onMount(() => invoke('init'));
</script>

<Resizable.PaneGroup direction="horizontal" class="border">
	<Resizable.Pane defaultSize={25} class="bg-background-10 pl-2">
		<Locations />
	</Resizable.Pane>
	<Resizable.Handle class="bg-background-20" />
	<Resizable.Pane class="bg-background-20">Two</Resizable.Pane>
	<Resizable.Handle class="bg-background-30" />
	<Resizable.Pane class="bg-background-30">Three</Resizable.Pane>
</Resizable.PaneGroup>
