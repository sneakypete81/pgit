<script lang="ts">
	import { ScrollArea } from '$lib/components/ui/scroll-area';

	export let branches: string[] = [];
</script>

<div class="pl-4 font-bold uppercase">Branches</div>
<ScrollArea class="h-32 h-full cursor-default select-none">
	{#each branches as branch}
		<div class="pl-4 text-foreground-dim hover:bg-background-30">
			{branch}
		</div>
	{/each}
</ScrollArea>
