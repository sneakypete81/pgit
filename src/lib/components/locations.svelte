<script lang="ts">
	import { ScrollArea } from '$lib/components/ui/scroll-area/index.js';

	import { listen, type Event } from '@tauri-apps/api/event';

	let branches: string[] = [];

	type Payload = {
		branches: string[];
	};

	listen('branches', (event: Event<Payload>) => (branches = event.payload.branches));
</script>

<div class="font-bold uppercase">Branches</div>
<ScrollArea class="h-32 h-full">
	{#each branches as branch}
		<div class="overflow-clip text-foreground-dim">
			{branch}
		</div>
	{/each}
</ScrollArea>
