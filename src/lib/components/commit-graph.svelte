<script lang="ts">
	import type { Commit } from './commit.svelte';
	import CommitCell from './commit.svelte';
	import { ScrollArea } from '$lib/components/ui/scroll-area';

	export let commits: string[] = [];

	let mappedCommits: Commit[];
	$: mappedCommits = commits.map((message) => {
		return { message, author: 'me', date: new Date(), column: 0 };
	});
</script>

<ScrollArea class="h-32 h-full cursor-default select-none">
	{#each mappedCommits as commit}
		<CommitCell {commit} />
	{/each}
</ScrollArea>
