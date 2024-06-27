<script lang="ts">
	import * as Resizable from '$lib/components/ui/resizable';
	import { ScrollArea } from '$lib/components/ui/scroll-area/index.js';

	import { listen, type Event } from '@tauri-apps/api/event';
	import { invoke } from '@tauri-apps/api/tauri';

	let branches: string[] = [];

	type Payload = {
		branches: string[];
	};

	listen('branches', (event: Event<Payload>) => (branches = event.payload.branches));

	invoke('init');
</script>

<Resizable.PaneGroup direction="horizontal" class="border">
	<Resizable.Pane defaultSize={25} class="bg-background-10 pl-2">
		<div class="font-bold uppercase">Branches</div>
		<ScrollArea class="h-32 h-full">
			{#each branches as branch}
				<div class="overflow-clip text-foreground-dim">
					{branch}
				</div>
			{/each}
		</ScrollArea>
	</Resizable.Pane>
	<Resizable.Handle class="bg-background-20" />
	<Resizable.Pane class="bg-background-20">Two</Resizable.Pane>
	<Resizable.Handle class="bg-background-30" />
	<Resizable.Pane class="bg-background-30">Three</Resizable.Pane>
</Resizable.PaneGroup>
