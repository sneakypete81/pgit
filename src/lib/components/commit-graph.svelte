<script lang="ts">
	import type { Commit } from './commit.svelte';
	import CommitCell from './commit.svelte';
	import { ScrollArea } from '$lib/components/ui/scroll-area/index.js';

	let commits: Commit[] = [
		{ message: 'feat: styling', author: 'sneakypete81', date: new Date() },
		{ message: 'feat: styling', author: 'sneakypete81', date: new Date() },
		{ message: 'feat: styling', author: 'sneakypete81', date: new Date() },
		{ message: 'feat: styling', author: 'sneakypete81', date: new Date() },
		{ message: 'feat: styling', author: 'sneakypete81', date: new Date() },
		{ message: 'feat: styling', author: 'sneakypete81', date: new Date() },
		{ message: 'feat: styling', author: 'sneakypete81', date: new Date() },
		{ message: 'feat: styling', author: 'sneakypete81', date: new Date() },
		{ message: 'feat: styling', author: 'sneakypete81', date: new Date() },
		{ message: 'feat: styling', author: 'sneakypete81', date: new Date() },
		{ message: 'feat: styling', author: 'sneakypete81', date: new Date() },
		{ message: 'feat: styling', author: 'sneakypete81', date: new Date() },
		{ message: 'feat: styling', author: 'sneakypete81', date: new Date() },
		{ message: 'feat: styling', author: 'sneakypete81', date: new Date() },
		{ message: 'feat: styling', author: 'sneakypete81', date: new Date() },
		{ message: 'feat: styling', author: 'sneakypete81', date: new Date() },
		{ message: 'feat: styling', author: 'sneakypete81', date: new Date() },
		{ message: 'feat: styling', author: 'sneakypete81', date: new Date() },
		{ message: 'feat: styling', author: 'sneakypete81', date: new Date() },
		{ message: 'feat: styling', author: 'sneakypete81', date: new Date() }
	];
</script>

<ScrollArea class="h-32 h-full cursor-default select-none">
	{#each commits as commit}
		<CommitCell {commit} />
	{/each}
</ScrollArea>
