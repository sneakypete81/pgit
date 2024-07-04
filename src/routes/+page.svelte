<script lang="ts">
	import { commands, events, type Status } from '$lib/bindings';
	import CommitGraph from '$lib/components/commit-graph.svelte';
	import Locations from '$lib/components/locations.svelte';
	import * as Resizable from '$lib/components/ui/resizable';
	import { onMount } from 'svelte';

	onMount(commands.init);

	let payload: Status = { branches: [], commits: [] };
	events.status.listen((e) => (payload = e.payload));
</script>

<Resizable.PaneGroup direction="horizontal" class="text-nowrap">
	<Resizable.Pane class="bg-background-10">
		<Locations branches={payload.branches} />
	</Resizable.Pane>
	<Resizable.Handle class="bg-background-20" />
	<Resizable.Pane class="bg-background-20">
		<CommitGraph commits={payload.commits} />
	</Resizable.Pane>
	<Resizable.Handle class="bg-background-30" />
	<Resizable.Pane defaultSize={50} class="bg-background-30">Three</Resizable.Pane>
</Resizable.PaneGroup>
