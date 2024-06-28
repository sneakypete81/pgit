<script lang="ts">
	import CommitGraph from '$lib/components/commit-graph.svelte';
	import Locations from '$lib/components/locations.svelte';
	import * as Resizable from '$lib/components/ui/resizable';

	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';

	onMount(() => invoke('init'));
</script>

<Resizable.PaneGroup direction="horizontal" class="text-nowrap">
	<Resizable.Pane class="bg-background-10">
		<Locations />
	</Resizable.Pane>
	<Resizable.Handle class="bg-background-20" />
	<Resizable.Pane class="bg-background-20">
		<CommitGraph />
	</Resizable.Pane>
	<Resizable.Handle class="bg-background-30" />
	<Resizable.Pane defaultSize={50} class="bg-background-30">Three</Resizable.Pane>
</Resizable.PaneGroup>
