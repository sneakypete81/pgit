<script lang="ts">
	import { ScrollArea } from '$lib/components/ui/scroll-area/index.js';

	import { listen, type Event } from '@tauri-apps/api/event';

	let branches: string[] = [];

	type Payload = {
		branches: string[];
	};

	listen('branches', (event: Event<Payload>) => (branches = event.payload.branches));
</script>

<div class="pl-4 font-bold uppercase">Branches</div>
<ScrollArea class="h-32 h-full cursor-default select-none">
	{#each branches as branch}
		<div class="pl-4 text-foreground-dim hover:bg-background-30">
			{branch}
		</div>
	{/each}
</ScrollArea>
