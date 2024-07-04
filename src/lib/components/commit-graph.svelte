<script lang="ts">
	import { type Commit } from '$lib/bindings';
	import CommitCell from './commit.svelte';
	import { ScrollArea } from '$lib/components/ui/scroll-area';

	export let commits: Commit[] = [];
</script>

<ScrollArea class="h-32 h-full cursor-default select-none">
	{#each commits as commit}
		<CommitCell {commit} />
	{/each}
</ScrollArea>
